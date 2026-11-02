//! `Havok SDK/hkpconstrainedsystemfilterclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dffb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e4_6b80`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dffb00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e4_6b80, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dffb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0xc` (`SUB ECX,0xc`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00e46b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the constrained system filter (the engine map gives it no name; it is
/// the one the deleting destructor `00dffb00` runs, and the first vtable it installs,
/// `010e7874`, is the class's): installs the class's vtables (`+0` and the secondary ones at
/// `+8`, `+0xC`, `+0x10`, `+0x14`, `+0x30`), releases the object at `+0x34` when there is
/// one (`00c905b0`, a reference count release), then installs the base class vtables, the
/// last of them `hkBaseObject`'s `0102e388` at `+0`.
pub fn fn_00e46b80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let held = e.mem.u32(base + 0x34);
    e.mem.set_u32(base, 0x010e_7874);
    e.mem.set_u32(base + 0x8, 0x010e_7868);
    e.mem.set_u32(base + 0xc, 0x010e_7854);
    e.mem.set_u32(base + 0x10, 0x010e_7848);
    e.mem.set_u32(base + 0x14, 0x010e_783c);
    e.mem.set_u32(base + 0x30, 0x010e_781c);
    if held != 0 {
        e.call(0x00c9_05b0, &args![held]);
    }
    e.mem.set_u32(base + 0x30, 0x010c_b1e4);
    e.mem.set_u32(base + 0x14, 0x010c_3b8c);
    e.mem.set_u32(base + 0x10, 0x010c_3bac);
    e.mem.set_u32(base + 0xc, 0x010c_3b98);
    e.mem.set_u32(base + 0x8, 0x010c_3b80);
    e.mem.set_u32(base, 0x0102_e388);
}

/// The owner of a collidable: the object `m_ownerOffset` (signed byte `+0x10`) bytes from
/// it when its broad phase handle type byte (`+0x18`) is 1 (an entity), else 0.
fn entity_owner(e: &Engine, collidable: u32) -> u32 {
    if e.mem.u8(collidable + 0x18) == 1 {
        collidable.wrapping_add(e.mem.u8(collidable + 0x10) as i8 as u32)
    } else {
        0
    }
}

