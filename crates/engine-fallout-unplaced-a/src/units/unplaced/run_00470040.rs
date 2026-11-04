//! `(unplaced)/run_00470040` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The chained container: a head value at `+0` and a singly linked chain of
//! 8-byte nodes (value at `+0`, next node at `+4`) whose first node is at
//! `+4` of the container. Each node is itself such a container (its `+4`
//! word is the rest of the chain), so a node's destructor empties the chain
//! behind it.

#[allow(unused_imports)]
use crate::prelude::*;

/// Destructor body of the container (`0046ffb0`): empties it (`00470470`).
const CONTAINER_DESTRUCT: u32 = 0x0046_ffb0;
/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 004702f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Container destructor with the compiler's "scalar deleting" flag
/// (`RET 4`): destroys the contents (`0046ffb0`) and, when bit 0 of `flags`
/// is set, deletes the memory. Returns `this`.
pub fn fn_004702f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(CONTAINER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00470470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the container: detaches each chained node from the one after it,
/// destroys and deletes it (`004702f0` with the delete flag), and finally
/// zeroes the head value at `+0`.
pub fn fn_00470470(e: &mut Engine, this: Ptr) {
    loop {
        let node = e.mem.u32(this.addr() + 4);
        if node == 0 {
            break;
        }
        let rest = e.mem.u32(node + 4);
        e.mem.set_u32(node + 4, 0);
        // The game re-reads the chain head and tests it again before the
        // call; it is the same node.
        let node = e.mem.u32(this.addr() + 4);
        if node != 0 {
            fn_004702f0(e, Ptr::new(node), 1);
        }
        e.mem.set_u32(this.addr() + 4, rest);
    }
    e.mem.set_u32(this.addr(), 0);
}

// Translated from 00470440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the pointed-to item into a new list node and clears its link.
/// The source is read before either store, including when it aliases a field.
pub fn construct_node(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let item = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr(), item);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    this
}

/// A hash map's vtable slots: the bucket of a key (+4), whether two keys are
/// equal (+8), filling a new node (+0xC); and the node allocator
/// (`00844a70`, `thiscall` on the map).
const MAP_HASH_SLOT: u32 = 0x4;
const MAP_EQUAL_SLOT: u32 = 0x8;
const MAP_SET_NODE_SLOT: u32 = 0xc;
const MAP_NEW_NODE: u32 = 0x0084_4a70;

// Translated from 00470200 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A hash map's `SetAt` (`thiscall`, `RET 8`: key, value). The map: vtable,
/// bucket count (+4), bucket array (+8), entry count (+0xC), copies-keys flag
/// (byte +0x10); a node: next (+0), key (+4), value (+8). When a node of the
/// key's bucket has an equal key, its value is replaced (and its key too
/// when the map doesn't copy keys); otherwise a new node (`00844a70`, filled
/// through slot +0xC) goes at the head of the bucket and the count grows.
pub fn fn_00470200(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, MAP_HASH_SLOT, &args![key]).u32();
    let mut node = e
        .mem
        .u32(e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let stored = e.mem.u32(node + 4);
        let equal = e.vcall(map, MAP_EQUAL_SLOT, &args![key, stored]).u32() as u8;
        if equal != 0 {
            if e.mem.u8(map + 0x10) == 0 {
                e.mem.set_u32(node + 4, key);
            }
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.call(MAP_NEW_NODE, &args![this]).u32();
    e.vcall(map, MAP_SET_NODE_SLOT, &args![node, key, value]);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let slot = e.mem.u32(map + 8).wrapping_add(bucket.wrapping_mul(4));
    e.mem.set_u32(slot, node);
    let count = e.mem.u32(map + 0xc).wrapping_add(1);
    e.mem.set_u32(map + 0xc, count);
}

// Translated from 00470320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0046ffd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470320(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0046_ffd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0045d930` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470350(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0045_d930, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00470070` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0047_0070, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004703b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `0045d950` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004703b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0045_d950, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004703e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00470110` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004703e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0047_0110, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00470410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00464f40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00470410(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0046_4f40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00470200, fn_00470200(Ptr, u32, u32)),
        entry!(0x00470440, construct_node(Ptr, Ptr) -> Ptr),
        entry!(0x004702f0, fn_004702f0(Ptr, u32) -> Ptr),
        entry!(0x00470470, fn_00470470(Ptr)),
        entry!(0x0047_0320, fn_00470320(Ptr, u32) -> Ptr),
        entry!(0x0047_0350, fn_00470350(Ptr, u32) -> Ptr),
        entry!(0x0047_0380, fn_00470380(Ptr, u32) -> Ptr),
        entry!(0x0047_03b0, fn_004703b0(Ptr, u32) -> Ptr),
        entry!(0x0047_03e0, fn_004703e0(Ptr, u32) -> Ptr),
        entry!(0x0047_0410, fn_00470410(Ptr, u32) -> Ptr),
        entry!(0x0047_0040, fn_00470040(Ptr, u16, u16) -> Ptr),
        entry!(0x0047_0070, fn_00470070(Ptr)),
        entry!(0x0047_00a0, fn_004700a0(Ptr, u16, u16) -> Ptr),
        entry!(0x0047_0110, fn_00470110(Ptr)),
        entry!(0x0047_0140, fn_00470140(Ptr, Ptr) -> u32),
        entry!(0x0047_04e0, fn_004704e0(Ptr, u16, u16) -> Ptr),
        entry!(0x0047_0550, fn_00470550(Ptr, u16, u16) -> Ptr),
        entry!(0x0047_05c0, fn_004705c0(Ptr) -> u32),
        entry!(0x0047_00d0, fn_004700d0(Ptr, Ptr)),
    ]
}

