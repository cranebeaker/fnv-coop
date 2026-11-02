//! `NiMain/nibacktofrontaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// `NiBackToFrontAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BACK_TO_FRONT_ACCUMULATOR_RTTI: u32 = 0x011f_6058;

layout! {
    /// `NiBackToFrontAccumulator` (Xbox PDB), 0x30 bytes on PC (the
    /// constructor `00aa0f00` clears +0x0C..+0x28; the sized delete frees 0x30).
    pub struct NiBackToFrontAccumulator: 0x30 {
        /// `m_kItems` (Xbox PDB): an [`NiTPointerList`] of `NiGeometry*`.
        0x0c m_kItems: Inline<NiTPointerList>,
        /// `m_kItemsToSortAndRender` (Xbox PDB).
        0x18 m_kItemsToSortAndRender: u32,
        /// `m_iNumItems` (Xbox PDB).
        0x1c m_iNumItems: i32,
        /// `m_iMaxItems` (Xbox PDB).
        0x20 m_iMaxItems: i32,
        /// `m_ppkItems` (Xbox PDB).
        0x24 m_ppkItems: u32,
        /// `m_pfDepths` (Xbox PDB).
        0x28 m_pfDepths: u32,
        /// `m_iCurrItem` (Xbox PDB).
        0x2c m_iCurrItem: i32,
    }
}

/// `NiBackToFrontAccumulator`'s virtual table (RTTI `.?AVNiBackToFrontAccumulator@@`).
const NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE: u32 = 0x010a_23ac;
/// `NiAccumulator`'s virtual table (RTTI `.?AVNiAccumulator@@`).
const NI_ACCUMULATOR_VTABLE: u32 = 0x010a_2304;
/// `NiAccumulator::m_pkCamera` (Xbox PDB) +0x08.
const NI_ACCUMULATOR_CAMERA: u32 = 0x08;

// Callees outside this unit.
/// `00a5d3a0`: the `NiObject` constructor body (`niobject.cpp`), `__thiscall`.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `00a5d3d0`: the `NiObject` destructor body (`niobject.cpp`), `__thiscall`.
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00aa10f0`: the cdecl free wrapper (one word).
const FREE: u32 = 0x00aa_10f0;
/// `00b71300`: `NiTPointerListBase<NiTPointerAllocator<unsigned int>, char*>::RemoveAll`
/// (map name), `__thiscall` on the list.
const POINTER_LIST_REMOVE_ALL: u32 = 0x00b7_1300;
/// `0040fbf0` (`__thiscall(lock, 0)`): locks the shared pool of twelve-byte list
/// nodes; `011c5f80` is the lock, `011c5f84` its count, `011c5f58` the free list.
const POOL_LOCK_FUNCTION: u32 = 0x0040_fbf0;
const POOL_LOCK: u32 = 0x011c_5f80;
const POOL_LOCK_COUNT: u32 = 0x011c_5f84;
const POOL_FREE_LIST: u32 = 0x011c_5f58;

// Translated from 00aa0fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAccumulator` constructor body (unnamed in the map; the virtual table it
/// stores is `NiAccumulator`'s): the `NiObject` constructor (`00a5d3a0`), then
/// `NiAccumulator`'s virtual table and a null `m_pkCamera` (+0x08). Answers `this`.
pub fn fn_00aa0fc0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI_ACCUMULATOR_VTABLE);
    e.mem
        .set_u32(this.addr().wrapping_add(NI_ACCUMULATOR_CAMERA), 0);
    this
}

// Translated from 00aa0f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator` constructor (unnamed in the map; the virtual table
/// it stores is `NiBackToFrontAccumulator`'s): the `NiAccumulator` constructor
/// body ([`fn_00aa0fc0`]), the class's virtual table, then an empty item list
/// (count, head, tail) and every other field from +0x18 to +0x28 zero. +0x2C is
/// left as it was. Answers `this`.
pub fn fn_00aa0f00(
    e: &mut Engine,
    this: Ptr<NiBackToFrontAccumulator>,
) -> Ptr<NiBackToFrontAccumulator> {
    fn_00aa0fc0(e, this.cast());
    e.mem
        .set_u32(this.addr(), NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE);
    let items: Ptr<NiTPointerList> = Ptr::new(this.addr().wrapping_add(0x0c));
    e.set(items, NiTPointerList::m_uiCount, 0);
    e.set(items, NiTPointerList::m_pkHead, 0);
    e.set(items, NiTPointerList::m_pkTail, 0);
    e.set(this, NiBackToFrontAccumulator::m_kItemsToSortAndRender, 0);
    e.set(this, NiBackToFrontAccumulator::m_iNumItems, 0);
    e.set(this, NiBackToFrontAccumulator::m_iMaxItems, 0);
    e.set(this, NiBackToFrontAccumulator::m_ppkItems, 0);
    e.set(this, NiBackToFrontAccumulator::m_pfDepths, 0);
    this
}

