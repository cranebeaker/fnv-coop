//! `(unplaced)/run_00b03b30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager singleton and its deallocate (`00aa4060`, `thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;
/// Import slots of `EnterCriticalSection` and `LeaveCriticalSection` (`stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// The critical section guarding the global instance list (`011f8bc4`).
const INSTANCE_LIST_LOCK: u32 = 0x011f_8bc4;
/// The global instance list (`011f8bdc`): an array of pointers whose first and one-past-last
/// element pointers are the words at +0xc (`011f8be8`) and +0x10 (`011f8bec`).
const INSTANCE_LIST: u32 = 0x011f_8bdc;
/// `00b10430` (`thiscall` on an 8-byte iterator, `RET 8`: element pointer and container):
/// builds an iterator.
const MAKE_ITERATOR: u32 = 0x00b1_0430;
/// `00b0eca0` (`thiscall` on an iterator, `RET 4`: the other iterator): iterator equality
/// (low byte of `EAX`).
const ITERATORS_EQUAL: u32 = 0x00b0_eca0;
/// `00b0df00` (`thiscall` on the container, `RET 0xc`: the address of the result iterator
/// and the erased position's iterator by value, two words): erases one element.
const ERASE: u32 = 0x00b0_df00;
/// `CSpeedTreeRT::DeleteTransientData` (`00b053e0`, `thiscall`).
const DELETE_TRANSIENT_DATA: u32 = 0x00b0_53e0;
/// The member destructors (`thiscall`, no stack word) run before freeing the members.
const MEMBER_DESTRUCTOR_00B2E9C0: u32 = 0x00b2_e9c0;
const MEMBER_DESTRUCTOR_00B276F0: u32 = 0x00b2_76f0;
const MEMBER_DESTRUCTOR_00B0E7F0: u32 = 0x00b0_e7f0;
const MEMBER_DESTRUCTOR_00B0EA80: u32 = 0x00b0_ea80;
const MEMBER_DESTRUCTOR_00B181B0: u32 = 0x00b1_81b0;
/// `0044a730` (`thiscall`, `RET 8`; unit `fallout shared/tesobject.cpp`): run with (1, 0) on
/// +0x24 of the member at +0x50.
const MEMBER_CLEAR_0044A730: u32 = 0x0044_a730;
/// The live instance count (`011f8b7c`) and the call made when it reaches 0 (`00b37a00`,
/// unit `SpeedTree/idvspline.cpp`).
const INSTANCE_COUNT: u32 = 0x011f_8b7c;
const LAST_INSTANCE_GONE: u32 = 0x00b3_7a00;

