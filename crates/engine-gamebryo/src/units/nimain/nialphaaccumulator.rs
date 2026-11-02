//! `NiMain/nialphaaccumulator.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiAlphaAccumulator::ms_RTTI` (the class's `NiRTTI` record).
const NI_ALPHA_ACCUMULATOR_RTTI: u32 = 0x011f_5f74;

// Translated from 00a9b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_alpha_accumulator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ALPHA_ACCUMULATOR_RTTI)
}

/// `00aa0ac0`: the jump target of `00a9b4f0`.
const JUMP_TARGET_00A9B4F0: u32 = 0x00aa_0ac0;

/// `00aa0ad0`: the jump target of `00a9b500`.
const JUMP_TARGET_00A9B500: u32 = 0x00aa_0ad0;

// Translated from 00a9b4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::LoadBinary` (Xbox PDB): a 5-byte jump to `00aa0ac0` (engine map: `NiKeyBasedInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiAlphaAccumulator@@UAAXAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A9B4F0, &args![this, stream]);
}

// Translated from 00a9b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00aa0ad0`, passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiAlphaAccumulator@@UAA_NAAVNiStream@@@Z`.
pub fn ni_alpha_accumulator_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A9B500, &args![this, stream]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9b530, ni_alpha_accumulator_get_rtti() -> Ptr),
        entry!(0x00a9_b4f0, ni_alpha_accumulator_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b500, ni_alpha_accumulator_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a9b570, ni_alpha_accumulator_sort(Ptr)),
        entry!(0x00a9_b510, ni_alpha_accumulator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_b540,
            ni_alpha_accumulator_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00a9_b730, ni_alpha_accumulator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_b790,
            ni_alpha_accumulator_register_object_array(Ptr, Ptr)
        ),
        entry!(
            0x00a9_b850,
            ni_alpha_accumulator_register_object(Ptr, Ptr) -> bool
        ),
    ]
}

/// `NiBackToFrontAccumulator::IsEqual` (`00aa0ae0`, thiscall: other object), its
/// destructor body (`00aa0dc0`), its constructor (`00aa0f00`, by the vtable the clone then
/// stores) and the member copy (`00aa0ab0`, thiscall on the source: clone, cloning process).
const BACK_TO_FRONT_IS_EQUAL: u32 = 0x00aa_0ae0;
const BACK_TO_FRONT_DESTRUCT: u32 = 0x00aa_0dc0;
const BACK_TO_FRONT_CONSTRUCT: u32 = 0x00aa_0f00;
const BACK_TO_FRONT_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiAlphaAccumulator`'s vtable (`010a13ac`, the scalar deleting destructor's) and size.
const NI_ALPHA_ACCUMULATOR_VTABLE: u32 = 0x010a_13ac;
const NI_ALPHA_ACCUMULATOR_SIZE: u32 = 0x34;
/// `NiAlphaAccumulator::m_bObserveNoSortHint` (Xbox PDB) +0x30.
const OBSERVE_NO_SORT_HINT: u32 = 0x30;
/// `NiNew`'s allocator (`00aa13e0`, cdecl: size) and the sized free (`00aa1460`, cdecl:
/// block, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// The renderer object (`011f4748`).
const RENDERER: u32 = 0x011f_4748;
/// The geometry's virtual slot +0xdc (called with the renderer: drawn now), its word at
/// +0x9c (an object whose 16-bit word at +0x18 the test reads: bit 0 must be set and, with
/// the no-sort hint observed, bit 13 clear) and its word at +0x30 (bit 6 must be clear).
const GEOMETRY_SLOT_RENDER: u32 = 0xdc;
const GEOMETRY_SORT_FLAGS_OWNER: u32 = 0x9c;
const GEOMETRY_FLAGS: u32 = 0x30;
/// The pointer list's node allocation (`0043a010`, from the shared node pool; called with
/// `ECX` the list's count word) and its add at the tail (`00b58570`, thiscall on the list:
/// address of the element).
const LIST_NEW_NODE: u32 = 0x0043_a010;
const LIST_ADD_TAIL: u32 = 0x00b5_8570;

/// Whether `geometry` goes into the sort list: its sort flags (the word at +0x9c, then
/// +0x18) allow sorting, the no-sort hint is not observed or not set, and bit 6 of its word
/// at +0x30 is clear.
fn sorts(e: &Engine, this: u32, geometry: u32) -> bool {
    let owner = e.mem.u32(geometry.wrapping_add(GEOMETRY_SORT_FLAGS_OWNER));
    if e.mem.u8(owner.wrapping_add(0x18)) & 1 == 0 {
        return false;
    }
    if e.mem.u8(this.wrapping_add(OBSERVE_NO_SORT_HINT)) != 0
        && (e.mem.u16(owner.wrapping_add(0x18)) >> 13) & 1 != 0
    {
        return false;
    }
    (!(e.mem.u32(geometry.wrapping_add(GEOMETRY_FLAGS)) >> 6) as u8) & 1 != 0
}

// Translated from 00a9b510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::IsEqual` (Xbox PDB; thiscall, `RET 4`):
/// `NiBackToFrontAccumulator::IsEqual` (`00aa0ae0`), its answer as a boolean.
pub fn ni_alpha_accumulator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(BACK_TO_FRONT_IS_EQUAL, &args![this, other]).u8() != 0
}