// Translated from 00aa0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator` destructor body (unnamed in the map): the class's
/// virtual table, frees `m_ppkItems` then `m_pfDepths` (`00aa10f0`), then takes
/// every node off the head of `m_kItems` (the next node's previous link, or the
/// tail when it was the last, is cleared) and gives it back to the shared node
/// pool under its lock (element and previous link cleared, pushed on the free
/// list; the lock's owner is cleared when its count reaches zero), until the
/// count is zero. Then `RemoveAll` on the list (`00b71300`), `NiAccumulator`'s
/// virtual table and the `NiObject` destructor (`00a5d3d0`, a tail jump).
pub fn fn_00aa0dc0(e: &mut Engine, this: Ptr<NiBackToFrontAccumulator>) {
    let items_array = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
    e.mem
        .set_u32(this.addr(), NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE);
    e.call(FREE, &args![items_array]);
    let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
    e.call(FREE, &args![depths]);
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr().wrapping_add(0x0c));
    if e.get(list, NiTPointerList::m_uiCount) != 0 {
        loop {
            let node: Ptr<NiTListItem> = Ptr::new(e.get(list, NiTPointerList::m_pkHead));
            let next = e.get(node, NiTListItem::m_pkNext);
            e.set(list, NiTPointerList::m_pkHead, next);
            if next == 0 {
                e.set(list, NiTPointerList::m_pkTail, 0);
            } else {
                e.set(Ptr::<NiTListItem>::new(next), NiTListItem::m_pkPrev, 0);
            }
            e.set(node, NiTListItem::m_element, 0);
            e.call(POOL_LOCK_FUNCTION, &args![POOL_LOCK, 0u32]);
            e.set(node, NiTListItem::m_pkPrev, 0);
            let free_head = e.mem.u32(POOL_FREE_LIST);
            e.set(node, NiTListItem::m_pkNext, free_head);
            let lock_count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
            e.mem.set_u32(POOL_LOCK_COUNT, lock_count);
            e.mem.set_u32(POOL_FREE_LIST, node.addr());
            if lock_count == 0 {
                e.mem.set_u32(POOL_LOCK, 0);
            }
            let count = e.get(list, NiTPointerList::m_uiCount).wrapping_sub(1);
            e.set(list, NiTPointerList::m_uiCount, count);
            if count == 0 {
                break;
            }
        }
    }
    e.call(POINTER_LIST_REMOVE_ALL, &args![list]);
    e.mem.set_u32(this.addr(), NI_ACCUMULATOR_VTABLE);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00aa0e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_back_to_front_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BACK_TO_FRONT_ACCUMULATOR_RTTI)
}

// Translated from 00aa0f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00aa0dc0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_back_to_front_accumulator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00aa_0dc0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

/// The depth at `index` in `m_pfDepths`, as `FLD dword` leaves it.
fn depth(e: &Engine, this: Ptr<NiBackToFrontAccumulator>, index: i32) -> f64 {
    let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
    let at = depths.wrapping_add((index as u32).wrapping_mul(4));
    x87_load32(e.mem.u32(at))
}

/// `FCOMPP`/`FCOMP` with `TEST AH,0x41` and no jump: `a` (the value pushed last, `ST0`)
/// compares strictly above `b`, both ordered.
fn above(a: f64, b: f64) -> bool {
    a > b
}

// Translated from 00aa0a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::ChoosePivot` (Xbox PDB; `__thiscall`, two words: left, right
/// index): the median of the depths at `left`, `(left + right) >> 1` (arithmetic shift) and
/// `right`, by the exe's comparisons (each "above" is false for a NaN, which picks the branch
/// the flags give). Answers the chosen depth in `ST0`.
pub fn ni_back_to_front_accumulator_choose_pivot(
    e: &mut Engine,
    this: Ptr<NiBackToFrontAccumulator>,
    left: i32,
    right: i32,
) -> f64 {
    let middle = left.wrapping_add(right) >> 1;
    let (l, m, r) = (
        depth(e, this, left),
        depth(e, this, middle),
        depth(e, this, right),
    );
    if above(m, l) {
        if above(r, m) {
            m
        } else if above(r, l) {
            r
        } else {
            l
        }
    } else if above(r, l) {
        l
    } else if above(r, m) {
        r
    } else {
        m
    }
}

// Translated from 00aa0b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::SortObjectsByDepth` (Xbox PDB; `__thiscall`, two words: left,
/// right index): quicksorts `m_ppkItems` and `m_pfDepths` together by depth, ascending.
/// While `right > left`: the pivot is [`ni_back_to_front_accumulator_choose_pivot`] rounded
/// to a float; from the right a scan moves down past depths above the pivot and from the left
/// a scan moves up past depths below it (neither scan is bounded, as in the exe); crossing
/// scans end the pass, otherwise the two items and depths swap (the depths copied through the
/// x87 stack). When the right scan stopped at `right` the range shrinks by one; otherwise the
/// left part is sorted recursively and the right part is the next range.
pub fn ni_back_to_front_accumulator_sort_objects_by_depth(
    e: &mut Engine,
    this: Ptr<NiBackToFrontAccumulator>,
    left: i32,
    right: i32,
) {
    let mut left = left;
    let mut right = right;
    if right <= left {
        return;
    }
    loop {
        let pivot = ni_back_to_front_accumulator_choose_pivot(e, this, left, right);
        let pivot = x87_load32(x87_store32(pivot));
        let mut low = left.wrapping_sub(1);
        let mut high = right.wrapping_add(1);
        loop {
            loop {
                high = high.wrapping_sub(1);
                if !above(depth(e, this, high), pivot) {
                    break;
                }
            }
            loop {
                low = low.wrapping_add(1);
                // `TEST AH,0x5` / `JNP`: on while below the pivot, ordered.
                let value = depth(e, this, low);
                if value >= pivot || value.is_nan() {
                    break;
                }
            }
            if low >= high {
                break;
            }
            let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
            let low_at = items.wrapping_add((low as u32).wrapping_mul(4));
            let high_at = items.wrapping_add((high as u32).wrapping_mul(4));
            let low_item = e.mem.u32(low_at);
            let high_item = e.mem.u32(high_at);
            e.mem.set_u32(low_at, high_item);
            e.mem.set_u32(high_at, low_item);
            let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
            let low_at = depths.wrapping_add((low as u32).wrapping_mul(4));
            let high_at = depths.wrapping_add((high as u32).wrapping_mul(4));
            let saved = x87_quiet_bits(e.mem.u32(low_at));
            let moved = x87_quiet_bits(e.mem.u32(high_at));
            e.mem.set_u32(low_at, moved);
            e.mem.set_u32(high_at, saved);
        }
        if high == right {
            right = high.wrapping_sub(1);
        } else {
            ni_back_to_front_accumulator_sort_objects_by_depth(e, this, left, high);
            left = high.wrapping_add(1);
        }
        if right <= left {
            return;
        }
    }
}

