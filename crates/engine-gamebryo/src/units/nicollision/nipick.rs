//! `NiCollision/nipick.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPick::Results::Results` (`00e98db0`, `this` plus two stack words, `RET 8`).
const RESULTS_CONSTRUCTOR: u32 = 0x00e9_8db0;
/// The vtable stored at `+0x18` (RTTI class `NiPick::Results`).
const VTABLE_RESULTS: u32 = 0x010f_16ec;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00e98f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::NiPick` (engine map; the two stack words go to the results constructor
/// `00e98db0`, which is `NiPick::Results` at `+0x18`): sets the default state, clears the
/// root `NiPointer` and returns `this`.
///
/// Fields (Xbox PDB names, PC offsets as in the code): `m_ePickType` +0 = 0,
/// `m_eSortType` +4 = 0, `m_eIntersectType` +8 = 1, `m_eCoordinateType` +0xc = 1,
/// `m_bFrontOnly` +0x10 = 1, `m_bObserveAppCullFlag` +0x11 = 0, `m_spRoot` +0x14,
/// `m_pickResults` +0x18 (vtable `010f16ec`; its `m_uiValidResults` at +0x28 = 0),
/// `m_pLastAddedRecord` +0x2c = 0, `m_bReturnTexture` .. `m_bReturnColor` +0x30..+0x33 = 0.
/// The root is cleared before the results are constructed, and then released again when it
/// is non-null: decrement the count at `+4` and, when it reaches 0, call the object's
/// virtual at slot `+4`; then store null.
pub fn ni_pick_ni_pick(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 0x14, 0);
    e.call(RESULTS_CONSTRUCTOR, &args![base + 0x18, first, second]);
    e.mem.set_u32(base + 0x18, VTABLE_RESULTS);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u32(base + 0x8, 1);
    e.mem.set_u32(base + 0xc, 1);
    e.mem.set_u8(base + 0x10, 1);
    e.mem.set_u8(base + 0x11, 0);
    let root = e.mem.u32(base + 0x14);
    if root != 0 {
        if e.call(INTERLOCKED_DECREMENT, &args![root + 4]).u32() == 0 {
            e.vcall(root, 4, &args![]);
        }
        e.mem.set_u32(base + 0x14, 0);
    }
    e.mem.set_u8(base + 0x30, 0);
    e.mem.set_u8(base + 0x31, 0);
    e.mem.set_u8(base + 0x32, 0);
    e.mem.set_u8(base + 0x33, 0);
    e.mem.set_u32(base + 0x2c, 0);
    this
}

/// `NiTArray<NiPick::Record *, NiTMallocInterface<..>>` vtable (RTTI), stored first by the
/// array constructor and last by the results destructor.
const VTABLE_TARRAY: u32 = 0x010f_16dc;
/// `NiTPrimitiveArray<NiPick::Record *>` vtable (RTTI).
const VTABLE_PRIMITIVE_ARRAY: u32 = 0x010f_16e4;
/// The engine's `malloc` (`00aa1070`, cdecl: size in bytes).
const ALLOCATE: u32 = 0x00aa_1070;
/// The engine's `free` (`00aa10f0`, cdecl: the block).
const FREE: u32 = 0x00aa_10f0;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton at `011f6238`).
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The pick traversal (`00e9c490`, cdecl: the two words given to `PickObjects`, the pick and
/// its root), returns `bool`.
const PICK_TRAVERSE: u32 = 0x00e9_c490;
/// The other results sort (`00e98be0`, thiscall on the results), used when the pick type at
/// `+0` is not 0.
const RESULTS_SORT_OTHER: u32 = 0x00e9_8be0;

/// Releases one `NiPointer` reference to `object` (non-null): `InterlockedDecrement` of the
/// count at `+4` and, when it reaches 0, the object's virtual at slot `+4`.
fn release_reference(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One more element in the array's `m_usESize` (+0xc).
fn count_element(e: &mut Engine, base: u32) {
    let count = e.mem.u16(base + 0xc);
    e.mem.set_u16(base + 0xc, count.wrapping_add(1));
}

// Translated from 00e98b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the pick results (`this` is the `NiPick::Results` array) by `m_fDistance`
/// (`NiPick::Record` +0x14, Xbox PDB), nearest first: for each slot `i` it walks the slots
/// after it and swaps whenever the later record's distance is below the one now in slot `i`
/// (`FCOMPP` with C0/C3 tested: an equal or unordered pair is not swapped). The loops run to
/// `m_uiValidResults` (+0x10), read again at each step. Each store is the array's inlined
/// `SetAt`: an index at or past `m_usSize` (+0xa) raises it to index + 1 and counts one more
/// element in `m_usESize` (+0xc); an index below it counts one more element only when the
/// slot held null (for slot `i`, the slot is read again before the store).
pub fn fn_00e98b40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut outer = 0u32;
    if e.mem.u32(base + 0x10) <= outer {
        return;
    }
    loop {
        let mut current = e.mem.u32(e.mem.u32(base + 4) + outer * 4);
        let next = outer + 1;
        let mut inner = next;
        if inner < e.mem.u32(base + 0x10) {
            loop {
                let data = e.mem.u32(base + 4);
                let candidate = e.mem.u32(data + inner * 4);
                let candidate_distance = f32::from_bits(e.mem.u32(candidate + 0x14));
                let current_distance = f32::from_bits(e.mem.u32(current + 0x14));
                if current_distance > candidate_distance {
                    if inner >= u32::from(e.mem.u16(base + 0xa)) {
                        e.mem.set_u16(base + 0xa, (inner + 1) as u16);
                        count_element(e, base);
                    } else if candidate == 0 {
                        count_element(e, base);
                    }
                    e.mem.set_u32(data + inner * 4, current);
                    if outer >= u32::from(e.mem.u16(base + 0xa)) {
                        e.mem.set_u16(base + 0xa, (outer + 1) as u16);
                        count_element(e, base);
                    } else {
                        let data = e.mem.u32(base + 4);
                        if e.mem.u32(data + outer * 4) == 0 {
                            count_element(e, base);
                        }
                    }
                    let data = e.mem.u32(base + 4);
                    e.mem.set_u32(data + outer * 4, candidate);
                    current = candidate;
                }
                inner += 1;
                if inner >= e.mem.u32(base + 0x10) {
                    break;
                }
            }
        }
        outer = next;
        if outer >= e.mem.u32(base + 0x10) {
            break;
        }
    }
}

// Translated from 00e98db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiPick::Record *>` constructor (vtables by RTTI; `NiPick::Results`
/// derives from it): `m_usMaxSize` (+8) = `max_size`, `m_usGrowBy` (+0xe) = `grow_by`,
/// `m_usSize` and `m_usESize` = 0, and `m_pBase` (+4) a block of `max_size` pointers from
/// `00aa1070`, or null when `max_size` is 0. Returns `this`.
pub fn ni_t_primitive_array_record_constructor(
    e: &mut Engine,
    this: Ptr,
    max_size: u16,
    grow_by: u16,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 0xe, grow_by);
    e.mem.set_u32(base, VTABLE_TARRAY);
    e.mem.set_u16(base + 8, max_size);
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    if max_size != 0 {
        let block = e.call(ALLOCATE, &args![u32::from(max_size) * 4]).u32();
        e.mem.set_u32(base + 4, block);
    } else {
        e.mem.set_u32(base + 4, 0);
    }
    e.mem.set_u32(base, VTABLE_PRIMITIVE_ARRAY);
    this
}