// Translated from 00a9b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator`'s scalar deleting destructor (Xbox PDB; thiscall, `RET 4`): the
/// class's vtable, `NiBackToFrontAccumulator`'s destructor body (`00aa0dc0`) and, with bit
/// 0 of `flags`, the 0x34 bytes given back (`00aa1460`). Answers `this`.
pub fn ni_alpha_accumulator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_ALPHA_ACCUMULATOR_VTABLE);
    e.call(BACK_TO_FRONT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, NI_ALPHA_ACCUMULATOR_SIZE]);
    }
    this
}

// Translated from 00a9b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::CreateClone` (Xbox PDB; thiscall, `RET 4`: cloning process): 0x34
/// bytes (`00aa13e0`) constructed as an `NiBackToFrontAccumulator` (`00aa0f00`) with the
/// class's vtable, the no-sort hint observed and both sort flags clear (null when the
/// allocation fails); the members copied (`00aa0ab0`), then the no-sort hint and the
/// closest-point flag copied (written through the null clone too, as the exe does).
pub fn ni_alpha_accumulator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_ALPHA_ACCUMULATOR_SIZE]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(BACK_TO_FRONT_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NI_ALPHA_ACCUMULATOR_VTABLE);
        e.mem.set_u8(block + OBSERVE_NO_SORT_HINT, 1);
        e.mem.set_u8(block + SORT_BY_CLOSEST_POINT, 0);
        e.mem.set_u8(block + INTERFACE_SORT, 0);
        block
    };
    e.call(BACK_TO_FRONT_COPY_MEMBERS, &args![this, clone, cloning]);
    let hint = e.mem.u8(this.addr() + OBSERVE_NO_SORT_HINT);
    e.mem.set_u8(clone.wrapping_add(OBSERVE_NO_SORT_HINT), hint);
    let closest = e.mem.u8(this.addr() + SORT_BY_CLOSEST_POINT);
    e.mem
        .set_u8(clone.wrapping_add(SORT_BY_CLOSEST_POINT), closest);
    Ptr::new(clone)
}