/// `00ba87d0` (`thiscall`, the other object): the base comparison `IsEqual` passes on.
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
/// `00aa1070` (`cdecl`, the size): the array allocation.
const ALLOCATE: u32 = 0x00aa_1070;
/// The bound centre used for an object without a world bound (`011f4288`, three floats).
const ZERO_POINT: u32 = 0x011f_4288;
/// The renderer object (`011f4748`) finishing hands every item to.
const RENDERER_OBJECT: u32 = 0x011f_4748;
/// `NiAccumulator::FinishAccumulating` (`00aa0fe0`, `thiscall`), the tail jump of the
/// override.
const ACCUMULATOR_FINISH: u32 = 0x00aa_0fe0;
/// `0043a080` (no arguments): refills the shared node pool's free list.
const POOL_REFILL: u32 = 0x0043_a080;
/// `00aa13e0` (`cdecl`, the size): the object allocation; `00aa1000` (`thiscall`, (clone,
/// cloning process); the engine map names it `NiKeyBasedInterpolator::CopyMembers`, folded
/// code): the base member copy.
const NI_NEW: u32 = 0x00aa_13e0;
const OBJECT_COPY_MEMBERS: u32 = 0x00aa_1000;

// Translated from 00aa0ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::IsEqual` (Xbox PDB): the base comparison (`00ba87d0`) as a
/// `bool`.
pub fn ni_back_to_front_accumulator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(BASE_IS_EQUAL, &args![this, other]).u8() != 0
}

// Translated from 00aa0c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::Sort` (Xbox PDB): the list to sort (`m_kItemsToSortAndRender`,
/// +0x18; this one's own `m_kItems` when null) gives the item count (+0x1c); with items, the
/// item and depth arrays (+0x24, +0x28) grow to the count when it passes `m_iMaxItems` (+0x20;
/// freed by `00aa10f0`, made by `00aa1070`), each item is stored and its depth is its world
/// bound's centre (+0x20; `011f4288` without a bound) dotted with the camera's world direction
/// (column 0 of the world rotation of `m_pkCamera` +8), and the arrays are sorted
/// ([`ni_back_to_front_accumulator_sort_objects_by_depth`] from 0 to count - 1, also for a
/// negative count). x87 arithmetic, one helper per instruction.
pub fn ni_back_to_front_accumulator_sort(e: &mut Engine, this: Ptr<NiBackToFrontAccumulator>) {
    let t = this.addr();
    if e.get(this, NiBackToFrontAccumulator::m_kItemsToSortAndRender) == 0 {
        e.set(
            this,
            NiBackToFrontAccumulator::m_kItemsToSortAndRender,
            t + 0xc,
        );
    }
    let list = e.get(this, NiBackToFrontAccumulator::m_kItemsToSortAndRender);
    let count = e.mem.u32(list + 8) as i32;
    e.set(this, NiBackToFrontAccumulator::m_iNumItems, count);
    if count == 0 {
        return;
    }
    if count > e.get(this, NiBackToFrontAccumulator::m_iMaxItems) {
        let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
        e.call(FREE, &args![items]);
        let count = e.get(this, NiBackToFrontAccumulator::m_iNumItems);
        e.set(this, NiBackToFrontAccumulator::m_iMaxItems, count);
        let items = e
            .call(ALLOCATE, &args![(count as u32).wrapping_mul(4)])
            .u32();
        let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
        e.set(this, NiBackToFrontAccumulator::m_ppkItems, items);
        e.call(FREE, &args![depths]);
        let max = e.get(this, NiBackToFrontAccumulator::m_iMaxItems) as u32;
        let depths = e.call(ALLOCATE, &args![max.wrapping_mul(4)]).u32();
        e.set(this, NiBackToFrontAccumulator::m_pfDepths, depths);
    }
    let list = e.get(this, NiBackToFrontAccumulator::m_kItemsToSortAndRender);
    let mut node = e.mem.u32(list);
    let camera = e.mem.u32(t + NI_ACCUMULATOR_CAMERA);
    // The direction goes through three float locals before the loop loads them back.
    let direction: Vec<f64> = [0x68u32, 0x74, 0x80]
        .iter()
        .map(|offset| x87_load32(e.mem.u32(camera + offset)))
        .collect();
    let mut index = 0i32;
    if e.get(this, NiBackToFrontAccumulator::m_iNumItems) > 0 {
        loop {
            let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
            let element = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            e.mem.set_u32(items + 4 * index as u32, element);
            let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
            let object = e.mem.u32(items + 4 * index as u32);
            let mut centre = e.mem.u32(object + 0x20);
            if centre == 0 {
                centre = ZERO_POINT;
            }
            index += 1;
            let y = x87_mul(x87_load32(e.mem.u32(centre + 4)), direction[1]);
            let x = x87_mul(x87_load32(e.mem.u32(centre)), direction[0]);
            let sum = x87_add(y, x);
            let z = x87_mul(x87_load32(e.mem.u32(centre + 8)), direction[2]);
            let value = x87_load32(x87_store32(x87_add(sum, z)));
            let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
            e.mem
                .set_u32(depths + 4 * (index as u32 - 1), x87_store32(value));
            if index >= e.get(this, NiBackToFrontAccumulator::m_iNumItems) {
                break;
            }
        }
    }
    let last = e
        .get(this, NiBackToFrontAccumulator::m_iNumItems)
        .wrapping_sub(1);
    ni_back_to_front_accumulator_sort_objects_by_depth(e, this, 0, last);
}