// Translated from 00e98e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::PickObjects` (engine map): unless `append` is set, clears the valid-result count
/// (`m_pickResults.m_uiValidResults`, +0x28); runs the pick (`00e9c490` with the two words,
/// the pick and its root at +0x14). When that succeeds with results, they are sorted if the
/// sort type (+4) is not 1, the intersect type (+8) is not 0 and there is more than one: by
/// `00e98b40` when the pick type (+0) is 0, otherwise by `00e98be0`. Returns whether the pick
/// succeeded with at least one result.
pub fn ni_pick_pick_objects(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    append: bool,
) -> bool {
    let base = this.addr();
    if !append {
        e.mem.set_u32(base + 0x28, 0);
    }
    let root = e.mem.u32(base + 0x14);
    let picked = e
        .call(PICK_TRAVERSE, &args![first, second, base, root])
        .bool();
    if !picked {
        return false;
    }
    let valid = e.mem.u32(base + 0x28);
    if valid == 0 {
        return false;
    }
    if e.mem.u32(base + 4) != 1 && e.mem.u32(base + 8) != 0 && valid > 1 {
        if e.mem.u32(base) == 0 {
            fn_00e98b40(e, Ptr::new(base + 0x18));
            return true;
        }
        e.call(RESULTS_SORT_OTHER, &args![base + 0x18]);
    }
    true
}