/// `NiTPrimitiveArray<TESObjectCELL*>`'s vtable (RTTI).
const CELL_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0101_9a1c;
/// `NiTArray<TESObjectCELL*,NiTMallocInterface<..>>`'s vtable (RTTI).
const CELL_ARRAY_VTABLE: u32 = 0x0101_9a14;
/// `NiTArray<BGSAddonNode*,NiTMallocInterface<..>>`'s vtable (RTTI).
const ADDON_NODE_ARRAY_VTABLE: u32 = 0x0101_9a24;
/// `NiTPrimitiveArray<BGSAddonNode*>`'s vtable (RTTI).
const ADDON_NODE_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0101_9a2c;
/// `NiTLargeArray<TESForm*,NiTMallocInterface<..>>`'s vtable (RTTI).
const FORM_LARGE_ARRAY_VTABLE: u32 = 0x0101_9a34;
/// `NiTMallocInterface` allocation of `count` elements (`0096afc0`, cdecl).
const MALLOC_ALLOCATE: u32 = 0x0096_afc0;
/// `NiTMallocInterface` release (`004ede70`, cdecl(block)).
const MALLOC_DEALLOCATE: u32 = 0x004e_de70;
/// `NiTLargeArray::SetAtGrow` shape (`00864240`, thiscall(index, item
/// address)).
const LARGE_ARRAY_SET_AT_GROW: u32 = 0x0086_4240;
/// `006f99e0` (`bgsdistanttreeblock.cpp`, thiscall), on `this + 8`.
const CALL_6F99E0: u32 = 0x006f_99e0;

/// The `NiTArray` constructor body `004704e0` and `00470550` share (each
/// with its own vtable): `m_usMaxSize` and `m_usGrowBy` from the arguments,
/// size and used size cleared, and `m_pBase` allocated for `max_size`
/// elements (null for 0).
fn ni_t_array_construct(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    max_size: u16,
    grow_by: u16,
) -> Ptr {
    let array: Ptr<crate::types::NiTArray> = this.cast();
    e.mem.set_u32(this.addr(), vtable);
    e.set(array, crate::types::NiTArray::m_usMaxSize, max_size);
    e.set(array, crate::types::NiTArray::m_usGrowBy, grow_by);
    e.set(array, crate::types::NiTArray::m_usSize, 0);
    e.set(array, crate::types::NiTArray::m_usESize, 0);
    let max = e.get(array, crate::types::NiTArray::m_usMaxSize);
    if max as i32 > 0 {
        let base = e.call(MALLOC_ALLOCATE, &args![u32::from(max)]).u32();
        e.set(array, crate::types::NiTArray::m_pBase, base);
    } else {
        e.set(array, crate::types::NiTArray::m_pBase, 0);
    }
    this
}

