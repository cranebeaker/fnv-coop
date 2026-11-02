//! `(unplaced)/run_006ecd60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `006ed010` (`this` only, plain `RET`).
const BASE_CONSTRUCTOR_006ED010: u32 = 0x006e_d010;
/// The element-array constructor helper `00ec782f` (cdecl-looking, `RET 0x14`: array, element
/// size, count, element constructor, element destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The element constructor (`006ed0e0`) and destructor (`00483710`) handed to it.
const ELEMENT_CONSTRUCTOR_006ED0E0: u32 = 0x006e_d0e0;
const ELEMENT_DESTRUCTOR_00483710: u32 = 0x0048_3710;
/// The follow-up call `006ed040` (`this` plus two stack words, `RET 8`).
const FOLLOW_UP_006ED040: u32 = 0x006e_d040;
/// The vtable stored at `+0` (RTTI class `BSTStaticFreeList<ActorPathingMessage,256>`).
const VTABLE_006ECE90: u32 = 0x0106_d798;

// Translated from 006ece90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ece90 - the compiler's exception frame (FS:[0] chain and the
// unwinding that would destroy the element array) is not translated.
/// Unnamed in the engine map: runs `006ed010` on `this`, stores the vtable `0106d798` (RTTI
/// class `BSTStaticFreeList<ActorPathingMessage,256>`), constructs 0x100 elements of 0xc
/// bytes at `this + 0xc` through `00ec782f` (constructor `006ed0e0`, destructor `00483710`),
/// then calls `006ed040(this, 0x100, this + 0xc)`. Returns `this`.
pub fn fn_006ece90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_006ED010, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_006ECE90);
    let elements = this.addr() + 0xc;
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            elements,
            0xcu32,
            0x100u32,
            ELEMENT_CONSTRUCTOR_006ED0E0,
            ELEMENT_DESTRUCTOR_00483710
        ],
    );
    e.call(FOLLOW_UP_006ED040, &args![this, 0x100u32, elements]);
    this
}

/// The vtable of `BSTFreeList<ActorPathingMessage>` (RTTI
/// `.?AV?$BSTFreeList@VActorPathingMessage@@@@`).
const FREE_LIST_VTABLE: u32 = 0x0106_d7a0;
/// The free list's lock word (+4, taken and released through `0043b460`)
/// and the head of its free elements (+8).
const FREE_LIST_LOCK: u32 = 0x4;
const FREE_LIST_HEAD: u32 = 0x8;
/// An element (0xc bytes): 8 bytes of message, then the next free element
/// at +8.
const ELEMENT_SIZE: u32 = 0xc;
const ELEMENT_NEXT: u32 = 0x8;
/// The number of elements of the static free list.
const STATIC_ELEMENT_COUNT: u32 = 0x100;
/// `0043b460` (`cdecl`, three words: the word, then two values): an
/// `InterlockedCompareExchange` (`[00fdf1bc]`) wrapper; returns the old
/// value. Called with (lock, 0, 1) until it returns 0 to take the lock and
/// with (lock, 1, 0) to release it.
const COMPARE_EXCHANGE: u32 = 0x0043_b460;
/// `0040fbe0` (engine map `Error`, `cdecl`, no words): an empty function the
/// code calls right after taking and right before releasing the lock.
const EMPTY_HOOK: u32 = 0x0040_fbe0;
/// `006e6da0` (`cdecl`, two words): returns its second word (the placement
/// `operator new(size, where)` shape).
const PLACEMENT_NEW: u32 = 0x006e_6da0;
/// `006eaa00` (`thiscall`, `RET 4`, `pathmanager.cpp`): copies the two
/// words at its argument into `this`; returns `this`.
const COPY_TWO_WORDS: u32 = 0x006e_aa00;
/// `` `eh vector destructor iterator' `` (`00ec5fce`, `RET 0x10`: array,
/// element size, count, element destructor).
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;

/// Spins on the lock word until `0043b460(lock, 0, 1)` returns 0, then calls
/// the empty hook.
fn lock(e: &mut Engine, this: u32) {
    let lock = this.wrapping_add(FREE_LIST_LOCK);
    // The exe's `while (true)` loop: it retries for as long as the old
    // value is non-zero.
    while e.call(COMPARE_EXCHANGE, &args![lock, 0u32, 1u32]).u32() != 0 {}
    e.call(EMPTY_HOOK, &args![]);
}