/// Gives a block back to the memory manager.
fn deallocate(e: &mut Engine, block: u32) {
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// The compiler kept `if (1 & 1)` around the frees of the members' deleting destructors
/// (`MOV ECX,1; AND ECX,1; JZ`): always true.
fn folded_flag() -> bool {
    std::hint::black_box(1u32) & 1 != 0
}

/// Searches the pointer array between the iterators the exe builds at `end_at` (from the
/// container's one-past-last word) and `begin_at` (from its first word) for `this`: returns
/// the begin iterator's first word and the element pointer where the search stopped.
fn find_in_list(e: &mut Engine, end_at: u32, begin_at: u32, this: u32) -> (u32, u32) {
    let end = e.mem.u32(end_at + 4);
    let container_word = e.mem.u32(begin_at);
    let mut cursor = e.mem.u32(begin_at + 4);
    while cursor != end && e.mem.u32(cursor) != this {
        cursor = cursor.wrapping_add(4);
    }
    (container_word, cursor)
}

// Translated from 00b03b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b03b30 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f23adc, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word; it calls
/// `CSpeedTreeRT::DeleteTransientData`): the tear-down of a tree instance.
///
/// Without a private list (+0x34 null), removes `this` from the global instance list
/// `011f8bdc` inside the critical section `011f8bc4`: the element is searched from the first
/// to the last pointer and erased (`00b0df00`) only when the found iterator differs from the
/// end iterator (`00b0eca0`). Then decrements the shared reference count the word at +0x30
/// points to. With a private list, removes `this` from the list +0x38 points to the same way
/// (erased without the end test) and frees the private list block, clearing +0x34.
///
/// When the shared count reaches 0, frees the shared data: `00b053e0`, then the members at
/// +4 (`00b2e9c0` first), +0xc, +0x10, +8 (`00b276f0` first), +0x14, the object at +0
/// (its vtable slot 0 with 1), +0x30, +0x38 (`00b0e7f0`, then its first word and itself),
/// +0x40, +0x58 (`00b0ea80`, then its first word and itself), +0x4c ([`fn_00b04420`]), +0x5c
/// (`00b181b0`), +0x60 (`00b2e9c0`), +0x2c, +0x50 (`0044a730(1, 0)` on its +0x24) and +0x68,
/// each optional member only when set. In every case the sixteen member words are cleared
/// and the live instance count `011f8b7c` goes down, calling `00b37a00` when it reaches 0.
pub fn fn_00b03b30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The frame below `EBP`: offsets are `0x80 - k` for the exe's `[EBP - k]`.
    e.with_stack(0x80, |e, frame| {
        let at = |k: u32| frame.addr() + 0x80 - k;
        if e.mem.u32(base + 0x34) == 0 {
            e.call(ENTER_CRITICAL_SECTION, &args![INSTANCE_LIST_LOCK]);
            let end_word = e.mem.u32(INSTANCE_LIST + 0x10);
            e.call(MAKE_ITERATOR, &args![at(0x28), end_word, INSTANCE_LIST]);
            let begin_word = e.mem.u32(INSTANCE_LIST + 0xc);
            e.call(MAKE_ITERATOR, &args![at(0x30), begin_word, INSTANCE_LIST]);
            let (container_word, cursor) = find_in_list(e, at(0x28), at(0x30), base);
            e.mem.set_u32(at(0x14), container_word);
            e.mem.set_u32(at(0x10), cursor);
            let end_word = e.mem.u32(INSTANCE_LIST + 0x10);
            e.call(MAKE_ITERATOR, &args![at(0x38), end_word, INSTANCE_LIST]);
            let equal = e.call(ITERATORS_EQUAL, &args![at(0x14), at(0x38)]).u8();
            if equal == 0 {
                let first = e.mem.u32(at(0x14));
                let second = e.mem.u32(at(0x10));
                e.mem.set_u32(at(0x40), first);
                e.mem.set_u32(at(0x3c), second);
                e.call(ERASE, &args![INSTANCE_LIST, at(0x48), first, second]);
            }
            e.call(LEAVE_CRITICAL_SECTION, &args![INSTANCE_LIST_LOCK]);
        }
        let shared_count = e.mem.u32(base + 0x30);
        let count = e.mem.u32(shared_count).wrapping_sub(1);
        e.mem.set_u32(shared_count, count);
        if e.mem.u32(base + 0x34) != 0 {
            let list = e.mem.u32(base + 0x38);
            let end_word = e.mem.u32(list + 0x10);
            e.call(MAKE_ITERATOR, &args![at(0x54), end_word, list]);
            let list = e.mem.u32(base + 0x38);
            let begin_word = e.mem.u32(list + 0xc);
            e.call(MAKE_ITERATOR, &args![at(0x5c), begin_word, list]);
            let (container_word, cursor) = find_in_list(e, at(0x54), at(0x5c), base);
            e.mem.set_u32(at(0x1c), container_word);
            e.mem.set_u32(at(0x18), cursor);
            e.mem.set_u32(at(0x64), container_word);
            e.mem.set_u32(at(0x60), cursor);
            let list = e.mem.u32(base + 0x38);
            e.call(ERASE, &args![list, at(0x6c), container_word, cursor]);
            let private = e.mem.u32(base + 0x34);
            deallocate(e, private);
            e.mem.set_u32(base + 0x34, 0);
        }
    });
    let shared_count = e.mem.u32(base + 0x30);
    if e.mem.u32(shared_count) == 0 {
        e.call(DELETE_TRANSIENT_DATA, &args![this]);
        let member = e.mem.u32(base + 4);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B2E9C0, &args![member]);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        for offset in [0xcu32, 0x10] {
            let block = e.mem.u32(base + offset);
            deallocate(e, block);
        }
        let member = e.mem.u32(base + 8);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B276F0, &args![member]);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let block = e.mem.u32(base + 0x14);
        deallocate(e, block);
        let object = e.mem.u32(base);
        if object != 0 {
            e.vcall(object, 0, &args![1u32]);
        }
        let block = e.mem.u32(base + 0x30);
        deallocate(e, block);
        let member = e.mem.u32(base + 0x38);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B0E7F0, &args![member]);
            let storage = e.mem.u32(member);
            deallocate(e, storage);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let block = e.mem.u32(base + 0x40);
        deallocate(e, block);
        let member = e.mem.u32(base + 0x58);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B0EA80, &args![member]);
            let storage = e.mem.u32(member);
            deallocate(e, storage);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let member = e.mem.u32(base + 0x4c);
        if member != 0 {
            fn_00b04420(e, Ptr::new(member));
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let member = e.mem.u32(base + 0x5c);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B181B0, &args![member]);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let member = e.mem.u32(base + 0x60);
        if member != 0 {
            e.call(MEMBER_DESTRUCTOR_00B2E9C0, &args![member]);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let block = e.mem.u32(base + 0x2c);
        deallocate(e, block);
        let member = e.mem.u32(base + 0x50);
        if member != 0 {
            e.call(MEMBER_CLEAR_0044A730, &args![member + 0x24, 1u32, 0u32]);
            if folded_flag() {
                deallocate(e, member);
            }
        }
        let block = e.mem.u32(base + 0x68);
        deallocate(e, block);
    }
    for offset in [
        4u32, 0xc, 0x10, 8, 0x14, 0, 0x30, 0x40, 0x38, 0x58, 0x4c, 0x5c, 0x60, 0x2c, 0x50, 0x68,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    let instances = e.mem.u32(INSTANCE_COUNT).wrapping_sub(1);
    e.mem.set_u32(INSTANCE_COUNT, instances);
    if instances == 0 {
        e.call(LAST_INSTANCE_GONE, &args![]);
    }
}