// Translated from 00a9b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::RegisterObjectArray` (Xbox PDB; thiscall, `RET 4`: the geometry
/// array, its elements at +0 and count at +4): for each geometry (the count read once, the
/// elements re-read), one that sorts is appended to `m_kItems` (a node from `0043a010`
/// holding it, linked after the tail, the count raised), any other is drawn at once
/// (its virtual slot +0xdc with the renderer read on entry).
pub fn ni_alpha_accumulator_register_object_array(e: &mut Engine, this: Ptr, array: Ptr) {
    let base = this.addr();
    let renderer = e.mem.u32(RENDERER);
    let count = e.mem.u32(array.addr() + 4);
    let mut index = 0u32;
    while index < count {
        let elements = e.mem.u32(array.addr());
        let geometry = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if sorts(e, base, geometry) {
            // `m_kItems`: head +0xc, tail +0x10, count +0x14.
            let count_word = base + ITEMS + 8;
            let node = e.call(LIST_NEW_NODE, &args![count_word]).u32();
            e.mem.set_u32(node + 8, geometry);
            e.mem.set_u32(node, 0);
            let tail = e.mem.u32(base + ITEMS + 4);
            e.mem.set_u32(node + 4, tail);
            let tail = e.mem.u32(base + ITEMS + 4);
            let total = e.mem.u32(count_word).wrapping_add(1);
            if tail != 0 {
                e.mem.set_u32(tail, node);
                e.mem.set_u32(count_word, total);
                e.mem.set_u32(base + ITEMS + 4, node);
            } else {
                e.mem.set_u32(count_word, total);
                e.mem.set_u32(base + ITEMS, node);
                e.mem.set_u32(base + ITEMS + 4, node);
            }
        } else {
            e.vcall(geometry, GEOMETRY_SLOT_RENDER, &args![renderer]);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00a9b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::RegisterObject` (Xbox PDB; thiscall, `RET 4`: geometry): a geometry
/// that sorts is added at the tail of `m_kItems` (`00b58570` with the address of the
/// argument word), any other is drawn at once (its virtual slot +0xdc with the renderer
/// read on entry). Answers true.
pub fn ni_alpha_accumulator_register_object(e: &mut Engine, this: Ptr, geometry: Ptr) -> bool {
    let renderer = e.mem.u32(RENDERER);
    if sorts(e, this.addr(), geometry.addr()) {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), geometry.addr());
            e.call(LIST_ADD_TAIL, &args![this.addr() + ITEMS, argument]);
        });
        return true;
    }
    e.vcall(geometry.addr(), GEOMETRY_SLOT_RENDER, &args![renderer]);
    true
}

/// `NiAccumulator::m_pkCamera` (Xbox PDB) +8.
const CAMERA: u32 = 0x8;
/// `NiBackToFrontAccumulator::m_kItems` (Xbox PDB) +0xC: the inline list.
const ITEMS: u32 = 0xc;
/// `NiBackToFrontAccumulator::m_kItemsToSortAndRender` (Xbox PDB) +0x18.
const ITEMS_TO_SORT: u32 = 0x18;
/// `NiBackToFrontAccumulator::m_iNumItems` (Xbox PDB) +0x1C.
const NUM_ITEMS: u32 = 0x1c;
/// `NiBackToFrontAccumulator::m_iMaxItems` (Xbox PDB) +0x20.
const MAX_ITEMS: u32 = 0x20;
/// `NiBackToFrontAccumulator::m_ppkItems` (Xbox PDB) +0x24.
const ITEM_ARRAY: u32 = 0x24;
/// `NiBackToFrontAccumulator::m_pfDepths` (Xbox PDB) +0x28.
const DEPTHS: u32 = 0x28;
/// `NiAlphaAccumulator::m_bSortByClosestPoint` (Xbox PDB) +0x31.
const SORT_BY_CLOSEST_POINT: u32 = 0x31;
/// `NiAlphaAccumulator::m_bInterfaceSort` (Xbox PDB) +0x32.
const INTERFACE_SORT: u32 = 0x32;
/// The camera's world direction (`NiCamera`, `NiAVObject::m_kWorld`'s
/// rotation column the code reads at +0x68, +0x74, +0x80).
const CAMERA_DIRECTION_X: u32 = 0x68;
const CAMERA_DIRECTION_Y: u32 = 0x74;
const CAMERA_DIRECTION_Z: u32 = 0x80;
/// The geometry's world bound pointer at +0x20 (`NiAVObject`), and the
/// float at +0x90 that the interface sort uses as the depth.
const GEOMETRY_BOUND: u32 = 0x20;
const GEOMETRY_INTERFACE_DEPTH: u32 = 0x90;
/// The bound used when a geometry has none (`NiBound`: centre, radius).
const DEFAULT_BOUND: u32 = 0x011f_4288;
/// `NiFree` (`00aa10f0`, `cdecl`) and `NiAlloc` (`00aa1070`, `cdecl`).
const NI_FREE: u32 = 0x00aa_10f0;
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiBackToFrontAccumulator::SortObjectsByDepth` (`00aa0b40`, `RET 8`).
const SORT_OBJECTS_BY_DEPTH: u32 = 0x00aa_0b40;

// Translated from 00a9b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAlphaAccumulator::Sort` (Xbox PDB): points `m_kItemsToSortAndRender`
/// at the own list when it is null, takes its count as `m_iNumItems`, and
/// with items: grows the item and depth arrays to the count when it is above
/// `m_iMaxItems` (signed; both freed and allocated again), copies the list's
/// geometries into the item array in order, gives each a depth and sorts
/// `[0, count - 1]` with `SortObjectsByDepth`. The depth is the geometry's
/// float at +0x90 for an interface sort; otherwise the world bound's centre
/// (the default bound when null) dotted with the camera's direction
/// (`x*dx`, `y*dy`, `z*dz`, added as `(y*dy + x*dx) + z*dz`, rounded to a
/// float), less the bound's radius when sorting by closest point. A
/// non-positive count copies nothing but still sorts.
pub fn ni_alpha_accumulator_sort(e: &mut Engine, this: Ptr) {
    let at = |offset: u32| this.addr().wrapping_add(offset);
    if e.mem.u32(at(ITEMS_TO_SORT)) == 0 {
        e.mem.set_u32(at(ITEMS_TO_SORT), at(ITEMS));
    }
    let list = e.mem.u32(at(ITEMS_TO_SORT));
    // NiTPointerList::m_uiCount +8.
    let count = e.mem.i32(list.wrapping_add(8));
    e.mem.set_i32(at(NUM_ITEMS), count);
    if count == 0 {
        return;
    }
    if count > e.mem.i32(at(MAX_ITEMS)) {
        let items = e.mem.u32(at(ITEM_ARRAY));
        e.call(NI_FREE, &args![items]);
        let count = e.mem.u32(at(NUM_ITEMS));
        e.mem.set_u32(at(MAX_ITEMS), count);
        let items = e.call(NI_ALLOC, &args![count.wrapping_mul(4)]).u32();
        let depths = e.mem.u32(at(DEPTHS));
        e.mem.set_u32(at(ITEM_ARRAY), items);
        e.call(NI_FREE, &args![depths]);
        let max = e.mem.u32(at(MAX_ITEMS));
        let depths = e.call(NI_ALLOC, &args![max.wrapping_mul(4)]).u32();
        e.mem.set_u32(at(DEPTHS), depths);
    }
    let camera = e.mem.u32(at(CAMERA));
    let direction_x = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_X)));
    // NiTPointerList::m_pkHead +0.
    let mut node = e.mem.u32(e.mem.u32(at(ITEMS_TO_SORT)));
    let direction_y = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_Y)));
    let direction_z = x87_load32(e.mem.u32(camera.wrapping_add(CAMERA_DIRECTION_Z)));
    let interface_sort = e.mem.u8(at(INTERFACE_SORT)) != 0;
    let closest_point = e.mem.u8(at(SORT_BY_CLOSEST_POINT)) != 0;
    let mut index = 0i32;
    while index < e.mem.i32(at(NUM_ITEMS)) {
        let slot = (index as u32).wrapping_mul(4);
        // NiTListItem: next +0, element +8.
        let element = e.mem.u32(node.wrapping_add(8));
        node = e.mem.u32(node);
        let items = e.mem.u32(at(ITEM_ARRAY));
        e.mem.set_u32(items.wrapping_add(slot), element);
        let geometry = e.mem.u32(e.mem.u32(at(ITEM_ARRAY)).wrapping_add(slot));
        let depth = if interface_sort {
            x87_quiet_bits(e.mem.u32(geometry.wrapping_add(GEOMETRY_INTERFACE_DEPTH)))
        } else {
            let mut bound = e.mem.u32(geometry.wrapping_add(GEOMETRY_BOUND));
            if bound == 0 {
                bound = DEFAULT_BOUND;
            }
            let y = x87_mul(x87_load32(e.mem.u32(bound.wrapping_add(4))), direction_y);
            let x = x87_mul(x87_load32(e.mem.u32(bound)), direction_x);
            let sum = x87_add(y, x);
            let z = x87_mul(x87_load32(e.mem.u32(bound.wrapping_add(8))), direction_z);
            let dot = x87_store32(x87_add(sum, z));
            if closest_point {
                let radius = x87_m32(e.mem.u32(bound.wrapping_add(0xc)));
                x87_store32(x87_sub(x87_load32(dot), radius))
            } else {
                dot
            }
        };
        let depths = e.mem.u32(at(DEPTHS));
        e.mem.set_u32(depths.wrapping_add(slot), depth);
        index += 1;
    }
    let last = e.mem.u32(at(NUM_ITEMS)).wrapping_sub(1);
    e.call(SORT_OBJECTS_BY_DEPTH, &args![this, 0u32, last]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_alpha_accumulator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9b530, &args![]).u32(),
            NI_ALPHA_ACCUMULATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a9b4f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b4f0;
        const TARGET: u32 = 0x00aa_0ac0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a9b500_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_b500;
        const TARGET: u32 = 0x00aa_0ad0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
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

    /// An accumulator with two geometries in its own list, the camera looking
    /// along (1, 2, 3).
    fn setup(e: &mut Engine, interface: bool, closest: bool) -> (u32, [u32; 2]) {
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(NI_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(SORT_OBJECTS_BY_DEPTH, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        let camera = e.mem.alloc(0x90);
        e.mem.set_f32(camera + 0x68, 1.0);
        e.mem.set_f32(camera + 0x74, 2.0);
        e.mem.set_f32(camera + 0x80, 3.0);
        e.mem.set_u32(this + 8, camera);
        e.mem.set_u8(this + 0x31, u8::from(closest));
        e.mem.set_u8(this + 0x32, u8::from(interface));
        let bound = e.mem.alloc(0x10);
        for (i, v) in [4.0f32, 5.0, 6.0, 0.5].iter().enumerate() {
            e.mem.set_f32(bound + 4 * i as u32, *v);
        }
        let with_bound = e.mem.alloc(0x94);
        e.mem.set_u32(with_bound + 0x20, bound);
        e.mem.set_f32(with_bound + 0x90, 7.0);
        let without_bound = e.mem.alloc(0x94);
        e.mem.set_f32(without_bound + 0x90, 8.0);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, without_bound);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, with_bound);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(this + 0x14, 2);
        // The default bound: centre (1, 1, 1), radius 2.
        e.map(DEFAULT_BOUND, 0x10);
        for (i, v) in [1.0f32, 1.0, 1.0, 2.0].iter().enumerate() {
            e.mem.set_f32(DEFAULT_BOUND + 4 * i as u32, *v);
        }
        (this, [with_bound, without_bound])
    }

    #[test]
    fn sort_gives_each_item_its_depth_and_sorts_them() {
        // (interface, closest point, expected depths)
        for (interface, closest, expected) in [
            (false, false, [32.0f32, 6.0]),
            (false, true, [31.5, 4.0]),
            (true, false, [7.0, 8.0]),
        ] {
            let mut e = Engine::new();
            let (this, geometries) = setup(&mut e, interface, closest);
            e.call_log = Some(vec![]);
            e.call(0x00a9b570, &args![this]);
            assert_eq!(e.mem.u32(this + 0x18), this + 0xc);
            assert_eq!(e.mem.u32(this + 0x1c), 2);
            assert_eq!(e.mem.u32(this + 0x20), 2);
            let items = e.mem.u32(this + 0x24);
            assert_eq!([e.mem.u32(items), e.mem.u32(items + 4)], geometries);
            let depths = e.mem.u32(this + 0x28);
            assert_eq!([e.mem.f32(depths), e.mem.f32(depths + 4)], expected);
            assert_eq!(calls(&e, NI_ALLOC), vec![vec![8], vec![8]]);
            assert_eq!(calls(&e, SORT_OBJECTS_BY_DEPTH), vec![vec![this, 0, 1]]);
        }
    }

    #[test]
    fn sort_keeps_big_enough_arrays_and_stops_on_an_empty_list() {
        let mut e = Engine::new();
        let (this, _) = setup(&mut e, true, false);
        let items = e.mem.alloc(0x10);
        let depths = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x20, 4);
        e.mem.set_u32(this + 0x24, items);
        e.mem.set_u32(this + 0x28, depths);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert!(calls(&e, NI_ALLOC).is_empty());
        assert_eq!(e.mem.u32(this + 0x24), items);
        assert_eq!(e.mem.f32(depths + 4), 8.0);

        // An empty list: no sort at all.
        e.mem.set_u32(this + 0x14, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert!(calls(&e, SORT_OBJECTS_BY_DEPTH).is_empty());

        // A negative count copies nothing and still sorts.
        e.mem.set_i32(this + 0x14, -1);
        e.call_log = Some(vec![]);
        e.call(0x00a9b570, &args![this]);
        assert_eq!(
            calls(&e, SORT_OBJECTS_BY_DEPTH),
            vec![vec![this, 0, 0xffff_fffe]]
        );
    }
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    const RENDER_TARGET: u32 = 0x03a0_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with a renderer word and a geometry vtable whose slot +0xdc is a double.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(RENDER_TARGET, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + GEOMETRY_SLOT_RENDER, RENDER_TARGET);
        e.mem.map(RENDERER, 4);
        e.mem.set_u32(RENDERER, 0x7777);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    /// A geometry with the given sort word (+0x9c, +0x18) and flags word (+0x30).
    fn geometry(e: &mut Engine, vtable: u32, sort_word: u16, flags: u32) -> u32 {
        let geometry = e.mem.alloc(0xa0);
        e.mem.set_u32(geometry, vtable);
        let owner = e.mem.alloc(0x20);
        e.mem.set_u16(owner + 0x18, sort_word);
        e.mem.set_u32(geometry + GEOMETRY_SORT_FLAGS_OWNER, owner);
        e.mem.set_u32(geometry + GEOMETRY_FLAGS, flags);
        geometry
    }

    #[test]
    fn is_equal_answers_the_base_byte() {
        let mut e = Engine::new();
        e.register(BACK_TO_FRONT_IS_EQUAL, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        assert!(e.call(0x00a9_b510, &args![0x1000u32, 0x2001u32]).bool());
        assert!(!e.call(0x00a9_b510, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_on_request() {
        let mut e = Engine::new();
        e.register(BACK_TO_FRONT_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b540, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_ALPHA_ACCUMULATOR_VTABLE);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00a9_b540, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_FREE_SIZED), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, BACK_TO_FRONT_DESTRUCT).len(), 2);
    }

    #[test]
    fn create_clone_copies_the_hint_and_the_closest_point_flag() {
        let mut e = Engine::new();
        e.register(BACK_TO_FRONT_CONSTRUCT, |_, _| Ret::default());
        e.register(BACK_TO_FRONT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.set_u8(block + INTERFACE_SORT, 9);
            block.into_ret()
        });
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + OBSERVE_NO_SORT_HINT, 0);
        e.mem.set_u8(this + SORT_BY_CLOSEST_POINT, 1);
        e.mem.set_u8(this + INTERFACE_SORT, 1);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a9_b730, &args![this, 0x6000u32]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, BACK_TO_FRONT_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            calls_to(&e, BACK_TO_FRONT_COPY_MEMBERS),
            vec![vec![this, clone, 0x6000]]
        );
        assert_eq!(e.mem.u32(clone), NI_ALPHA_ACCUMULATOR_VTABLE);
        assert_eq!(e.mem.u8(clone + OBSERVE_NO_SORT_HINT), 0);
        assert_eq!(e.mem.u8(clone + SORT_BY_CLOSEST_POINT), 1);
        // The interface flag is cleared by the construction, not copied.
        assert_eq!(e.mem.u8(clone + INTERFACE_SORT), 0);
    }

    #[test]
    fn register_object_array_sorts_some_and_draws_the_rest() {
        let (mut e, vtable) = engine();
        e.register(LIST_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + OBSERVE_NO_SORT_HINT, 1);
        let sorted_a = geometry(&mut e, vtable, 1, 0);
        let not_sortable = geometry(&mut e, vtable, 0, 0);
        let hinted = geometry(&mut e, vtable, 0x2001, 0);
        let bit6 = geometry(&mut e, vtable, 1, 0x40);
        let sorted_b = geometry(&mut e, vtable, 1, 0xffff_ffbf);
        let elements = e.mem.alloc(20);
        for (i, g) in [sorted_a, not_sortable, hinted, bit6, sorted_b]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(elements + 4 * i as u32, *g);
        }
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, elements);
        e.mem.set_u32(array + 4, 5);
        e.call(0x00a9_b790, &args![this, array]);
        let drawn: Vec<Vec<u32>> = calls_to(&e, RENDER_TARGET);
        assert_eq!(
            drawn,
            vec![
                vec![not_sortable, 0x7777],
                vec![hinted, 0x7777],
                vec![bit6, 0x7777]
            ]
        );
        assert_eq!(e.mem.u32(this + ITEMS + 8), 2);
        let head = e.mem.u32(this + ITEMS);
        let tail = e.mem.u32(this + ITEMS + 4);
        assert_eq!(
            (e.mem.u32(head + 8), e.mem.u32(tail + 8)),
            (sorted_a, sorted_b)
        );
        assert_eq!((e.mem.u32(head), e.mem.u32(tail + 4)), (tail, head));
        assert_eq!(e.mem.u32(tail), 0);
        assert!(calls_to(&e, LIST_NEW_NODE)
            .iter()
            .all(|c| c[0] == this + 0x14));
        // Without the hint observed, bit 13 does not matter.
        e.mem.set_u8(this + OBSERVE_NO_SORT_HINT, 0);
        e.mem.set_u32(array + 4, 3);
        e.call(0x00a9_b790, &args![this, array]);
        assert_eq!(e.mem.u32(this + ITEMS + 8), 4);
    }

    #[test]
    fn register_object_adds_or_draws_and_answers_true() {
        let (mut e, vtable) = engine();
        e.register(LIST_ADD_TAIL, |e, a| {
            let element = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], element);
            Ret::default()
        });
        let this = e.mem.alloc(0x34);
        let sorted = geometry(&mut e, vtable, 1, 0);
        let drawn = geometry(&mut e, vtable, 1, 0x40);
        assert!(e.call(0x00a9_b850, &args![this, sorted]).bool());
        assert_eq!(calls_to(&e, LIST_ADD_TAIL)[0][0], this + ITEMS);
        assert_eq!(e.mem.u32(this + ITEMS), sorted);
        assert!(e.call(0x00a9_b850, &args![this, drawn]).bool());
        assert_eq!(calls_to(&e, RENDER_TARGET), vec![vec![drawn, 0x7777]]);
        assert_eq!(calls_to(&e, LIST_ADD_TAIL).len(), 1);
    }
}