/// Calls the empty hook, then releases the lock (`0043b460(lock, 1, 0)`).
fn unlock(e: &mut Engine, this: u32) {
    e.call(EMPTY_HOOK, &args![]);
    e.call(
        COMPARE_EXCHANGE,
        &args![this.wrapping_add(FREE_LIST_LOCK), 1u32, 0u32],
    );
}

// Translated from 006ecd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`; unoptimized code): takes an element off
/// the free list under its lock. With a free element it writes it at
/// `out`, makes its next element the new head, clears its next pointer and
/// returns true; with none it writes 0 at `out` and returns false.
pub fn fn_006ecd60(e: &mut Engine, this: Ptr, out: Ptr) -> bool {
    let base = this.addr();
    lock(e, base);
    let taken = if e.mem.u32(base.wrapping_add(FREE_LIST_HEAD)) != 0 {
        let head = e.mem.u32(base.wrapping_add(FREE_LIST_HEAD));
        e.mem.set_u32(out.addr(), head);
        let element = e.mem.u32(out.addr());
        let next = e.mem.u32(element.wrapping_add(ELEMENT_NEXT));
        e.mem.set_u32(base.wrapping_add(FREE_LIST_HEAD), next);
        let element = e.mem.u32(out.addr());
        e.mem.set_u32(element.wrapping_add(ELEMENT_NEXT), 0);
        true
    } else {
        e.mem.set_u32(out.addr(), 0);
        false
    };
    unlock(e, base);
    taken
}

// Translated from 006ece00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ece00 - the compiler's exception frame (FS:[0] chain, handler 00f066b1) is not translated
/// Unnamed (`thiscall`, `RET 4`; unoptimized code): re-makes the 8-byte
/// message at `this` as a copy of `source`: its (empty) destructor
/// `00483710`, then a placement new (`006e6da0(8, this)`) and, when that is
/// non-null, the copy `006eaa00(this, source)`. Returns the copy's result
/// (0 when the placement new gave null).
pub fn fn_006ece00(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(ELEMENT_DESTRUCTOR_00483710, &args![this]);
    let place = e.call(PLACEMENT_NEW, &args![8u32, this]).u32();
    if place != 0 {
        e.call(COPY_TWO_WORDS, &args![place, source]).ptr()
    } else {
        Ptr::new(0)
    }
}

// Translated from 006ecf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTFreeList<ActorPathingMessage>::~BSTFreeList` (by the vtable it
/// stores; unnamed in the engine map): stores the vtable.
pub fn fn_006ecf20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FREE_LIST_VTABLE);
}

// Translated from 006ecfa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ecfa0 - the compiler's exception frame (FS:[0] chain, handler 00f06718) is not translated
/// `BSTStaticFreeList<ActorPathingMessage, 256>::~BSTStaticFreeList` (by
/// the vtable it stores; unnamed in the engine map): stores the vtable
/// `0106d798`, destroys the 0x100 elements at `+0xc` (`` `eh vector
/// destructor iterator' `` with `00483710`), then the base destructor
/// `006ecf20`.
pub fn fn_006ecfa0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_006ECE90);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![
            this.addr().wrapping_add(0xc),
            ELEMENT_SIZE,
            STATIC_ELEMENT_COUNT,
            ELEMENT_DESTRUCTOR_00483710
        ],
    );
    fn_006ecf20(e, this);
}

// Translated from 006ed010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTFreeList<ActorPathingMessage>::BSTFreeList` (by the vtable it
/// stores): the vtable, the lock word and the free head cleared. Returns
/// `this`.
pub fn fn_006ed010(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, FREE_LIST_VTABLE);
    e.mem.set_u32(base.wrapping_add(FREE_LIST_LOCK), 0);
    e.mem.set_u32(base.wrapping_add(FREE_LIST_HEAD), 0);
    this
}

// Translated from 006ed040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`; unoptimized code): makes `count` elements
/// from `elements` on the free list: the head is `elements`, then under the
/// lock each of the first `count - 1` elements (unsigned: a count of 0
/// wraps) points at the next and the last one's next pointer is cleared.
pub fn fn_006ed040(e: &mut Engine, this: Ptr, count: u32, elements: u32) {
    let base = this.addr();
    e.mem.set_u32(base.wrapping_add(FREE_LIST_HEAD), elements);
    lock(e, base);
    let mut index = 0u32;
    let mut element = elements;
    while index < count.wrapping_sub(1) {
        e.mem.set_u32(
            element.wrapping_add(ELEMENT_NEXT),
            element.wrapping_add(ELEMENT_SIZE),
        );
        index = index.wrapping_add(1);
        element = element.wrapping_add(ELEMENT_SIZE);
    }
    e.mem.set_u32(element.wrapping_add(ELEMENT_NEXT), 0);
    unlock(e, base);
}