// Translated from 00aa0d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::FinishAccumulating` (Xbox PDB): only with a renderer object
/// (`011f4748`): the accumulator sorts (its slot +0xa4), then the items are rendered from the
/// last to the first (`m_iCurrItem` +0x2c counting down; each item's slot +0xdc with the
/// renderer) until the count runs out or a null item is met; then every node of `m_kItems` is
/// taken off the head and given back to the shared node pool under its lock (as the destructor
/// does), and `NiAccumulator::FinishAccumulating` (`00aa0fe0`) runs (a tail jump).
pub fn ni_back_to_front_accumulator_finish_accumulating(
    e: &mut Engine,
    this: Ptr<NiBackToFrontAccumulator>,
) {
    let renderer = e.global::<u32>(RENDERER_OBJECT);
    if renderer == 0 {
        return;
    }
    e.vcall(this.addr(), 0xa4, &args![]);
    let mut current = e.get(this, NiBackToFrontAccumulator::m_iNumItems);
    e.set(this, NiBackToFrontAccumulator::m_iCurrItem, current);
    while current != 0 {
        current = current.wrapping_sub(1);
        e.set(this, NiBackToFrontAccumulator::m_iCurrItem, current);
        let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
        let item = e
            .mem
            .u32(items.wrapping_add((current as u32).wrapping_mul(4)));
        if item == 0 {
            break;
        }
        e.vcall(item, 0xdc, &args![renderer]);
        current = e.get(this, NiBackToFrontAccumulator::m_iCurrItem);
    }
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr() + 0xc);
    while e.get(list, NiTPointerList::m_uiCount) != 0 {
        let node: Ptr<NiTListItem> = Ptr::new(e.get(list, NiTPointerList::m_pkHead));
        let next = e.get(node, NiTListItem::m_pkNext);
        e.set(list, NiTPointerList::m_pkHead, next);
        if next == 0 {
            e.set(list, NiTPointerList::m_pkTail, 0);
        } else {
            e.set(Ptr::<NiTListItem>::new(next), NiTListItem::m_pkPrev, 0);
        }
        e.set(node, NiTListItem::m_element, 0);
        e.call(POOL_LOCK_FUNCTION, &args![POOL_LOCK, 0u32]);
        e.set(node, NiTListItem::m_pkPrev, 0);
        let free_head = e.mem.u32(POOL_FREE_LIST);
        e.set(node, NiTListItem::m_pkNext, free_head);
        let lock_count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(POOL_LOCK_COUNT, lock_count);
        e.mem.set_u32(POOL_FREE_LIST, node.addr());
        if lock_count == 0 {
            e.mem.set_u32(POOL_LOCK, 0);
        }
        let count = e.get(list, NiTPointerList::m_uiCount).wrapping_sub(1);
        e.set(list, NiTPointerList::m_uiCount, count);
    }
    e.call(ACCUMULATOR_FINISH, &args![this]);
}