// Translated from 00e46bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `isCollisionEnabled` taking two collidables (`RET 0xC`: the result byte pointer `out`,
/// collidable A and collidable B). The engine map labels it
/// `hkpConstrainedSystemFilter::_vector_deleting_destructor__ov2`, which the code does not
/// match. First the filter at `[this + 0x2C]`, when set, is asked (slot `4` of the object at
/// filter `+ 8`, with a result buffer and the two collidables); a zero byte in its answer
/// stores 0 to `*out`. The owners of the two collidables (an entity at the offset given by
/// byte `+0x10`, when byte `+0x18` is 1) are then looked up; when either is missing the
/// answer is 1. Otherwise the owner with fewer constraints (`00c9c380`; B when A has more,
/// else A) is scanned with `00c9c3c0`: a constraint (not null, whose object at `+0xC` has a
/// slot-`0x20` answer other than `0xB`) with the other owner at `+0x14` or `+0x18` stores 0.
/// Anything else stores 1. Returns `out` (`EAX`).
pub fn fn_00e46bf0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    collidable_a: Ptr,
    collidable_b: Ptr,
) -> Ptr {
    let owner_a = entity_owner(e, collidable_a.addr());
    let owner_b = entity_owner(e, collidable_b.addr());
    let filter = e.mem.u32(this.addr() + 0x2c);
    if filter != 0 {
        let enabled = e.with_stack(4, |e, result| {
            let answer = e
                .vcall(filter + 8, 4, &args![result, collidable_a, collidable_b])
                .u32();
            e.mem.u8(answer)
        });
        if enabled == 0 {
            e.mem.set_u8(out.addr(), 0);
            return out;
        }
    }
    if owner_a == 0 || owner_b == 0 {
        e.mem.set_u8(out.addr(), 1);
        return out;
    }
    let count_b = e.call(0x00c9_c380, &args![owner_b]).i32();
    let count_a = e.call(0x00c9_c380, &args![owner_a]).i32();
    let (scanned, other) = if count_a > count_b {
        (owner_b, owner_a)
    } else {
        (owner_a, owner_b)
    };
    let count = e.call(0x00c9_c380, &args![scanned]).i32();
    let mut index = 0;
    while index < count {
        let constraint = e.call(0x00c9_c3c0, &args![scanned, index]).u32();
        if constraint != 0 {
            let object = e.mem.u32(constraint + 0xc);
            let kind = e.vcall(object, 0x20, &args![]).u32();
            if kind != 0xb
                && (e.mem.u32(constraint + 0x14) == other || e.mem.u32(constraint + 0x18) == other)
            {
                e.mem.set_u8(out.addr(), 0);
                return out;
            }
        }
        index += 1;
    }
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00e46da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `isCollisionEnabled` taking four words (`RET 0x14`: the result byte pointer `out` and
/// four words). The engine map labels it
/// `hkpConstrainedSystemFilter::_vector_deleting_destructor__ov3`, which the code does not
/// match. When the filter at `[this + 0x24]` is set, asks it (slot `0` of the object at
/// filter `+ 0x10`, with a result buffer and the four words) and stores 0 to `*out` when the
/// byte its answer points to is 0; otherwise stores 1. Returns `out` (`EAX`).
pub fn fn_00e46da0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
) -> Ptr {
    let filter = e.mem.u32(this.addr() + 0x24);
    if filter != 0 {
        let enabled = e.with_stack(4, |e, result| {
            let answer = e
                .vcall(
                    filter + 0x10,
                    0,
                    &args![result, word_1, word_2, word_3, word_4],
                )
                .u32();
            e.mem.u8(answer)
        });
        if enabled == 0 {
            e.mem.set_u8(out.addr(), 0);
            return out;
        }
    }
    e.mem.set_u8(out.addr(), 1);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_fb00, deleting_destructor_00dffb00(Ptr, u8) -> Ptr),
        entry!(0x00df_fb40, fn_00dffb40(Ptr, u32) -> u32),
        entry!(0x00e4_6b80, fn_00e46b80(Ptr)),
        entry!(0x00e4_6bf0, fn_00e46bf0(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e4_6da0, fn_00e46da0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dffb00_frees_only_when_bit_zero_is_set() {
        check(0x00df_fb00, 0x00e4_6b80);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dffb40() {
        assert_eq!(check(0x00df_fb40, 0x00df_fb00, 0xc), 0xdead_beef);
    }
}

#[cfg(test)]
mod a08_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const FILTER_SLOT_4: u32 = 0x0900_0004;
    const FILTER_SLOT_0: u32 = 0x0900_0100;
    const COUNT: u32 = 0x00c9_c380;
    const CONSTRAINT_AT: u32 = 0x00c9_c3c0;
    const RELEASE: u32 = 0x00c9_05b0;

    /// A filter object with a double at slot 4 of `filter + 8` answering a byte
    /// `answer` and one at slot 0 of `filter + 0x10`; returns it.
    fn filter(e: &mut Engine, log: &Log, answer: u8) -> u32 {
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, answer);
        log_double(e, log, FILTER_SLOT_4, byte);
        log_double(e, log, FILTER_SLOT_0, byte);
        let vtable_8 = e.mem.alloc(8);
        e.mem.set_u32(vtable_8 + 4, FILTER_SLOT_4);
        let vtable_10 = e.mem.alloc(8);
        e.mem.set_u32(vtable_10, FILTER_SLOT_0);
        let filter = e.mem.alloc(0x40);
        e.mem.set_u32(filter + 8, vtable_8);
        e.mem.set_u32(filter + 0x10, vtable_10);
        filter
    }

    #[test]
    fn destructor_body_installs_vtables_and_releases_00e46b80() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let held = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x34, held);
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(RELEASE, move |e, a| {
            let mut words = vec![a[0]];
            words.extend([0x0, 0x8, 0xc, 0x10, 0x14, 0x30].map(|offset| e.mem.u32(this + offset)));
            copy.borrow_mut().extend(words);
            Ret::default()
        });
        e.call(0x00e4_6b80, &[this]);
        // At the release the class vtables are installed.
        assert_eq!(
            *seen.borrow(),
            vec![
                held,
                0x010e_7874,
                0x010e_7868,
                0x010e_7854,
                0x010e_7848,
                0x010e_783c,
                0x010e_781c
            ]
        );
        let after: Vec<u32> = [0x0, 0x8, 0xc, 0x10, 0x14, 0x30]
            .map(|offset| e.mem.u32(this + offset))
            .to_vec();
        assert_eq!(
            after,
            vec![
                0x0102_e388,
                0x010c_3b80,
                0x010c_3b98,
                0x010c_3bac,
                0x010c_3b8c,
                0x010c_b1e4
            ]
        );
    }

    #[test]
    fn destructor_body_without_a_held_object_00e46b80() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let log = new_log();
        log_double(&mut e, &log, RELEASE, 0);
        e.call(0x00e4_6b80, &[this]);
        assert!(calls(&log, RELEASE).is_empty());
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }

    #[test]
    fn four_word_variant_asks_the_filter_00e46da0() {
        for answer in [0u8, 1] {
            let mut e = engine();
            let log = new_log();
            let filter = filter(&mut e, &log, answer);
            let this = e.mem.alloc(0x40);
            e.mem.set_u32(this + 0x24, filter);
            let out = e.mem.alloc(4);
            let result = e.call(0x00e4_6da0, &[this, out, 1, 2, 3, 4]);
            assert_eq!(result.u32(), out);
            assert_eq!(e.mem.u8(out), answer);
            let asked = calls(&log, FILTER_SLOT_0);
            assert_eq!(asked.len(), 1);
            assert_eq!(asked[0][0], filter + 0x10);
            assert_eq!(asked[0][2..], [1, 2, 3, 4]);
        }
    }

    #[test]
    fn four_word_variant_without_a_filter_enables_00e46da0() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(4);
        let result = e.call(0x00e4_6da0, &[this, out, 1, 2, 3, 4]);
        assert_eq!(result.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
    }

    /// A collidable with the given handle type byte whose owner block lies `0x40` bytes
    /// before it. Returns `(collidable, owner)`.
    fn collidable(e: &mut Engine, kind: u8) -> (u32, u32) {
        let owner = e.mem.alloc(0x100);
        let collidable = owner + 0x40;
        e.mem.set_u8(collidable + 0x10, 0xc0);
        e.mem.set_u8(collidable + 0x18, kind);
        (collidable, owner)
    }

    /// A constraint whose object answers `kind` in slot `0x20` and that connects the two
    /// owners.
    fn constraint(e: &mut Engine, log: &Log, id: u32, kind: u32, a: u32, b: u32) -> u32 {
        let double = 0x0910_0000 + id * 0x100;
        log_double(e, log, double, kind);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, double);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 0xc, object);
        e.mem.set_u32(constraint + 0x14, a);
        e.mem.set_u32(constraint + 0x18, b);
        constraint
    }

    struct Scene {
        e: Engine,
        log: Log,
        this: u32,
        out: u32,
        a: u32,
        b: u32,
        owner_a: u32,
        owner_b: u32,
    }

    /// Two entity collidables whose owners have `counts` constraints, listed by
    /// `lists = (owner_a_list, owner_b_list)`; no filter.
    fn scene(counts: (i32, i32), lists: (Vec<u32>, Vec<u32>)) -> Scene {
        let mut e = engine();
        let log = new_log();
        let (a, owner_a) = collidable(&mut e, 1);
        let (b, owner_b) = collidable(&mut e, 1);
        let table = [(owner_a, counts.0, lists.0), (owner_b, counts.1, lists.1)];
        let count_table = table.clone();
        let seen = Rc::clone(&log);
        e.register_double(COUNT, move |_, args| {
            seen.borrow_mut().push((COUNT, args.to_vec()));
            let eax = count_table.iter().find(|row| row.0 == args[0]).unwrap().1;
            Ret {
                eax: eax as u32,
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(CONSTRAINT_AT, move |_, args| {
            seen.borrow_mut().push((CONSTRAINT_AT, args.to_vec()));
            let row = table.iter().find(|row| row.0 == args[0]).unwrap();
            Ret {
                eax: row.2.get(args[1] as usize).copied().unwrap_or(0),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0x55);
        Scene {
            e,
            log,
            this,
            out,
            a,
            b,
            owner_a,
            owner_b,
        }
    }

    fn run(s: &mut Scene) -> u32 {
        let result = s.e.call(0x00e4_6bf0, &[s.this, s.out, s.a, s.b]);
        assert_eq!(result.u32(), s.out);
        u32::from(s.e.mem.u8(s.out))
    }

    #[test]
    fn filter_answer_zero_disables_before_anything_else_00e46bf0() {
        let mut s = scene((0, 0), (vec![], vec![]));
        let filter = filter(&mut s.e, &s.log, 0);
        s.e.mem.set_u32(s.this + 0x2c, filter);
        assert_eq!(run(&mut s), 0);
        let asked = calls(&s.log, FILTER_SLOT_4);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][0], filter + 8);
        assert_eq!(asked[0][2..], [s.a, s.b]);
        assert!(calls(&s.log, COUNT).is_empty());
    }

    #[test]
    fn filter_answer_nonzero_goes_on_to_the_constraints_00e46bf0() {
        let mut s = scene((0, 0), (vec![], vec![]));
        let filter = filter(&mut s.e, &s.log, 1);
        s.e.mem.set_u32(s.this + 0x2c, filter);
        assert_eq!(run(&mut s), 1);
        assert_eq!(calls(&s.log, COUNT).len(), 3);
    }

    #[test]
    fn a_collidable_without_an_entity_owner_is_enabled_00e46bf0() {
        let mut s = scene((0, 0), (vec![], vec![]));
        s.e.mem.set_u8(s.a + 0x18, 2);
        assert_eq!(run(&mut s), 1);
        assert!(calls(&s.log, COUNT).is_empty());
        let mut s = scene((0, 0), (vec![], vec![]));
        s.e.mem.set_u8(s.b + 0x18, 0);
        assert_eq!(run(&mut s), 1);
        assert!(calls(&s.log, COUNT).is_empty());
    }

    #[test]
    fn the_owner_with_fewer_constraints_is_scanned_00e46bf0() {
        // A has more constraints than B: B is scanned, and a constraint holding A at +0x14
        // disables the pair.
        let mut s = scene((3, 1), (vec![], vec![]));
        let log = Rc::clone(&s.log);
        let hit = constraint(&mut s.e, &log, 1, 5, s.owner_a, 0x1234);
        s.e.register_double(CONSTRAINT_AT, {
            let owner_b = s.owner_b;
            move |_, args| Ret {
                eax: if args[0] == owner_b && args[1] == 0 {
                    hit
                } else {
                    0
                },
                ..Ret::default()
            }
        });
        assert_eq!(run(&mut s), 0);
        let counts: Vec<u32> = s
            .log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == COUNT)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(counts, vec![s.owner_b, s.owner_a, s.owner_b]);
    }

    #[test]
    fn equal_counts_scan_owner_a_and_match_the_second_slot_00e46bf0() {
        let mut s = scene((2, 2), (vec![], vec![]));
        let log = Rc::clone(&s.log);
        let hit = constraint(&mut s.e, &log, 1, 5, 0x1234, s.owner_b);
        s.e.register_double(CONSTRAINT_AT, {
            let owner_a = s.owner_a;
            let seen = Rc::clone(&s.log);
            move |_, args| {
                seen.borrow_mut().push((CONSTRAINT_AT, args.to_vec()));
                Ret {
                    eax: if args[0] == owner_a && args[1] == 1 {
                        hit
                    } else {
                        0
                    },
                    ..Ret::default()
                }
            }
        });
        assert_eq!(run(&mut s), 0);
        let scans: Vec<Vec<u32>> = calls(&s.log, CONSTRAINT_AT);
        assert_eq!(scans, vec![vec![s.owner_a, 0], vec![s.owner_a, 1]]);
    }

    #[test]
    fn constraints_of_type_0xb_null_slots_and_strangers_are_skipped_00e46bf0() {
        // A (3 constraints) is not above B (5), so A is scanned.
        let mut s = scene((3, 5), (vec![], vec![]));
        let log = Rc::clone(&s.log);
        let ignored = constraint(&mut s.e, &log, 1, 0xb, s.owner_b, s.owner_a);
        let stranger = constraint(&mut s.e, &log, 2, 5, 0x1111, 0x2222);
        let list = [ignored, 0, stranger];
        s.e.register_double(CONSTRAINT_AT, move |_, args| Ret {
            eax: list.get(args[1] as usize).copied().unwrap_or(0),
            ..Ret::default()
        });
        assert_eq!(run(&mut s), 1);
        // The first constraint is asked for its type (0xB, so skipped even though it holds
        // owner B), the null slot is skipped, the third is asked and does not match.
        assert_eq!(calls(&s.log, 0x0910_0100).len(), 1);
        assert_eq!(calls(&s.log, 0x0910_0200).len(), 1);
    }
}