// Translated from 006ed0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element constructor `006ece90` hands to the vector constructor:
/// clears the element's next pointer (+8); returns `this`.
pub fn fn_006ed0e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr().wrapping_add(ELEMENT_NEXT), 0);
    this
}

/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// The scalar deleting destructor shape: `destroy(this)`, then `operator delete(this)` when
/// bit 0 of `flags` is set. Returns `this`.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006ecf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BSTFreeList<ActorPathingMessage>` (slot +0x00 of its
/// vtable `0106d7a0`; `thiscall`, `RET 4`): [`fn_006ecf20`]; see [`scalar_deleting`].
pub fn fn_006ecf40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_006ecf20)
}

// Translated from 006ecf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BSTStaticFreeList<ActorPathingMessage,256>` (slot
/// +0x00 of its vtable `0106d798`; `thiscall`, `RET 4`): [`fn_006ecfa0`].
pub fn fn_006ecf70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_006ecfa0)
}

/// The constructors `006ed100` runs on its members (thiscall): `006815c0` (a point's
/// default constructor, answering its `this`), `006bbc30` (in `navmeshobstaclecutter.cpp`
/// by the engine map), `006f2630`, `006f2750`, `006f2510` (three words each: 0, 0 and an
/// address) and `006f2b30`; and the `float` stored at `+0x2C` (`0101e340`, read at run
/// time).
const POINT_DEFAULT: u32 = 0x0068_15c0;
const CONSTRUCT_006BBC30: u32 = 0x006b_bc30;
const CONSTRUCT_006F2630: u32 = 0x006f_2630;
const CONSTRUCT_006F2750: u32 = 0x006f_2750;
const CONSTRUCT_006F2510: u32 = 0x006f_2510;
const CONSTRUCT_006F2B30: u32 = 0x006f_2b30;
const FLOAT_AT_2C: u32 = 0x0101_e340;

// Translated from 006ed100 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ed100 - the compiler's exception frame (FS:[0] unwind states 0..5) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`; called from `006cd8a0`, `006d1c60` and
/// `006d1f80`): a constructor. The points at `+0x08` and `+0x14` (`006815c0`), the float at
/// `+0x2C` from `0101e340`, the member at `+0x30` (`006bbc30`), then the members at `+0x40`
/// and `+0x54` (`006f2630`), `+0x68` and `+0x7C` (`006f2750`) and `+0x90` (`006f2510`), each
/// given `(0, 0, address)` with the address `006815c0` answers for `+0x30` (called again
/// before each), and the member at `+0xA4` (`006f2b30`). Returns `this`.
pub fn fn_006ed100(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(POINT_DEFAULT, &args![base + 0x8]);
    e.call(POINT_DEFAULT, &args![base + 0x14]);
    let value = x87_quiet_bits(e.mem.u32(FLOAT_AT_2C));
    e.mem.set_u32(base + 0x2c, value);
    e.call(CONSTRUCT_006BBC30, &args![base + 0x30]);
    for (offset, construct) in [
        (0x40u32, CONSTRUCT_006F2630),
        (0x54, CONSTRUCT_006F2630),
        (0x68, CONSTRUCT_006F2750),
        (0x7c, CONSTRUCT_006F2750),
        (0x90, CONSTRUCT_006F2510),
    ] {
        let point = e.call(POINT_DEFAULT, &args![base + 0x30]).u32();
        e.call(construct, &args![base + offset, 0u32, 0u32, point]);
    }
    e.call(CONSTRUCT_006F2B30, &args![base + 0xa4]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_ce90, fn_006ece90(Ptr) -> Ptr),
        entry!(0x006e_cd60, fn_006ecd60(Ptr, Ptr) -> bool),
        entry!(0x006e_ce00, fn_006ece00(Ptr, Ptr) -> Ptr),
        entry!(0x006e_cf20, fn_006ecf20(Ptr)),
        entry!(0x006e_cfa0, fn_006ecfa0(Ptr)),
        entry!(0x006e_d010, fn_006ed010(Ptr) -> Ptr),
        entry!(0x006e_d040, fn_006ed040(Ptr, u32, u32)),
        entry!(0x006e_d0e0, fn_006ed0e0(Ptr) -> Ptr),
        entry!(0x006e_cf40, fn_006ecf40(Ptr, u32) -> Ptr),
        entry!(0x006e_cf70, fn_006ecf70(Ptr, u32) -> Ptr),
        entry!(0x006e_d100, fn_006ed100(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn constructs_the_base_the_elements_and_the_free_list() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_006ED010, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            // The vtable is already stored when the elements are constructed.
            let seen = e.mem.u32(a[0] - 0xc);
            e.mem.set_u32(a[0], seen);
            Ret::default()
        });
        e.register(FOLLOW_UP_006ED040, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_ce90, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_006ED010),
            vec![vec![this]]
        );
        assert_eq!(
            calls_to(&mut e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![this + 0xc, 0xc, 0x100, 0x006e_d0e0, 0x0048_3710]]
        );
        assert_eq!(
            calls_to(&mut e, FOLLOW_UP_006ED040),
            vec![vec![this, 0x100, this + 0xc]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_006ECE90);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_006ECE90);
    }
}