// Translated from 00aa0e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::RegisterObjectArray` (Xbox PDB): appends each object of the
/// array (`+0` base, `+4` count) to the tail of `m_kItems`, each in a node taken from the
/// shared node pool under its lock (the pool refilled by `0043a080` when its free list is
/// empty; the node cleared, the lock's owner cleared when its count reaches zero).
pub fn ni_back_to_front_accumulator_register_object_array(e: &mut Engine, this: Ptr, array: Ptr) {
    let count = e.mem.u32(array.addr() + 4);
    let list: Ptr<NiTPointerList> = Ptr::new(this.addr() + 0xc);
    let mut index = 0u32;
    while index < count {
        let object = e.mem.u32(e.mem.u32(array.addr()) + 4 * index);
        e.call(POOL_LOCK_FUNCTION, &args![POOL_LOCK, 0u32]);
        let mut node = e.mem.u32(POOL_FREE_LIST);
        if node == 0 {
            e.call(POOL_REFILL, &args![]);
            node = e.mem.u32(POOL_FREE_LIST);
        }
        let node: Ptr<NiTListItem> = Ptr::new(node);
        let next_free = e.get(node, NiTListItem::m_pkNext);
        e.mem.set_u32(POOL_FREE_LIST, next_free);
        e.set(node, NiTListItem::m_element, 0);
        e.set(node, NiTListItem::m_pkNext, 0);
        e.set(node, NiTListItem::m_pkPrev, 0);
        let lock_count = e.mem.u32(POOL_LOCK_COUNT).wrapping_sub(1);
        e.mem.set_u32(POOL_LOCK_COUNT, lock_count);
        if lock_count == 0 {
            e.mem.set_u32(POOL_LOCK, 0);
        }
        e.set(node, NiTListItem::m_element, object);
        e.set(node, NiTListItem::m_pkNext, 0);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        e.set(node, NiTListItem::m_pkPrev, tail);
        let tail = e.get(list, NiTPointerList::m_pkTail);
        if tail != 0 {
            e.set(
                Ptr::<NiTListItem>::new(tail),
                NiTListItem::m_pkNext,
                node.addr(),
            );
        } else {
            e.set(list, NiTPointerList::m_pkHead, node.addr());
        }
        let total = e.get(list, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(list, NiTPointerList::m_uiCount, total);
        index += 1;
        e.set(list, NiTPointerList::m_pkTail, node.addr());
    }
}

// Translated from 00aa0f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBackToFrontAccumulator::CreateClone` (Xbox PDB): a 0x30-byte accumulator from `00aa13e0`
/// (the `NiAccumulator` constructor body [`fn_00aa0fc0`], the class's vtable, an empty item
/// list and +0x18 to +0x28 zero; null when the allocation fails) and the base member copy
/// (`00aa1000`, run on a null clone too). Answers the clone.
pub fn ni_back_to_front_accumulator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning_process: Ptr,
) -> Ptr {
    let block = e.call(NI_NEW, &args![0x30u32]).u32();
    if block == 0 {
        e.call(OBJECT_COPY_MEMBERS, &args![this, 0u32, cloning_process]);
        return Ptr::new(0);
    }
    fn_00aa0fc0(e, Ptr::new(block));
    e.mem.set_u32(block, NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE);
    for offset in [0x14u32, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x28] {
        e.mem.set_u32(block + offset, 0);
    }
    e.call(OBJECT_COPY_MEMBERS, &args![this, block, cloning_process]);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa0a20,
            ni_back_to_front_accumulator_choose_pivot(
                Ptr<NiBackToFrontAccumulator>,
                i32,
                i32,
            ) -> f64
        ),
        entry!(
            0x00aa0b40,
            ni_back_to_front_accumulator_sort_objects_by_depth(
                Ptr<NiBackToFrontAccumulator>,
                i32,
                i32,
            )
        ),
        entry!(0x00aa0e50, ni_back_to_front_accumulator_get_rtti() -> Ptr),
        entry!(0x00aa0f30, ni_back_to_front_accumulator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00aa0fc0, fn_00aa0fc0(Ptr) -> Ptr),
        entry!(
            0x00aa0f00,
            fn_00aa0f00(Ptr<NiBackToFrontAccumulator>) -> Ptr<NiBackToFrontAccumulator>
        ),
        entry!(0x00aa0dc0, fn_00aa0dc0(Ptr<NiBackToFrontAccumulator>)),
        entry!(0x00aa_0ae0, ni_back_to_front_accumulator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00aa_0c00,
            ni_back_to_front_accumulator_sort(Ptr<NiBackToFrontAccumulator>)
        ),
        entry!(
            0x00aa_0d00,
            ni_back_to_front_accumulator_finish_accumulating(Ptr<NiBackToFrontAccumulator>)
        ),
        entry!(
            0x00aa_0e60,
            ni_back_to_front_accumulator_register_object_array(Ptr, Ptr)
        ),
        entry!(0x00aa_0f50, ni_back_to_front_accumulator_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_back_to_front_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00aa0e50, &args![]).u32(),
            NI_BACK_TO_FRONT_ACCUMULATOR_RTTI
        );
    }

    #[test]
    fn ni_back_to_front_accumulator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00aa_0dc0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00aa0f30, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00aa0f30, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00aa_0dc0), vec![vec![this]; 2]);
    }

    fn calls_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn fn_00aa0fc0_builds_the_accumulator_base() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_OBJECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 0xdead);
        assert_eq!(e.call(0x00aa0fc0, &args![this]).u32(), this);
        assert_eq!(calls_of(&e, NI_OBJECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_ACCUMULATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn fn_00aa0f00_clears_the_fields_and_leaves_the_current_item() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        for off in (0..0x30).step_by(4) {
            e.mem.set_u32(this + off, 0x7777_0000 + off);
        }
        assert_eq!(e.call(0x00aa0f00, &args![this]).u32(), this);
        assert_eq!(calls_of(&e, NI_OBJECT_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x7777_0004);
        for off in (0x08..0x2c).step_by(4) {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x2c), 0x7777_002c);
    }

    #[test]
    fn fn_00aa0dc0_frees_the_arrays_and_returns_every_node_to_the_pool() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [FREE, POINTER_LIST_REMOVE_ALL, POOL_LOCK_FUNCTION] {
            e.register(addr, |_, _| Ret::default());
        }
        // The vtable must already be the class's when the base destructor runs.
        e.register(NI_OBJECT_DESTRUCTOR, |e, a| {
            assert_eq!(e.mem.u32(a[0]), NI_ACCUMULATOR_VTABLE);
            Ret::default()
        });
        e.map(POOL_FREE_LIST, 4);
        e.map(POOL_LOCK, 8);
        e.mem.set_u32(POOL_FREE_LIST, 0x9999);
        e.mem.set_u32(POOL_LOCK, 0x55);
        e.mem.set_u32(POOL_LOCK_COUNT, 2);
        let this = e.mem.alloc(0x30);
        let a = e.mem.alloc(0x0c);
        let b = e.mem.alloc(0x0c);
        // a <-> b, elements 0x11 and 0x22.
        e.mem.set_u32(a, b);
        e.mem.set_u32(a + 4, 0);
        e.mem.set_u32(a + 8, 0x11);
        e.mem.set_u32(b, 0);
        e.mem.set_u32(b + 4, a);
        e.mem.set_u32(b + 8, 0x22);
        e.mem.set_u32(this + 0x0c, a);
        e.mem.set_u32(this + 0x10, b);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x24, 0x4400);
        e.mem.set_u32(this + 0x28, 0x4800);
        e.call(0x00aa0dc0, &args![this]);
        assert_eq!(calls_of(&e, FREE), vec![vec![0x4400], vec![0x4800]]);
        assert_eq!(
            calls_of(&e, POOL_LOCK_FUNCTION),
            vec![vec![POOL_LOCK, 0]; 2]
        );
        assert_eq!(
            calls_of(&e, POINTER_LIST_REMOVE_ALL),
            vec![vec![this + 0x0c]]
        );
        assert_eq!(calls_of(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this]]);
        // Both nodes cleared and pushed: b on top of a on top of the old head.
        assert_eq!(e.mem.u32(POOL_FREE_LIST), b);
        assert_eq!(
            (e.mem.u32(b), e.mem.u32(b + 4), e.mem.u32(b + 8)),
            (a, 0, 0)
        );
        assert_eq!(
            (e.mem.u32(a), e.mem.u32(a + 4), e.mem.u32(a + 8)),
            (0x9999, 0, 0)
        );
        assert_eq!(e.mem.u32(POOL_LOCK_COUNT), 0);
        // The lock's owner is cleared only when its count reached zero.
        assert_eq!(e.mem.u32(POOL_LOCK), 0);
        assert_eq!(e.mem.u32(this + 0x0c), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn fn_00aa0dc0_with_an_empty_list_takes_no_lock() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [
            FREE,
            POINTER_LIST_REMOVE_ALL,
            POOL_LOCK_FUNCTION,
            NI_OBJECT_DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(POOL_LOCK, 8);
        e.mem.set_u32(POOL_LOCK, 0x55);
        e.mem.set_u32(POOL_LOCK_COUNT, 3);
        let this = e.mem.alloc(0x30);
        e.call(0x00aa0dc0, &args![this]);
        assert!(calls_of(&e, POOL_LOCK_FUNCTION).is_empty());
        assert_eq!(calls_of(&e, FREE), vec![vec![0], vec![0]]);
        assert_eq!(e.mem.u32(POOL_LOCK), 0x55);
        assert_eq!(e.mem.u32(this), NI_ACCUMULATOR_VTABLE);
    }

    #[test]
    fn fn_00aa0dc0_keeps_the_pool_owner_while_the_lock_is_held() {
        let mut e = Engine::new();
        for addr in [
            FREE,
            POINTER_LIST_REMOVE_ALL,
            POOL_LOCK_FUNCTION,
            NI_OBJECT_DESTRUCTOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.map(POOL_FREE_LIST, 4);
        e.map(POOL_LOCK, 8);
        e.mem.set_u32(POOL_LOCK, 0x55);
        e.mem.set_u32(POOL_LOCK_COUNT, 5);
        let this = e.mem.alloc(0x30);
        let node = e.mem.alloc(0x0c);
        e.mem.set_u32(node + 8, 0x11);
        e.mem.set_u32(this + 0x0c, node);
        e.mem.set_u32(this + 0x10, node);
        e.mem.set_u32(this + 0x14, 1);
        e.call(0x00aa0dc0, &args![this]);
        assert_eq!(e.mem.u32(POOL_LOCK_COUNT), 4);
        assert_eq!(e.mem.u32(POOL_LOCK), 0x55);
        assert_eq!(e.mem.u32(POOL_FREE_LIST), node);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}

#[cfg(test)]
mod b007_tests {
    use super::*;

    /// An accumulator whose items are 0x100 + i and whose depths are `values`.
    fn accumulator(e: &mut Engine, values: &[f32]) -> u32 {
        let this = e.mem.alloc(0x30);
        let items = e.mem.alloc(4 * values.len() as u32 + 8);
        let depths = e.mem.alloc(4 * values.len() as u32 + 8);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(items + 4 * i as u32, 0x100 + i as u32);
            e.mem.set_u32(depths + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u32(this + 0x24, items);
        e.mem.set_u32(this + 0x28, depths);
        this
    }

    fn pivot(e: &mut Engine, values: &[f32]) -> f64 {
        let this = accumulator(e, values);
        let right = values.len() as i32 - 1;
        e.call(0x00aa0a20, &args![this, 0i32, right]).f64()
    }

    #[test]
    fn choose_pivot_takes_the_median_of_three() {
        let mut e = Engine::new();
        assert_eq!(pivot(&mut e, &[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(pivot(&mut e, &[1.0, 3.0, 2.0]), 2.0);
        assert_eq!(pivot(&mut e, &[2.0, 3.0, 1.0]), 2.0);
        assert_eq!(pivot(&mut e, &[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(pivot(&mut e, &[2.0, 1.0, 3.0]), 2.0);
        assert_eq!(pivot(&mut e, &[3.0, 2.0, 1.0]), 2.0);
        assert_eq!(pivot(&mut e, &[5.0, 5.0, 5.0, 5.0]), 5.0);
        // A NaN in the middle: m above l is false, r above l: the left depth.
        assert_eq!(pivot(&mut e, &[1.0, f32::NAN, 3.0]), 1.0);
    }

    #[test]
    fn sort_objects_by_depth_sorts_items_with_their_depths() {
        let mut e = Engine::new();
        let values = [5.0f32, 1.0, 4.0, 2.0, 3.0, 0.5, 9.0, 2.0];
        let this = accumulator(&mut e, &values);
        e.call(0x00aa0b40, &args![this, 0i32, 7i32]);
        let items = e.mem.u32(this + 0x24);
        let depths = e.mem.u32(this + 0x28);
        let sorted: Vec<f32> = (0..8)
            .map(|i| f32::from_bits(e.mem.u32(depths + 4 * i)))
            .collect();
        assert_eq!(sorted, vec![0.5, 1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 9.0]);
        for i in 0..8 {
            let item = e.mem.u32(items + 4 * i) - 0x100;
            assert_eq!(values[item as usize], sorted[i as usize]);
        }
        // An empty or one-element range is left alone.
        e.mem.set_u32(depths, 7.0f32.to_bits());
        e.call(0x00aa0b40, &args![this, 3i32, 3i32]);
        e.call(0x00aa0b40, &args![this, 3i32, 1i32]);
        assert_eq!(e.mem.u32(depths), 7.0f32.to_bits());
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn is_equal_passes_on_the_base_answer() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00aa_0ae0, &args![0x4000u32, 0x5000u32]).bool());
        assert_eq!(calls_to(&e, BASE_IS_EQUAL), vec![vec![0x4000, 0x5000]]);
        e.register(BASE_IS_EQUAL, |_, _| 1u32.into_ret());
        assert!(e.call(0x00aa_0ae0, &args![0x4000u32, 0x5000u32]).bool());
    }

    /// An object with a world bound centred at `centre` (or none).
    fn object(e: &mut Engine, centre: Option<[f32; 3]>) -> u32 {
        let object = e.mem.alloc(0x24);
        if let Some(centre) = centre {
            let bound = e.mem.alloc(0x10);
            for (index, value) in centre.into_iter().enumerate() {
                e.mem.set_f32(bound + 4 * index as u32, value);
            }
            e.mem.set_u32(object + 0x20, bound);
        }
        object
    }

    /// An accumulator whose own list holds `objects` (in order) and whose camera looks along
    /// `direction`.
    fn accumulator(
        e: &mut Engine,
        objects: &[u32],
        direction: [f32; 3],
    ) -> Ptr<NiBackToFrontAccumulator> {
        let this = e.new_object::<NiBackToFrontAccumulator>();
        let camera = e.mem.alloc(0x90);
        for (index, offset) in [0x68u32, 0x74, 0x80].into_iter().enumerate() {
            e.mem.set_f32(camera + offset, direction[index]);
        }
        e.mem.set_u32(this.addr() + 8, camera);
        let mut previous = 0;
        for object in objects.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, previous);
            e.mem.set_u32(node + 8, *object);
            previous = node;
        }
        e.mem.set_u32(this.addr() + 0xc, previous);
        e.mem.set_u32(this.addr() + 0x14, objects.len() as u32);
        this
    }

    #[test]
    fn sort_orders_the_items_by_depth() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.register(FREE, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let far = object(&mut e, Some([0.0, 0.0, 9.0]));
        let near = object(&mut e, Some([0.0, 0.0, 1.0]));
        let none = object(&mut e, None);
        e.mem.set_f32(ZERO_POINT + 8, 4.0);
        let this = accumulator(&mut e, &[far, near, none], [0.0, 0.0, 1.0]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_0c00, &args![this]);
        assert_eq!(
            e.get(this, NiBackToFrontAccumulator::m_kItemsToSortAndRender),
            this.addr() + 0xc
        );
        assert_eq!(e.get(this, NiBackToFrontAccumulator::m_iNumItems), 3);
        assert_eq!(e.get(this, NiBackToFrontAccumulator::m_iMaxItems), 3);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![12], vec![12]]);
        let items = e.get(this, NiBackToFrontAccumulator::m_ppkItems);
        let depths = e.get(this, NiBackToFrontAccumulator::m_pfDepths);
        let sorted: Vec<u32> = (0..3).map(|i| e.mem.u32(items + 4 * i)).collect();
        assert_eq!(sorted, vec![near, none, far]);
        let values: Vec<f32> = (0..3).map(|i| e.mem.f32(depths + 4 * i)).collect();
        assert_eq!(values, vec![1.0, 4.0, 9.0]);
        // Enough room: no new arrays; an empty list: nothing at all.
        e.call_log = Some(vec![]);
        e.call(0x00aa_0c00, &args![this]);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        e.mem.set_u32(this.addr() + 0x14, 0);
        e.mem.set_u32(depths, 0);
        e.call(0x00aa_0c00, &args![this]);
        assert_eq!(e.mem.u32(depths), 0);
    }

    #[test]
    fn finish_renders_from_the_back_and_empties_the_list() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011c_5000, 0x1000);
        e.register(POOL_LOCK_FUNCTION, |e, _| {
            let count = e.mem.u32(POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(POOL_LOCK_COUNT, count);
            e.mem.set_u32(POOL_LOCK, 1);
            Ret::default()
        });
        e.register(ACCUMULATOR_FINISH, |_, _| Ret::default());
        e.register(0x0300_00a4, |_, _| Ret::default());
        e.register(0x0300_00dc, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x38];
        slots[0x29] = 0x0300_00a4;
        slots[0x37] = 0x0300_00dc;
        e.put_vtable(0x0300_1000, &slots);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.mem.set_u32(first, 0x0300_1000);
        e.mem.set_u32(second, 0x0300_1000);
        let this = accumulator(&mut e, &[first, second], [0.0, 0.0, 1.0]);
        e.mem.set_u32(this.addr(), 0x0300_1000);
        let items = e.mem.alloc(8);
        e.mem.set_u32(items, first);
        e.mem.set_u32(items + 4, second);
        e.set(this, NiBackToFrontAccumulator::m_ppkItems, items);
        e.set(this, NiBackToFrontAccumulator::m_iNumItems, 2);
        e.call_log = Some(vec![]);
        // No renderer: nothing.
        e.call(0x00aa_0d00, &args![this]);
        assert!(calls_to(&e, 0x0300_00a4).is_empty());
        e.mem.set_u32(RENDERER_OBJECT, 0x7777);
        e.call(0x00aa_0d00, &args![this]);
        assert_eq!(calls_to(&e, 0x0300_00a4), vec![vec![this.addr()]]);
        assert_eq!(
            calls_to(&e, 0x0300_00dc),
            vec![vec![second, 0x7777], vec![first, 0x7777]]
        );
        assert_eq!(e.get(this, NiBackToFrontAccumulator::m_iCurrItem), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
        assert_eq!(e.mem.u32(POOL_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(POOL_LOCK), 0);
        assert_ne!(e.mem.u32(POOL_FREE_LIST), 0);
        assert_eq!(calls_to(&e, ACCUMULATOR_FINISH), vec![vec![this.addr()]]);
        // A null item stops the rendering.
        e.mem.set_u32(items + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_0d00, &args![this]);
        assert!(calls_to(&e, 0x0300_00dc).is_empty());
        assert_eq!(e.get(this, NiBackToFrontAccumulator::m_iCurrItem), 1);
    }

    #[test]
    fn register_object_array_appends_from_the_pool() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register(POOL_LOCK_FUNCTION, |e, _| {
            let count = e.mem.u32(POOL_LOCK_COUNT) + 1;
            e.mem.set_u32(POOL_LOCK_COUNT, count);
            Ret::default()
        });
        e.register(POOL_REFILL, |e, _| {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(POOL_FREE_LIST, node);
            Ret::default()
        });
        let this = e.new_object::<NiBackToFrontAccumulator>();
        let base = e.mem.alloc(8);
        e.mem.set_u32(base, 0x1111);
        e.mem.set_u32(base + 4, 0x2222);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, base);
        e.mem.set_u32(array + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_0e60, &args![this, array]);
        let head = e.mem.u32(this.addr() + 0xc);
        let tail = e.mem.u32(this.addr() + 0x10);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 2);
        assert_eq!((e.mem.u32(head + 8), e.mem.u32(head)), (0x1111, tail));
        assert_eq!((e.mem.u32(tail + 8), e.mem.u32(tail + 4)), (0x2222, head));
        assert_eq!(calls_to(&e, POOL_REFILL).len(), 2);
        assert_eq!(e.mem.u32(POOL_LOCK_COUNT), 0);
    }

    #[test]
    fn create_clone_builds_an_empty_accumulator() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x30]);
            block.into_ret()
        });
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let clone = e.call(0x00aa_0f50, &args![0x4000u32, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_BACK_TO_FRONT_ACCUMULATOR_VTABLE);
        for offset in [8u32, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(clone + offset), 0, "{offset:x}");
        }
        assert_eq!(e.mem.u32(clone + 0x2c), 0xcdcd_cdcd);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS),
            vec![vec![0x4000, clone, 0x66]]
        );
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00aa_0f50, &args![0x4000u32, 0x66u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, OBJECT_COPY_MEMBERS).last().unwrap(),
            &vec![0x4000, 0, 0x66]
        );
    }
}