// Translated from 00e98e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::Results::~Results` (engine map): sets the results vtable, then for every slot
/// below `m_usSize` (+0xa) that holds a record releases its `m_spProxyParent` (+4) and
/// `m_spObject` (+0) references and frees the record through the memory manager
/// (`00aa4060`); finally sets the `NiTArray` vtable and frees `m_pBase` (`00aa10f0`).
pub fn ni_pick_results_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_RESULTS);
    let mut index = 0u32;
    if e.mem.u16(base + 0xa) != 0 {
        loop {
            let record = e.mem.u32(e.mem.u32(base + 4) + index * 4);
            if record != 0 {
                let proxy_parent = e.mem.u32(record + 4);
                if proxy_parent != 0 {
                    release_reference(e, proxy_parent);
                }
                let object = e.mem.u32(record);
                if object != 0 {
                    release_reference(e, object);
                }
                e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, record]);
            }
            index += 1;
            if index >= u32::from(e.mem.u16(base + 0xa)) {
                break;
            }
        }
    }
    let data = e.mem.u32(base + 4);
    e.mem.set_u32(base, VTABLE_TARRAY);
    e.call(FREE, &args![data]);
}

// Translated from 00e98fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::~NiPick` (engine map): destroys the results at +0x18 (`00e98e90`) and releases
/// the root `NiPointer` at +0x14 (the final virtual call is a tail jump).
pub fn ni_pick_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    ni_pick_results_destructor(e, Ptr::new(base + 0x18));
    let root = e.mem.u32(base + 0x14);
    if root != 0 {
        release_reference(e, root);
    }
}

// Translated from 00e98be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Keeps only the nearest pick result (`this` is the `NiPick::Results` array; the other
/// "sort" `NiPick::PickObjects` uses when the pick type is not 0). With no valid results
/// (`m_uiValidResults`, +0x10) nothing happens. Otherwise it finds the record with the
/// smallest `m_fDistance` (+0x14): a later record replaces the best one only when the best
/// one's distance is greater (`FCOMPP` with C0/C3 tested: equal or unordered keeps it). The
/// first record goes to the best one's slot through `SetAt` (`00ba3aa0`, thiscall: index,
/// address of the pointer), and the best record is stored in slot 0 by the inlined `SetAt`:
/// with `m_usSize` (+0xa) 0 the size becomes 1 and a non-null record counts one more element
/// in `m_usESize` (+0xc); otherwise a non-null record over a null slot counts one more, a
/// null record over a non-null slot one less. The valid count becomes 1.
pub fn fn_00e98be0(e: &mut Engine, this: Ptr) {
    const SET_AT: u32 = 0x00ba_3aa0;
    let base = this.addr();
    let valid = e.mem.u32(base + 0x10);
    if valid == 0 {
        return;
    }
    let data = e.mem.u32(base + 4);
    let first = e.mem.u32(data);
    let mut best = first;
    let mut best_index = 0u32;
    let mut index = 1u32;
    if valid > index {
        loop {
            let candidate = e.mem.u32(data.wrapping_add(index * 4));
            let candidate_distance = f32::from_bits(e.mem.u32(candidate.wrapping_add(0x14)));
            let best_distance = f32::from_bits(e.mem.u32(best.wrapping_add(0x14)));
            if best_distance > candidate_distance {
                best = candidate;
                best_index = index;
            }
            index += 1;
            if index >= e.mem.u32(base + 0x10) {
                break;
            }
        }
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), first);
        e.call(SET_AT, &args![base, best_index, slot]);
    });
    if e.mem.u16(base + 0xa) == 0 {
        e.mem.set_u16(base + 0xa, 1);
        if best != 0 {
            count_element(e, base);
        }
    } else if best != 0 {
        let data = e.mem.u32(base + 4);
        if e.mem.u32(data) == 0 {
            count_element(e, base);
        }
    } else {
        let data = e.mem.u32(base + 4);
        if e.mem.u32(data) != 0 {
            let count = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, count.wrapping_add(0xffff));
        }
    }
    let data = e.mem.u32(base + 4);
    e.mem.set_u32(data, best);
    e.mem.set_u32(base + 0x10, 1);
}