#[cfg(test)]
mod free_list_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles for the lock: the compare-exchange answers `busy` times 1
    /// before it answers 0, and every lock call is logged.
    fn fixture(busy: u32) -> (Engine, Log) {
        let mut e = Engine::new();
        let log = Log::default();
        let l = log.clone();
        let mut left = busy;
        e.register_double(COMPARE_EXCHANGE, move |_, a| {
            l.borrow_mut().push((COMPARE_EXCHANGE, a.to_vec()));
            if a[1] == 0 && left > 0 {
                left -= 1;
                ret(1)
            } else {
                ret(0)
            }
        });
        let l = log.clone();
        e.register_double(EMPTY_HOOK, move |_, a| {
            l.borrow_mut().push((EMPTY_HOOK, a.to_vec()));
            Ret::default()
        });
        (e, log)
    }

    #[test]
    fn allocate_takes_the_head_element_under_the_lock() {
        let (mut e, log) = fixture(2);
        let this = e.mem.alloc(0x0c);
        let first = e.mem.alloc(0x0c);
        let second = e.mem.alloc(0x0c);
        e.mem.set_u32(this + 8, first);
        e.mem.set_u32(first + 8, second);
        let out = e.mem.alloc(4);
        assert!(e.call(0x006e_cd60, &args![this, out]).bool());
        assert_eq!(e.mem.u32(out), first);
        assert_eq!(e.mem.u32(this + 8), second);
        assert_eq!(e.mem.u32(first + 8), 0);
        let lock = this + 4;
        assert_eq!(
            *log.borrow(),
            [
                (COMPARE_EXCHANGE, vec![lock, 0, 1]),
                (COMPARE_EXCHANGE, vec![lock, 0, 1]),
                (COMPARE_EXCHANGE, vec![lock, 0, 1]),
                (EMPTY_HOOK, vec![]),
                (EMPTY_HOOK, vec![]),
                (COMPARE_EXCHANGE, vec![lock, 1, 0]),
            ]
        );
    }

    #[test]
    fn allocate_from_an_empty_list_answers_null() {
        let (mut e, _) = fixture(0);
        let this = e.mem.alloc(0x0c);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        assert!(!e.call(0x006e_cd60, &args![this, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn copy_remakes_the_message_from_the_source() {
        let mut e = Engine::new();
        e.register(ELEMENT_DESTRUCTOR_00483710, |_, _| Ret::default());
        e.register(COPY_TWO_WORDS, |e, a| {
            let first = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], first);
            ret(a[0])
        });
        e.register(PLACEMENT_NEW, |_, a| ret(a[1]));
        let this = e.mem.alloc(8);
        let source = e.mem.alloc(8);
        e.mem.set_u32(source, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_ce00, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x77);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|call| call.0).skip(1).collect();
        assert_eq!(
            order,
            [ELEMENT_DESTRUCTOR_00483710, PLACEMENT_NEW, COPY_TWO_WORDS]
        );
        // A null placement skips the copy and answers 0.
        e.register(PLACEMENT_NEW, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_ce00, &args![this, source]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|call| call.0 == COPY_TWO_WORDS));
    }

    #[test]
    fn destructors_store_their_vtables_and_destroy_the_elements() {
        let mut e = Engine::new();
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |e, a| {
            // The static list's vtable is stored before the elements go.
            let vtable = e.mem.u32(a[0] - 0xc);
            e.mem.set_u32(a[0], vtable);
            Ret::default()
        });
        let this = e.mem.alloc(0x0c + 0xc00);
        e.call_log = Some(vec![]);
        e.call(0x006e_cfa0, &args![this]);
        assert_eq!(e.mem.u32(this), FREE_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), VTABLE_006ECE90);
        let log = e.call_log.take().unwrap();
        let destroyed: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VECTOR_DESTRUCTOR_ITERATOR)
            .map(|call| call.1.clone())
            .collect();
        assert_eq!(destroyed, [vec![this + 0xc, 0xc, 0x100, 0x0048_3710]]);
        e.mem.set_u32(this, 0);
        e.call(0x006e_cf20, &args![this]);
        assert_eq!(e.mem.u32(this), FREE_LIST_VTABLE);
    }

    #[test]
    fn constructors_clear_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x0c);
        e.mem.write(this, &[0xaa; 0x0c]);
        assert_eq!(e.call(0x006e_d010, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), FREE_LIST_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        let element = e.mem.alloc(0x0c);
        e.mem.write(element, &[0xaa; 0x0c]);
        assert_eq!(e.call(0x006e_d0e0, &args![element]).u32(), element);
        assert_eq!(e.mem.u32(element), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(element + 8), 0);
    }

    #[test]
    fn initialise_chains_the_elements_under_the_lock() {
        let (mut e, log) = fixture(0);
        let this = e.mem.alloc(0x0c);
        let elements = e.mem.alloc(3 * 0xc);
        e.mem.write(elements, &[0xaa; 3 * 0xc]);
        e.call(0x006e_d040, &args![this, 3u32, elements]);
        assert_eq!(e.mem.u32(this + 8), elements);
        assert_eq!(e.mem.u32(elements + 8), elements + 0xc);
        assert_eq!(e.mem.u32(elements + 0xc + 8), elements + 0x18);
        assert_eq!(e.mem.u32(elements + 0x18 + 8), 0);
        assert_eq!(e.mem.u32(elements), 0xaaaa_aaaa);
        let lock = this + 4;
        assert_eq!(
            *log.borrow(),
            [
                (COMPARE_EXCHANGE, vec![lock, 0, 1]),
                (EMPTY_HOOK, vec![]),
                (EMPTY_HOOK, vec![]),
                (COMPARE_EXCHANGE, vec![lock, 1, 0]),
            ]
        );
        // One element: no chaining, its next pointer cleared.
        let single = e.mem.alloc(0x0c);
        e.mem.write(single, &[0xaa; 0x0c]);
        e.call(0x006e_d040, &args![this, 1u32, single]);
        assert_eq!(e.mem.u32(single + 8), 0);
        assert_eq!(e.mem.u32(this + 8), single);
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn deleting_destructors_put_their_vtables_back() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(VECTOR_DESTRUCTOR_ITERATOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_cf40, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FREE_LIST_VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
        e.call(0x006e_cf70, &args![this, 0u32]);
        assert_eq!(calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR).len(), 1);
        // The static list's destructor ends in the base one.
        assert_eq!(e.mem.u32(this), FREE_LIST_VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn constructor_builds_the_members_in_order() {
        let mut e = Engine::new();
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f32(FLOAT_AT_2C, 32.0);
        e.register(POINT_DEFAULT, |_, a| a[0].into_ret());
        for address in [
            CONSTRUCT_006BBC30,
            CONSTRUCT_006F2630,
            CONSTRUCT_006F2750,
            CONSTRUCT_006F2510,
            CONSTRUCT_006F2B30,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xc0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_d100, &args![this]).u32(), this);
        assert_eq!(e.mem.f32(this + 0x2c), 32.0);
        assert_eq!(
            calls_to(&e, CONSTRUCT_006F2630),
            vec![
                vec![this + 0x40, 0, 0, this + 0x30],
                vec![this + 0x54, 0, 0, this + 0x30]
            ]
        );
        assert_eq!(
            calls_to(&e, CONSTRUCT_006F2510),
            vec![vec![this + 0x90, 0, 0, this + 0x30]]
        );
        assert_eq!(calls_to(&e, POINT_DEFAULT).len(), 7);
        assert_eq!(calls_to(&e, CONSTRUCT_006F2B30), vec![vec![this + 0xa4]]);
    }
}