// Translated from 00470040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<TESObjectCELL*>`'s constructor: the `NiTArray` one
/// (`004704e0`) with `max_size` and `grow_by`, then its own vtable.
/// Returns `this`.
pub fn fn_00470040(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    fn_004704e0(e, this, max_size, grow_by);
    e.mem.set_u32(this.addr(), CELL_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00470070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `NiTArray<BGSAddonNode*,..>` (its vtable stored
/// again): releases `m_pBase` (`004ede70`).
pub fn fn_00470070(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ADDON_NODE_ARRAY_VTABLE);
    let base = e.mem.u32(this.addr() + 4);
    e.call(MALLOC_DEALLOCATE, &args![base]);
}

// Translated from 004700a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<BGSAddonNode*>`'s constructor: the `NiTArray` one
/// (`00470550`), then its own vtable. Returns `this`.
pub fn fn_004700a0(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    fn_00470550(e, this, max_size, grow_by);
    e.mem
        .set_u32(this.addr(), ADDON_NODE_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00470110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `NiTLargeArray<TESForm*,..>` (its vtable stored
/// again): releases the element block at `+4` (`004ede70`).
pub fn fn_00470110(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FORM_LARGE_ARRAY_VTABLE);
    let base = e.mem.u32(this.addr() + 4);
    e.call(MALLOC_DEALLOCATE, &args![base]);
}

// Translated from 00470140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `*item` to the first empty slot of the large array (block `+4`,
/// size `+0xC`, used count `+0x10`): -1 for a null item; the slot index
/// after storing it and counting it; when no slot is empty, the result of
/// `00864240(size, item)` (adding it at the end).
pub fn fn_00470140(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let value = e.mem.u32(item.addr());
    if value == 0 {
        return u32::MAX;
    }
    let base = this.addr();
    let mut index = 0u32;
    while index < e.mem.u32(base + 0xc) {
        let slot = e.mem.u32(base + 4).wrapping_add(index.wrapping_mul(4));
        if e.mem.u32(slot) == 0 {
            let value = e.mem.u32(item.addr());
            e.mem.set_u32(slot, value);
            let used = e.mem.u32(base + 0x10).wrapping_add(1);
            e.mem.set_u32(base + 0x10, used);
            return index;
        }
        index = index.wrapping_add(1);
    }
    let size = e.mem.u32(base + 0xc);
    e.call(LARGE_ARRAY_SET_AT_GROW, &args![this, size, item])
        .u32()
}

// Translated from 004704e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<TESObjectCELL*,NiTMallocInterface<..>>`'s constructor
/// (thiscall(max size, grow by)). Returns `this`.
pub fn fn_004704e0(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    ni_t_array_construct(e, this, CELL_ARRAY_VTABLE, max_size, grow_by)
}

// Translated from 00470550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTArray<BGSAddonNode*,NiTMallocInterface<..>>`'s constructor
/// (thiscall(max size, grow by)). Returns `this`.
pub fn fn_00470550(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    ni_t_array_construct(e, this, ADDON_NODE_ARRAY_VTABLE, max_size, grow_by)
}

// Translated from 004705c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `006f99e0` on `this + 8`, its result passed through (`EAX`).
pub fn fn_004705c0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALL_6F99E0, &args![this.addr().wrapping_add(8)])
        .u32()
}

/// The list's head insertion (`0049fa20`, unnamed; `thiscall`, the node).
const ADD_NODE_HEAD: u32 = 0x0049_fa20;

// Translated from 004700d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): makes a node ([`fn_004705c0`] on
/// `this`), stores the word at `item` in the node's +8 and puts the node at the head of the
/// list (`0049fa20`).
pub fn fn_004700d0(e: &mut Engine, this: Ptr, item: Ptr) {
    let node = fn_004705c0(e, this);
    let value = e.mem.u32(item.addr());
    e.mem.set_u32(node.wrapping_add(8), value);
    e.call(ADD_NODE_HEAD, &args![this, node]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_at_adds_a_node_then_replaces_its_value() {
        let mut e = Engine::new();
        // Hash: the key itself; equal: same word; set node: key and value.
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000, 0x0090_2000, 0x0090_3000]);
        e.register(0x0090_1000, |_, a| (a[1] % 4).into_ret());
        e.register(0x0090_2000, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(0x0090_3000, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(MAP_NEW_NODE, |e, _| e.mem.alloc(0xc).into_ret());
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(16);
        e.mem.set_u32(map, 0x0090_0000);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, buckets);
        e.call(0x00470200, &[map, 6, 100]);
        e.call(0x00470200, &[map, 10, 200]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        // Both in bucket 2, the newer at the head.
        let head = e.mem.u32(buckets + 8);
        assert_eq!(e.mem.u32(head + 4), 10);
        let next = e.mem.u32(head);
        assert_eq!(e.mem.u32(next + 4), 6);
        e.call(0x00470200, &[map, 6, 300]);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        assert_eq!(e.mem.u32(next + 8), 300);
    }

    #[test]
    fn constructor_copies_before_clearing_an_aliased_source_and_keeps_guards() {
        let mut e = Engine::new();
        let object = e.mem.alloc(16);
        e.mem.write(object, &[0xaa; 16]);
        e.mem.set_u32(object + 8, 0x12345678);
        assert_eq!(
            e.call(0x00470440, &args![object + 4, object + 8]).u32(),
            object + 4
        );
        assert_eq!(e.mem.u32(object), 0xaaaaaaaa);
        assert_eq!(e.mem.u32(object + 4), 0x12345678);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 12), 0xaaaaaaaa);
    }

    fn doubles(e: &mut Engine) {
        e.register(CONTAINER_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
    }

    #[test]
    fn destructor_without_the_delete_flag_keeps_the_memory() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        let back = e.call(0x0047_02f0, &args![0x1000u32, 0u32]).u32();
        assert_eq!(back, 0x1000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (CONTAINER_DESTRUCT, vec![0x1000]));
    }

    #[test]
    fn destructor_with_the_delete_flag_deletes() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.call_log = Some(vec![]);
        // Only bit 0 of the flags counts.
        e.call(0x0047_02f0, &args![0x1000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (OPERATOR_DELETE, vec![0x1000]));
        e.call_log = Some(vec![]);
        e.call(0x0047_02f0, &args![0x1000u32, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn clearing_destroys_each_node_in_chain_order_and_zeroes_the_head() {
        let mut e = Engine::new();
        doubles(&mut e);
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(container.addr(), 77);
        e.mem.set_u32(container.addr() + 4, first);
        e.mem.set_u32(first + 4, second);
        e.call_log = Some(vec![]);
        e.call(0x0047_0470, &args![container]);
        let log = e.call_log.take().unwrap();
        let destroyed: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == CONTAINER_DESTRUCT)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(destroyed, vec![first, second]);
        // Each node was detached from its successor before it was destroyed.
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(container.addr() + 4), 0);
        assert_eq!(e.mem.u32(container.addr()), 0);
    }

    #[test]
    fn clearing_an_empty_container_only_zeroes_the_head() {
        let mut e = Engine::new();
        doubles(&mut e);
        let container: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(container.addr(), 77);
        e.call_log = Some(vec![]);
        e.call(0x0047_0470, &args![container]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(container.addr()), 0);
    }
    #[test]
    fn at_00470320_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0046_ffd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0320, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0320)
                .collect();
            let mut expected = vec![(0x0046_ffd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470350_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0045_d930, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0350, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0350)
                .collect();
            let mut expected = vec![(0x0045_d930, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470380_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0047_0070, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0380, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0380)
                .collect();
            let mut expected = vec![(0x0047_0070, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004703b0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0045_d950, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_03b0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_03b0)
                .collect();
            let mut expected = vec![(0x0045_d950, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004703e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0047_0110, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_03e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_03e0)
                .collect();
            let mut expected = vec![(0x0047_0110, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00470410_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0046_4f40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_0410, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0047_0410)
                .collect();
            let mut expected = vec![(0x0046_4f40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b016_tests {
    use super::*;
    use crate::types::NiTArray;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(MALLOC_ALLOCATE, |_, a| (0x0010_0000 + a[0]).into_ret());
        e.register(MALLOC_DEALLOCATE, |_, _| Ret::default());
        e.register(LARGE_ARRAY_SET_AT_GROW, |_, a| (a[1] + 0x100).into_ret());
        e.register(CALL_6F99E0, |_, a| (a[0] ^ 1).into_ret());
        e
    }

    #[test]
    fn array_constructors_store_sizes_and_vtables() {
        for (address, vtable) in [
            (0x0047_0040u32, CELL_PRIMITIVE_ARRAY_VTABLE),
            (0x0047_00a0, ADDON_NODE_PRIMITIVE_ARRAY_VTABLE),
            (0x0047_04e0, CELL_ARRAY_VTABLE),
            (0x0047_0550, ADDON_NODE_ARRAY_VTABLE),
        ] {
            let mut e = engine();
            let array: Ptr<NiTArray> = e.new_object();
            e.set(array, NiTArray::m_usSize, 9);
            e.set(array, NiTArray::m_usESize, 9);
            e.call_log = Some(vec![]);
            // Only the low half of each word is the size.
            let back = e
                .call(address, &args![array, 0xffff_0005u32, 0x0003_0002u32])
                .u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(back, array.addr());
            assert_eq!(e.mem.u32(array.addr()), vtable);
            assert_eq!(e.get(array, NiTArray::m_usMaxSize), 5);
            assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
            assert_eq!(e.get(array, NiTArray::m_usSize), 0);
            assert_eq!(e.get(array, NiTArray::m_usESize), 0);
            assert_eq!(e.get(array, NiTArray::m_pBase), 0x0010_0005);
            assert_eq!(calls(&log, MALLOC_ALLOCATE), vec![vec![5]]);
        }
        // A maximum of 0 allocates nothing.
        let mut e = engine();
        let array: Ptr<NiTArray> = e.new_object();
        e.set(array, NiTArray::m_pBase, 7);
        e.call_log = Some(vec![]);
        e.call(0x0047_04e0, &args![array, 0u32, 1u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, MALLOC_ALLOCATE).is_empty());
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
    }

    #[test]
    fn array_destructors_release_the_block() {
        let mut e = engine();
        let array = e.mem.alloc(0x18);
        e.mem.set_u32(array + 4, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0047_0070, &args![array]);
        assert_eq!(e.mem.u32(array), ADDON_NODE_ARRAY_VTABLE);
        e.call(0x0047_0110, &args![array]);
        assert_eq!(e.mem.u32(array), FORM_LARGE_ARRAY_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, MALLOC_DEALLOCATE),
            vec![vec![0x4444], vec![0x4444]]
        );
    }

    #[test]
    fn add_to_first_empty_slot() {
        let mut e = engine();
        let array = e.mem.alloc(0x18);
        let block = e.mem.alloc(12);
        e.mem.set_u32(block, 0x11);
        e.mem.set_u32(array + 4, block);
        e.mem.set_u32(array + 0xc, 3);
        e.mem.set_u32(array + 0x10, 1);
        let item = e.mem.alloc(4);
        // A null item: -1.
        assert_eq!(e.call(0x0047_0140, &args![array, item]).u32(), u32::MAX);
        e.mem.set_u32(item, 0x22);
        assert_eq!(e.call(0x0047_0140, &args![array, item]).u32(), 1);
        assert_eq!(e.mem.u32(block + 4), 0x22);
        assert_eq!(e.mem.u32(array + 0x10), 2);
        e.mem.set_u32(block + 8, 0x33);
        // No empty slot: added at the end.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_0140, &args![array, item]).u32(), 0x103);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, LARGE_ARRAY_SET_AT_GROW),
            vec![vec![array, 3, item]]
        );
    }

    #[test]
    fn forwarder_passes_this_plus_8() {
        let mut e = engine();
        assert_eq!(e.call(0x0047_05c0, &args![0x1000u32]).u32(), 0x1009);
    }

    #[test]
    fn add_puts_a_new_node_with_the_item_at_the_head() {
        let mut e = Engine::new();
        let node = e.mem.alloc(0xc);
        e.register_double(CALL_6F99E0, move |_, _| node.into_ret());
        e.register(ADD_NODE_HEAD, |_, _| Ret::default());
        let list = e.mem.alloc(0x10);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x5151);
        e.call_log = Some(vec![]);
        e.call(0x0047_00d0, &args![list, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (CALL_6F99E0, vec![list + 8]));
        assert_eq!(log[2], (ADD_NODE_HEAD, vec![list, node]));
        assert_eq!(e.mem.u32(node + 8), 0x5151);
    }
}