// Translated from 00e98fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPick::Results::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// (`00e98e90`) and, when bit 0 of `flags` is set, frees `this` through the memory manager
/// (`00aa4060`). Answers `this`.
pub fn ni_pick_results_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    ni_pick_results_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_8be0, fn_00e98be0(Ptr)),
        entry!(
            0x00e9_8fd0,
            ni_pick_results_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e9_8f20, ni_pick_ni_pick(Ptr, u32, u32) -> Ptr),
        entry!(0x00e9_8b40, fn_00e98b40(Ptr)),
        entry!(
            0x00e9_8db0,
            ni_t_primitive_array_record_constructor(Ptr, u16, u16) -> Ptr
        ),
        entry!(0x00e9_8e20, ni_pick_pick_objects(Ptr, u32, u32, bool) -> bool),
        entry!(0x00e9_8e90, ni_pick_results_destructor(Ptr)),
        entry!(0x00e9_8fa0, ni_pick_destructor(Ptr)),
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

    fn pick(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        for i in 0..0x10u32 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        this
    }

    fn check_fields(e: &Engine, this: u32) {
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 1);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u8(this + 0x11), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), VTABLE_RESULTS);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        // The results constructor's own words (+0x1c to +0x27) are left alone.
        assert_eq!(e.mem.u32(this + 0x1c), 0xffff_ffff);
    }

    #[test]
    fn sets_the_defaults_and_passes_the_words_to_the_results_constructor() {
        let mut e = Engine::new();
        e.register(RESULTS_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |_, _| panic!("no root to release"));
        let this = pick(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_8f20, &args![this, 0u32, 8u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, RESULTS_CONSTRUCTOR),
            vec![vec![this + 0x18, 0, 8]]
        );
        check_fields(&e, this);
    }

    #[test]
    fn releases_a_root_the_results_constructor_left_and_calls_its_destructor_at_zero() {
        for (count, destroyed) in [(0u32, true), (3u32, false)] {
            let mut e = Engine::new();
            let root = e.mem.alloc(0x10);
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(root, vtable);
            e.mem.set_u32(vtable + 4, 0x00aa_0001);
            e.register_double(RESULTS_CONSTRUCTOR, move |e, a| {
                // `a[0] - 4` is the root pointer at +0x14.
                e.mem.set_u32(a[0] - 4, root);
                Ret::default()
            });
            e.register_double(INTERLOCKED_DECREMENT, move |_, _| Ret {
                eax: count,
                ..Ret::default()
            });
            e.register(0x00aa_0001, |_, _| Ret::default());
            let this = pick(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x00e9_8f20, &args![this, 0u32, 8u32]);
            assert_eq!(
                calls_to(&mut e, INTERLOCKED_DECREMENT),
                vec![vec![root + 4]]
            );
            assert_eq!(
                calls_to(&mut e, 0x00aa_0001),
                if destroyed { vec![vec![root]] } else { vec![] }
            );
            check_fields(&e, this);
        }
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A results array of `distances.len()` records with those distances; `m_usSize` and
    /// `m_usESize` as given.
    fn results(e: &mut Engine, distances: &[f32], size: u16, esize: u16) -> (u32, Vec<u32>) {
        let base = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(base + 4, data);
        e.mem.set_u16(base + 0xa, size);
        e.mem.set_u16(base + 0xc, esize);
        e.mem.set_u32(base + 0x10, distances.len() as u32);
        let mut records = vec![];
        for (i, d) in distances.iter().enumerate() {
            let record = e.mem.alloc(0x44);
            e.mem.set_u32(record + 0x14, d.to_bits());
            e.mem.set_u32(data + i as u32 * 4, record);
            records.push(record);
        }
        (base, records)
    }

    fn slots(e: &Engine, base: u32, n: u32) -> Vec<u32> {
        let data = e.mem.u32(base + 4);
        (0..n).map(|i| e.mem.u32(data + i * 4)).collect()
    }

    #[test]
    fn sort_orders_by_distance_and_leaves_counts_inside_the_size() {
        let mut e = Engine::new();
        let (base, r) = results(&mut e, &[3.0, 1.0, 2.0], 3, 3);
        e.call(0x00e9_8b40, &args![base]);
        assert_eq!(slots(&e, base, 3), vec![r[1], r[2], r[0]]);
        assert_eq!(e.mem.u16(base + 0xa), 3);
        assert_eq!(e.mem.u16(base + 0xc), 3);
    }

    #[test]
    fn sort_keeps_equal_and_nan_pairs_and_grows_the_size_past_it() {
        let mut e = Engine::new();
        let (base, r) = results(&mut e, &[1.0, 1.0, f32::NAN], 3, 3);
        e.call(0x00e9_8b40, &args![base]);
        assert_eq!(slots(&e, base, 3), r);
        // A swap at index 1, past m_usSize (0), raises it to 2 and counts one element; slot 0
        // is then below the size and held a record, so it is not counted again.
        let (base, r) = results(&mut e, &[2.0, 1.0], 0, 0);
        e.call(0x00e9_8b40, &args![base]);
        assert_eq!(slots(&e, base, 2), vec![r[1], r[0]]);
        assert_eq!(e.mem.u16(base + 0xa), 2);
        assert_eq!(e.mem.u16(base + 0xc), 1);
        // No valid results: nothing is read.
        let (base, _) = results(&mut e, &[], 0, 0);
        e.call(0x00e9_8b40, &args![base]);
    }

    #[test]
    fn array_constructor_allocates_max_size_pointers() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| Ret {
            eax: 0x5000 + a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let r = e.call(0x00e9_8db0, &args![this, 0xffff_0008u32, 0x1234_0004u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x20]]);
        assert_eq!(e.mem.u32(this), VTABLE_PRIMITIVE_ARRAY);
        assert_eq!(e.mem.u32(this + 4), 0x5020);
        assert_eq!(e.mem.u16(this + 8), 8);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u16(this + 0xe), 4);
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 4, 0xdead);
        e.call(0x00e9_8db0, &args![this, 0u32, 1u32]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
    }

    fn pick_with(e: &mut Engine, pick_type: u32, sort: u32, intersect: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, pick_type);
        e.mem.set_u32(this + 4, sort);
        e.mem.set_u32(this + 8, intersect);
        e.mem.set_u32(this + 0x14, 0x7700);
        this
    }

    #[test]
    fn pick_objects_runs_the_pick_and_sorts_by_type() {
        type Case = (u32, u32, u32, u32, bool, bool);
        // (pick type, sort, intersect, valid, traversal result, sorted by 00e98be0)
        let cases: [Case; 6] = [
            (1, 0, 1, 2, true, true),
            (1, 1, 1, 2, true, false),
            (1, 0, 0, 2, true, false),
            (1, 0, 1, 1, true, false),
            (1, 0, 1, 2, false, false),
            (1, 0, 1, 0, true, false),
        ];
        for (pick_type, sort, intersect, valid, ok, other) in cases {
            let mut e = Engine::new();
            e.register_double(PICK_TRAVERSE, move |e, a| {
                e.mem.set_u32(a[2] + 0x28, valid);
                Ret {
                    eax: ok as u32,
                    ..Ret::default()
                }
            });
            e.register(RESULTS_SORT_OTHER, |_, _| Ret::default());
            let this = pick_with(&mut e, pick_type, sort, intersect);
            e.call_log = Some(vec![]);
            let r = e.call(0x00e9_8e20, &args![this, 0x11u32, 0x22u32, 0u32]);
            assert_eq!(r.bool(), ok && valid != 0);
            assert_eq!(
                calls_to(&e, PICK_TRAVERSE),
                vec![vec![0x11, 0x22, this, 0x7700]]
            );
            assert_eq!(
                calls_to(&e, RESULTS_SORT_OTHER),
                if other {
                    vec![vec![this + 0x18]]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn pick_objects_keeps_the_count_when_appending_and_sorts_by_distance_for_type_0() {
        let mut e = Engine::new();
        e.register(PICK_TRAVERSE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(RESULTS_SORT_OTHER, |_, _| {
            panic!("type 0 sorts by distance")
        });
        let this = pick_with(&mut e, 0, 0, 1);
        let (_, r) = {
            let data = e.mem.alloc(0x10);
            let near = e.mem.alloc(0x44);
            let far = e.mem.alloc(0x44);
            e.mem.set_u32(near + 0x14, 1f32.to_bits());
            e.mem.set_u32(far + 0x14, 5f32.to_bits());
            e.mem.set_u32(data, far);
            e.mem.set_u32(data + 4, near);
            e.mem.set_u32(this + 0x1c, data);
            e.mem.set_u16(this + 0x22, 2);
            e.mem.set_u16(this + 0x24, 2);
            (data, vec![near, far])
        };
        e.mem.set_u32(this + 0x28, 2);
        let ok = e.call(0x00e9_8e20, &args![this, 0u32, 0u32, 1u32]).bool();
        assert!(ok);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert_eq!(slots(&e, this + 0x18, 2), r);
    }

    #[test]
    fn results_destructor_releases_each_record_and_frees_the_array() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], n);
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        e.register(0x00aa_0002, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x00aa_0002);
        let object = |e: &mut Engine, count: u32| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, vtable);
            e.mem.set_u32(o + 4, count);
            o
        };
        let shared = object(&mut e, 1);
        let kept = object(&mut e, 2);
        let record = e.mem.alloc(0x44);
        e.mem.set_u32(record, kept);
        e.mem.set_u32(record + 4, shared);
        let bare = e.mem.alloc(0x44);
        let this = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(data, record);
        e.mem.set_u32(data + 8, bare);
        e.mem.set_u16(this + 0xa, 3);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8e90, &args![this]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![shared + 4], vec![kept + 4]]
        );
        assert_eq!(calls_to(&e, 0x00aa_0002), vec![vec![shared]]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, record], vec![MEMORY_MANAGER, bare]]
        );
        assert_eq!(calls_to(&e, FREE), vec![vec![data]]);
        assert_eq!(e.mem.u32(this), VTABLE_TARRAY);
        // An empty array frees only the block.
        e.mem.set_u16(this + 0xa, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8e90, &args![this]);
        assert!(calls_to(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, FREE), vec![vec![data]]);
    }

    #[test]
    fn pick_destructor_destroys_the_results_and_releases_the_root() {
        for (root_count, destroyed) in [(1u32, true), (3, false), (0, false)] {
            let mut e = Engine::new();
            e.register(INTERLOCKED_DECREMENT, |e, a| {
                let n = e.mem.u32(a[0]) - 1;
                e.mem.set_u32(a[0], n);
                Ret {
                    eax: n,
                    ..Ret::default()
                }
            });
            e.register(0x00aa_0003, |_, _| Ret::default());
            e.register(FREE, |_, _| Ret::default());
            let this = e.mem.alloc(0x34);
            let root = if root_count == 0 {
                0
            } else {
                let vtable = e.mem.alloc(8);
                e.mem.set_u32(vtable + 4, 0x00aa_0003);
                let root = e.mem.alloc(8);
                e.mem.set_u32(root, vtable);
                e.mem.set_u32(root + 4, root_count);
                root
            };
            e.mem.set_u32(this + 0x14, root);
            e.mem.set_u32(this + 0x1c, 0x4444);
            e.call_log = Some(vec![]);
            e.call(0x00e9_8fa0, &args![this]);
            assert_eq!(calls_to(&e, FREE), vec![vec![0x4444]]);
            assert_eq!(e.mem.u32(this + 0x18), VTABLE_TARRAY);
            assert_eq!(
                calls_to(&e, 0x00aa_0003),
                if destroyed { vec![vec![root]] } else { vec![] }
            );
        }
    }
}

