//! `Havok SDK/run_00cdf850` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three cdecl functions that set a pair of vectors in a constraint modifier of one object
//! (`00cdf850` and `00cdfac0` for modifier types `0x18` and `0x1c`, `00cdf960` for type
//! `0x18` with a vector and a constant). They write Havok's profiling records (the label
//! `"TtSetMassChang"` and the end label `"Et"` with `RDTSC` time stamps appended to the
//! thread's monitor stream, found with `TlsGetValue` of the index at `0126835c`); the engine
//! has no model of `RDTSC`, so the host clock stands in for the time stamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator new modifiers come from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtSetMassChang"` (`010cb7b8`) and the end record `"Et"` (`010c41f8`).
const LABEL_START: u32 = 0x010c_b7b8;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpWorldConstraintUtil::findModifier` (`00d277c0`, cdecl `(owner, type)`) and `addModifier`
/// (`00d27960`, cdecl `(owner, entity, modifier)`), named by Ghidra from the Xbox PDB.
const FIND_MODIFIER: u32 = 0x00d2_77c0;
const ADD_MODIFIER: u32 = 0x00d2_7960;
/// The float `1.0` (`01012068`).
const ONE: u32 = 0x0101_2068;
/// The size of a modifier block (`0x40`).
const MODIFIER_SIZE: u32 = 0x40;

/// The stand-in for `RDTSC`'s low word: the host clock.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (`+4`) is below its limit (`+0xc`).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Copies one 16-byte vector (`MOVAPS` load and store), bit for bit.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// Finds the modifier of `kind` of `owner` (`00d277c0`); when there is none, takes `0x40` bytes
/// from the memory router's allocator (slot `4`), gives the block (when it is not null) the
/// 16-bit type word at `+0` and the 16-bit size word `0x40` at `+0x10`, and adds it to the
/// owner for `entity` (`00d27960`, called with a null modifier when the allocation failed).
/// Returns the modifier and whether it was made.
fn find_or_make_modifier(e: &mut Engine, owner: u32, entity: u32, kind: u32) -> (u32, bool) {
    let found = e.call(FIND_MODIFIER, &args![owner, kind]).u32();
    if found != 0 {
        return (found, false);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![MODIFIER_SIZE]).u32();
    let modifier = if block != 0 {
        e.mem.set_u16(block, kind as u16);
        e.mem.set_u16(block + 0x10, MODIFIER_SIZE as u16);
        block
    } else {
        0
    };
    e.call(ADD_MODIFIER, &args![owner, entity, modifier]);
    (modifier, true)
}

/// The shared body of `00cdf850` and `00cdfac0`: asks `first_object` (virtual method `+0x40`)
/// for the owner, does nothing when it is null, otherwise calls virtual method `+0x18` of
/// `second_object`, writes the start record, finds or makes the modifier of `kind`, stores
/// the two vectors in it (`first_vector` at `+0x20` and `second_vector` at `+0x30` when
/// `side` equals the owner's word at `+0x14`, the other way round otherwise) and writes the
/// end record.
fn set_vector_pair(
    e: &mut Engine,
    kind: u32,
    first_object: u32,
    side: u32,
    second_object: u32,
    first_vector: u32,
    second_vector: u32,
) {
    let owner = e.vcall(first_object, 0x40, &args![]).u32();
    if owner == 0 {
        return;
    }
    e.vcall(second_object, 0x18, &args![]);
    profile_record(e, LABEL_START);
    let (modifier, _) = find_or_make_modifier(e, owner, second_object, kind);
    if side == e.mem.u32(owner + 0x14) {
        copy_vector(e, first_vector, modifier + 0x20);
        copy_vector(e, second_vector, modifier + 0x30);
    } else {
        copy_vector(e, second_vector, modifier + 0x20);
        copy_vector(e, first_vector, modifier + 0x30);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00cdf850 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(first_object, side, _unused_2, second_object, first_vector, second_vector)`: sets