// Translated from 00b04420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): frees the blocks at +4, +0xc and
/// +0x14 (each through the memory manager, also when null, and cleared), then runs
/// `0044a730(1, 0)` on +0x18.
pub fn fn_00b04420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [4u32, 0xc, 0x14] {
        let block = e.mem.u32(base + offset);
        deallocate(e, block);
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMBER_CLEAR_0044A730, &args![base + 0x18, 1u32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0_3b30, fn_00b03b30(Ptr)),
        entry!(0x00b0_4420, fn_00b04420(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// An engine with every callee doubled; the iterator maker stores (container, element).
    fn fixture(equal: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_8000, 0x1000);
        e.mem.set_u32(INSTANCE_COUNT, 2);
        e.register(MAKE_ITERATOR, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register_double(ITERATORS_EQUAL, move |_, _| equal.into_ret());
        for callee in [
            ENTER_CRITICAL_SECTION,
            LEAVE_CRITICAL_SECTION,
            ERASE,
            DEALLOCATE,
            DELETE_TRANSIENT_DATA,
            MEMBER_DESTRUCTOR_00B2E9C0,
            MEMBER_DESTRUCTOR_00B276F0,
            MEMBER_DESTRUCTOR_00B0E7F0,
            MEMBER_DESTRUCTOR_00B0EA80,
            MEMBER_DESTRUCTOR_00B181B0,
            MEMBER_CLEAR_0044A730,
            LAST_INSTANCE_GONE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    /// A tree with the shared count at `count`; the global list holds 0x10, `this`, 0x30.
    fn tree(e: &mut Engine, count: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x70);
        let shared = e.mem.alloc(4);
        e.mem.set_u32(shared, count);
        e.mem.set_u32(this + 0x30, shared);
        let elements = e.mem.alloc(0xc);
        for (index, value) in [0x10u32, this, 0x30].into_iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, value);
        }
        e.mem.set_u32(INSTANCE_LIST + 0xc, elements);
        e.mem.set_u32(INSTANCE_LIST + 0x10, elements + 0xc);
        (this, elements)
    }

    #[test]
    fn a_shared_tree_leaves_the_global_list_and_frees_everything_last() {
        let mut e = fixture(0);
        let (this, elements) = tree(&mut e, 1);
        let shared = e.mem.u32(this + 0x30);
        let renderer = e.mem.alloc(4);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        e.mem.set_u32(renderer, 0x0150_0000);
        e.mem.set_u32(this, renderer);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x3800);
        e.mem.set_u32(this + 0x38, list);
        let indices = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x4c, indices);
        e.mem.set_u32(indices + 4, 0x4c04);
        e.mem.set_u32(this + 0x50, 0x5000);
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call_log = Some(vec![]);
        e.call(0x00b0_3b30, &args![this]);
        let log = e.call_log.take().unwrap();
        let erased = calls_to(&log, ERASE);
        assert_eq!(erased.len(), 1);
        assert_eq!(erased[0][0], INSTANCE_LIST);
        assert_eq!(erased[0][2..], [INSTANCE_LIST, elements + 4]);
        assert_eq!(
            calls_to(&log, ENTER_CRITICAL_SECTION),
            vec![vec![INSTANCE_LIST_LOCK]]
        );
        assert_eq!(calls_to(&log, DELETE_TRANSIENT_DATA), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0150_1000), vec![vec![renderer, 1]]);
        assert_eq!(calls_to(&log, MEMBER_DESTRUCTOR_00B0E7F0), vec![vec![list]]);
        assert_eq!(
            calls_to(&log, MEMBER_CLEAR_0044A730),
            vec![vec![indices + 0x18, 1, 0], vec![0x5024, 1, 0]]
        );
        let freed: Vec<u32> = calls_to(&log, DEALLOCATE).iter().map(|a| a[1]).collect();
        for block in [shared, 0x3800, list, 0x4c04, indices, 0x5000, 0x6800] {
            assert!(freed.contains(&block), "{block:#x} not freed");
        }
        assert!(calls_to(&log, MEMBER_DESTRUCTOR_00B2E9C0).is_empty());
        for offset in [0u32, 0x30, 0x38, 0x4c, 0x50, 0x68] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(INSTANCE_COUNT), 1);
        assert!(calls_to(&log, LAST_INSTANCE_GONE).is_empty());
    }

    #[test]
    fn a_missing_tree_is_not_erased_and_the_shared_data_stays() {
        let mut e = fixture(1);
        let (this, _) = tree(&mut e, 3);
        e.mem.set_u32(INSTANCE_COUNT, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b0_3b30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ERASE).is_empty());
        assert!(calls_to(&log, DELETE_TRANSIENT_DATA).is_empty());
        assert_eq!(calls_to(&log, LAST_INSTANCE_GONE).len(), 1);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn a_private_list_is_left_without_the_end_test_and_freed() {
        let mut e = fixture(1);
        let (this, _) = tree(&mut e, 3);
        let private = e.mem.alloc(0x14);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements, this);
        e.mem.set_u32(private + 0xc, elements);
        e.mem.set_u32(private + 0x10, elements + 8);
        e.mem.set_u32(this + 0x34, 0x3400);
        e.mem.set_u32(this + 0x38, private);
        e.call_log = Some(vec![]);
        e.call(0x00b0_3b30, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, ENTER_CRITICAL_SECTION).is_empty());
        let erased = calls_to(&log, ERASE);
        assert_eq!(erased.len(), 1);
        assert_eq!(erased[0][0], private);
        assert_eq!(erased[0][2..], [private, elements]);
        assert_eq!(
            calls_to(&log, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x3400]]
        );
        assert_eq!(e.mem.u32(this + 0x34), 0);
    }

    #[test]
    fn fn_00b04420_frees_three_blocks_and_clears_the_rest() {
        let mut e = fixture(0);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, 0x4000);
        e.mem.set_u32(this + 0xc, 0xc000);
        e.call_log = Some(vec![]);
        e.call(0x00b0_4420, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (DEALLOCATE, vec![MEMORY_MANAGER, 0x4000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0xc000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, 0]),
                (MEMBER_CLEAR_0044A730, vec![this + 0x18, 1, 0]),
            ]
        );
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }
}