#[cfg(test)]
mod n034_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A results array with records of those distances (`m_usSize`, `m_usESize` given).
    fn results(e: &mut Engine, distances: &[f32], size: u16, esize: u16) -> (u32, Vec<u32>) {
        let base = e.mem.alloc(0x14);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(base + 4, data);
        e.mem.set_u16(base + 0xa, size);
        e.mem.set_u16(base + 0xc, esize);
        e.mem.set_u32(base + 0x10, distances.len() as u32);
        let mut records = vec![];
        for (i, d) in distances.iter().enumerate() {
            let record = e.mem.alloc(0x44);
            e.mem.set_u32(record + 0x14, d.to_bits());
            e.mem.set_u32(data + i as u32 * 4, record);
            records.push(record);
        }
        (base, records)
    }

    #[test]
    fn keeps_the_nearest_result_in_slot_zero() {
        let mut e = Engine::new();
        e.register(0x00ba_3aa0, |e, a| {
            let data = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(data + a[1] * 4, value);
            Ret::default()
        });
        let (base, r) = results(&mut e, &[3.0, 1.0, 1.0, f32::NAN, 2.0], 5, 5);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8be0, &args![base]);
        // The first of the equal nearest records wins; the first record goes to its slot.
        let set_at = calls_to(&e, 0x00ba_3aa0);
        assert_eq!(set_at.len(), 1);
        assert_eq!(&set_at[0][..2], &[base, 1]);
        let data = e.mem.u32(base + 4);
        assert_eq!(e.mem.u32(data), r[1]);
        assert_eq!(e.mem.u32(data + 4), r[0]);
        assert_eq!(e.mem.u32(base + 0x10), 1);
        // Slot 0 held a record: the element count stays.
        assert_eq!(e.mem.u16(base + 0xc), 5);
    }

    #[test]
    fn counts_elements_like_set_at() {
        let mut e = Engine::new();
        e.register(0x00ba_3aa0, |_, _| Ret::default());
        // Size 0: becomes 1, a non-null record counts one more.
        let (base, r) = results(&mut e, &[2.0], 0, 4);
        e.call(0x00e9_8be0, &args![base]);
        assert_eq!(e.mem.u16(base + 0xa), 1);
        assert_eq!(e.mem.u16(base + 0xc), 5);
        assert_eq!(e.mem.u32(e.mem.u32(base + 4)), r[0]);
        // A null best record over a non-null slot counts one less.
        let (base, _) = results(&mut e, &[2.0], 1, 4);
        let data = e.mem.u32(base + 4);
        e.mem.set_u32(data, 0);
        e.register(0x00ba_3aa0, |e, a| {
            let data = e.mem.u32(a[0] + 4);
            e.mem.set_u32(data, 0x7777);
            Ret::default()
        });
        e.call(0x00e9_8be0, &args![base]);
        assert_eq!(e.mem.u16(base + 0xc), 3);
        assert_eq!(e.mem.u32(data), 0);
        // No valid results: nothing is touched.
        let (base, _) = results(&mut e, &[], 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_8be0, &args![base]);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(FREE, |_, _| Ret::default());
            e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x14);
            e.mem.set_u32(this + 4, 0x4444);
            e.call_log = Some(vec![]);
            let r = e.call(0x00e9_8fd0, &args![this, flags]).u32();
            assert_eq!(r, this);
            assert_eq!(calls_to(&e, FREE), vec![vec![0x4444]]);
            assert_eq!(
                calls_to(&e, MEMORY_MANAGER_DEALLOCATE),
                if flags == 1 {
                    vec![vec![MEMORY_MANAGER, this]]
                } else {
                    vec![]
                }
            );
        }
    }
}