/// the pair of vectors of the type-`0x18` modifier of the owner (see [`set_vector_pair`]).
/// The class is unconfirmed; the third word is never read.
pub fn fn_00cdf850(
    e: &mut Engine,
    first_object: u32,
    side: u32,
    _unused_2: u32,
    second_object: u32,
    first_vector: Ptr,
    second_vector: Ptr,
) {
    set_vector_pair(
        e,
        0x18,
        first_object,
        side,
        second_object,
        first_vector.addr(),
        second_vector.addr(),
    );
}

// Translated from 00cdfac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same as `00cdf850` for the modifier of type `0x1c` (see [`set_vector_pair`]).
pub fn fn_00cdfac0(
    e: &mut Engine,
    first_object: u32,
    side: u32,
    _unused_2: u32,
    second_object: u32,
    first_vector: Ptr,
    second_vector: Ptr,
) {
    set_vector_pair(
        e,
        0x1c,
        first_object,
        side,
        second_object,
        first_vector.addr(),
        second_vector.addr(),
    );
}

// Translated from 00cdf960 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(first_object, side, second_object, vector)`: like `00cdf850` with one vector. The
/// owner comes from `first_object` (virtual method `+0x40`; nothing happens when it is
/// null), virtual method `+0x18` of `second_object` is called, the start record written and
/// the type-`0x18` modifier found. When it was made, `vector` goes to `+0x20` and `1.0` in
/// every lane to `+0x30` if `side` equals the owner's word at `+0x14`, the other way round
/// otherwise. When it already existed, `vector` replaces the `+0x20` lanes (same `side`) or
/// the `+0x30` lanes, and when both the fourth lane of the first vector (`+0x2c`) and of the
/// second (`+0x3c`) compare equal to `0.0` (a NaN does not) both are set to `1.0`. The end
/// record follows.
pub fn fn_00cdf960(e: &mut Engine, first_object: u32, side: u32, second_object: u32, vector: Ptr) {
    let owner = e.vcall(first_object, 0x40, &args![]).u32();
    if owner == 0 {
        return;
    }
    e.vcall(second_object, 0x18, &args![]);
    profile_record(e, LABEL_START);
    let (modifier, made) = find_or_make_modifier(e, owner, second_object, 0x18);
    let same_side = side == e.mem.u32(owner + 0x14);
    if made {
        let one = e.mem.u32(ONE);
        let (vector_at, ones_at) = if same_side {
            (modifier + 0x20, modifier + 0x30)
        } else {
            (modifier + 0x30, modifier + 0x20)
        };
        copy_vector(e, vector.addr(), vector_at);
        for lane in 0..4 {
            e.mem.set_u32(ones_at + 4 * lane, one);
        }
    } else {
        let target = if same_side { 0x20 } else { 0x30 };
        copy_vector(e, vector.addr(), modifier + target);
        let first = x87_m32(e.mem.u32(modifier + 0x2c));
        if first == 0.0 {
            let second = x87_m32(e.mem.u32(modifier + 0x3c));
            if second == 0.0 {
                e.mem.set_u32(modifier + 0x2c, 0x3f80_0000);
                e.mem.set_u32(modifier + 0x3c, 0x3f80_0000);
            }
        }
    }
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd_f850, fn_00cdf850(u32, u32, u32, u32, Ptr, Ptr)),
        entry!(0x00cd_f960, fn_00cdf960(u32, u32, u32, Ptr)),
        entry!(0x00cd_fac0, fn_00cdfac0(u32, u32, u32, u32, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const OWNER_SLOT_DOUBLE: u32 = 0x0900_0041;
    const NOTIFY_SLOT_DOUBLE: u32 = 0x0900_0061;
    const ALLOCATE_DOUBLE: u32 = 0x0900_0021;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        log: Log,
        first: u32,
        second: u32,
        owner: u32,
        stream: u32,
        block: u32,
    }

    /// `first_object` answers its slot `0x40` with `owner` (word `+0x14` = 7), `second_object`
    /// has a slot `0x18`, the monitor stream and the allocator exist, and `findModifier`
    /// answers with `found` (0: none). `addModifier` is a double.
    fn rig(owner_answer: bool, found: u32) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(owner + 0x14, 7);
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x40, OWNER_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x18, NOTIFY_SLOT_DOUBLE);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first, vtable);
        e.mem.set_u32(second, vtable);
        let sink = Rc::clone(&log);
        let answer = if owner_answer { owner } else { 0 };
        e.register_double(OWNER_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("owner", a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(NOTIFY_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("notify", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(FIND_MODIFIER, move |_, a| {
            sink.borrow_mut().push(("find", a.to_vec()));
            Ret {
                eax: found,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(ADD_MODIFIER, move |_, a| {
            sink.borrow_mut().push(("add", a.to_vec()));
            Ret::default()
        });
        let block = e.mem.alloc(0x40);
        let allocator_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(allocator_vtable + 4, ALLOCATE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, allocator_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("allocate", a.to_vec()));
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        e.mem.set_u32(ONE, 0x3f80_0000);
        Rig {
            e,
            log,
            first,
            second,
            owner,
            stream,
            block,
        }
    }

    fn vector(e: &mut Engine, words: [u32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        for (i, word) in words.into_iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, word);
        }
        at
    }

    fn words(e: &Engine, at: u32) -> Vec<u32> {
        (0..4).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn labels(r: &Rig) -> Vec<u32> {
        let end = r.e.mem.u32(r.stream + 4);
        (r.stream + 0x40..end)
            .step_by(12)
            .map(|at| r.e.mem.u32(at))
            .collect()
    }

    /// With no owner nothing else happens; with one, a new modifier of the right type is made
    /// (type and size words, `0x40` bytes asked of the allocator), added and filled: the
    /// vectors go in order when `side` matches the owner's word and swapped when it does not.
    #[test]
    fn a_missing_modifier_is_made_and_filled_on_either_side() {
        for (address, kind) in [(0x00cd_f850u32, 0x18u32), (0x00cd_fac0, 0x1c)] {
            let mut r = rig(false, 0);
            let (a, b) = (
                vector(&mut r.e, [1, 2, 3, 4]),
                vector(&mut r.e, [5, 6, 7, 8]),
            );
            r.e.call(address, &args![r.first, 7u32, 0u32, r.second, a, b]);
            assert_eq!(r.log.borrow().len(), 1);
            assert!(labels(&r).is_empty());

            for (side, first_words, second_words) in [
                (7u32, [1, 2, 3, 4], [5, 6, 7, 8]),
                (8, [5, 6, 7, 8], [1, 2, 3, 4]),
            ] {
                let mut r = rig(true, 0);
                let (a, b) = (
                    vector(&mut r.e, [1, 2, 3, 4]),
                    vector(&mut r.e, [5, 6, 7, 8]),
                );
                r.e.call(address, &args![r.first, side, 0u32, r.second, a, b]);
                let calls: Vec<&str> = r.log.borrow().iter().map(|(what, _)| *what).collect();
                assert_eq!(calls, vec!["owner", "notify", "find", "allocate", "add"]);
                assert_eq!(r.log.borrow()[0].1, vec![r.first]);
                assert_eq!(r.log.borrow()[1].1, vec![r.second]);
                assert_eq!(r.log.borrow()[2].1, vec![r.owner, kind]);
                assert_eq!(r.log.borrow()[3].1[1], 0x40);
                assert_eq!(r.log.borrow()[4].1, vec![r.owner, r.second, r.block]);
                assert_eq!(r.e.mem.u16(r.block), kind as u16);
                assert_eq!(r.e.mem.u16(r.block + 0x10), 0x40);
                assert_eq!(words(&r.e, r.block + 0x20), first_words.to_vec());
                assert_eq!(words(&r.e, r.block + 0x30), second_words.to_vec());
                assert_eq!(labels(&r), vec![LABEL_START, LABEL_END]);
            }
        }
    }

    /// An existing modifier is filled in place, without allocating or adding.
    #[test]
    fn an_existing_modifier_is_updated_in_place() {
        let mut r = rig(true, 0);
        let existing = r.e.mem.alloc(0x40);
        r.e.register_double(FIND_MODIFIER, move |_, _| Ret {
            eax: existing,
            ..Ret::default()
        });
        let (a, b) = (
            vector(&mut r.e, [1, 2, 3, 4]),
            vector(&mut r.e, [5, 6, 7, 8]),
        );
        r.e.call(0x00cd_f850, &args![r.first, 7u32, 0u32, r.second, a, b]);
        assert_eq!(words(&r.e, existing + 0x20), vec![1, 2, 3, 4]);
        assert_eq!(words(&r.e, existing + 0x30), vec![5, 6, 7, 8]);
        let calls: Vec<&str> = r.log.borrow().iter().map(|(what, _)| *what).collect();
        assert_eq!(calls, vec!["owner", "notify"]);
    }

    /// The one-vector function: a new modifier gets the vector and a vector of ones on the
    /// two sides; an existing one gets the vector on one side and its two fourth lanes become
    /// `1.0` only when both compare equal to zero.
    #[test]
    fn the_one_vector_function_handles_new_and_existing_modifiers() {
        for (side, vector_at, ones_at) in [(7u32, 0x20u32, 0x30u32), (8, 0x30, 0x20)] {
            let mut r = rig(true, 0);
            let v = vector(&mut r.e, [1, 2, 3, 4]);
            r.e.call(0x00cd_f960, &args![r.first, side, r.second, v]);
            assert_eq!(words(&r.e, r.block + vector_at), vec![1, 2, 3, 4]);
            assert_eq!(words(&r.e, r.block + ones_at), vec![0x3f80_0000; 4]);
            assert_eq!(labels(&r), vec![LABEL_START, LABEL_END]);
        }
        // The copied vector's fourth lane lands at +0x2c (side 7) or +0x3c (side 8); the other
        // lane keeps its value. (copied lane, other lane, copied lane after, other lane after)
        let cases = [
            (0u32, 0u32, 0x3f80_0000u32, 0x3f80_0000u32),
            (0, 0x8000_0000, 0x3f80_0000, 0x3f80_0000),
            (0x4000_0000, 0, 0x4000_0000, 0),
            (0, 0x4000_0000, 0, 0x4000_0000),
            (0x7fc0_0000, 0, 0x7fc0_0000, 0),
        ];
        for (side, target) in [(7u32, 0x20u32), (8, 0x30)] {
            for (copied, other, copied_after, other_after) in cases {
                let mut r = rig(true, 0);
                let existing = r.e.mem.alloc(0x40);
                r.e.register_double(FIND_MODIFIER, move |_, _| Ret {
                    eax: existing,
                    ..Ret::default()
                });
                let (copied_at, other_at) = if target == 0x20 {
                    (existing + 0x2c, existing + 0x3c)
                } else {
                    (existing + 0x3c, existing + 0x2c)
                };
                r.e.mem.set_u32(other_at, other);
                let v = vector(&mut r.e, [1, 2, 3, copied]);
                r.e.call(0x00cd_f960, &args![r.first, side, r.second, v]);
                assert_eq!(
                    (r.e.mem.u32(copied_at), r.e.mem.u32(other_at)),
                    (copied_after, other_after),
                    "side {side}, lanes {copied:#x} and {other:#x}"
                );
                assert_eq!(words(&r.e, existing + target)[..3], [1, 2, 3]);
            }
        }
    }
}
