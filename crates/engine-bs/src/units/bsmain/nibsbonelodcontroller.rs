//! `BSMain/nibsbonelodcontroller.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;
use crate::units::nimain::nistream::NiTPrimitiveSet;

/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;

// Translated from 00a311e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiNode*,NiTMallocInterface<NiNode*>>::Realloc` (engine map; `thiscall`,
/// one stack word): changes the capacity of the set of pointers (layout
/// [`NiTPrimitiveSet`]: data, capacity, used count) to `new_capacity`. Does
/// nothing for the current capacity. Otherwise new storage of `new_capacity`
/// pointers (`00aa1070`; none for 0) takes the used elements, the old storage is
/// freed (`00aa10f0`, also when it is null) and the capacity is stored.
pub fn ni_t_set_realloc(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, new_capacity: u32) {
    if new_capacity == e.get(this, NiTPrimitiveSet::m_uiAlloced) {
        return;
    }
    let storage = if new_capacity == 0 {
        0
    } else {
        let storage = e.call(ALLOCATE, &args![new_capacity.wrapping_mul(4)]).u32();
        let old = e.get(this, NiTPrimitiveSet::m_pBase);
        let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
        for index in 0..used {
            let value = e.mem.u32(old.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(storage.wrapping_add(index.wrapping_mul(4)), value);
        }
        storage
    };
    let old = e.get(this, NiTPrimitiveSet::m_pBase);
    e.call(FREE, &args![old]);
    e.set(this, NiTPrimitiveSet::m_pBase, storage);
    e.set(this, NiTPrimitiveSet::m_uiAlloced, new_capacity);
}

// Translated from 00c53e00 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012031b8 without reading the receiver. The map names this body `NiBSBoneLODController::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c53e00(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_31b8
}

/// `00aa13e0`: `operator new` (`cdecl`: size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `00aa1460`: the sized delete (`cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free (`00aa4060`),
/// both `thiscall` with one stack word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// `NiTimeController`'s constructor (`00a6d220`), destructor (`00a6d290`), member copy
/// (`00a6cfe0`; `thiscall` on the source: clone, cloning process), `ProcessClone`
/// (`00a6d0c0`), `LoadBinary` (`00a6d5e0`) and `LinkObject` (`00a6d500`), the last three
/// `thiscall` with one stack word.
const TIME_CONTROLLER_CONSTRUCTOR: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCTOR: u32 = 0x00a6_d290;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
/// The base `PostLinkObject` (`00a89af0`; `thiscall`: stream).
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// The cloning process's map lookup (`00a53c30`; `thiscall` on the map at the cloning
/// process's `+0`: key, address of the result word; answers whether the key was found).
const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiTArray`'s `SetSize` (`00a90180`; `thiscall`: new size) and `SetAt` (`00ba3aa0`;
/// `thiscall`: index, address of the element).
const ARRAY_SET_SIZE: u32 = 0x00a9_0180;
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// Stream helpers: the `cdecl` read of one 32-bit word (`00a26df0`: stream,
/// destination), the link id read (`00a645b0`; `thiscall` on the stream) and the linked
/// object of the next id (`00a63430`; `thiscall` on the stream).
const READ_WORD: u32 = 0x00a2_6df0;
const READ_LINK_ID: u32 = 0x00a6_45b0;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The `NiBSBoneLODController` virtual table (`010c29cc`) and the bone array's virtual
/// tables: the one its constructor stores (`0109a2a4`) and the derived one the controller's
/// constructor stores over it (`0109a2c0`).
const BONE_LOD_CONTROLLER_VTABLE: u32 = 0x010c_29cc;
const ARRAY_BASE_VTABLE: u32 = 0x0109_a2a4;
const BONE_ARRAY_VTABLE: u32 = 0x0109_a2c0;
/// The size of an `NiBSBoneLODController` (0x4C bytes).
const BONE_LOD_CONTROLLER_SIZE: u32 = 0x4c;

layout! {
    /// `NiBSBoneLODController` (Xbox PDB), 0x4C bytes: `NiTimeController`, the LOD and the
    /// bone array (an `NiTPrimitiveArray` of node sets at `+0x3C`).
    pub struct NiBSBoneLODController: 0x4C {
        /// `m_pkTarget` (Xbox PDB, `NiTimeController`).
        0x2C m_pkTarget: u32,
        /// `m_iLOD` (Xbox PDB).
        0x34 m_iLOD: u32,
        /// `m_uiNumLODs` (Xbox PDB).
        0x38 m_uiNumLODs: u32,
        /// `m_kBoneArray.m_pBase` (Xbox PDB): node set pointers.
        0x40 m_pBase: u32,
        /// `m_kBoneArray.m_usMaxSize` (Xbox PDB).
        0x44 m_usMaxSize: u16,
        /// `m_kBoneArray.m_usSize` (Xbox PDB).
        0x46 m_usSize: u16,
        /// `m_kBoneArray.m_usESize` (Xbox PDB).
        0x48 m_usESize: u16,
        /// `m_kBoneArray.m_usGrowBy` (Xbox PDB).
        0x4A m_usGrowBy: u16,
    }
}

/// The node set at index `index` of the bone array.
fn bone_set(e: &Engine, this: Ptr<NiBSBoneLODController>, index: u32) -> u32 {
    let base = e.get(this, NiBSBoneLODController::m_pBase);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

/// A new empty node set: 0xC bytes from the memory manager with the inlined
/// `NiTPrimitiveSet` constructor for an initial size of 0 (its size test, `XOR`/`JBE`,
/// always skips the allocation of the storage, which is kept as written); null when the
/// allocation failed.
fn new_bone_set(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
        .u32();
    if block == 0 {
        return 0;
    }
    let initial_size = 0u32;
    let storage = if initial_size > 0 {
        e.call(ALLOCATE, &args![initial_size << 2]).u32()
    } else {
        0
    };
    e.mem.set_u32(block, storage);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    block
}

/// `m_kBoneArray.SetAtGrow(index, set)` as the exe inlines it on the array at `array`:
/// `SetSize(grow_by + index)` (`00a90180`) when the index is not below the capacity, then
/// `SetAt` (`00ba3aa0`) with the address of a word holding `set`.
fn bone_array_set_at_grow(e: &mut Engine, array: u32, index: u32, set: u32) {
    if index >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(ARRAY_SET_SIZE, &args![array, grow_by.wrapping_add(index)]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), set);
        e.call(ARRAY_SET_AT, &args![array, index, slot]);
    });
}

/// [`ni_t_set_add`] of the word `value` (passed by address, as the exe does).
fn add_to_set(e: &mut Engine, set: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        ni_t_set_add(e, Ptr::new(set), slot);
    });
}

// Translated from 00a54190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node set array's constructor (`NiTPrimitiveArray<NiTPrimitiveSet<NiNode*>*>`,
/// unnamed in the engine map; `thiscall`, `RET 8`: capacity, grow-by, both 16-bit): the
/// array's virtual table (`0109a2a4`), the counts, and storage for `capacity` pointers
/// (`00aa1070`) or none. Answers `this`.
pub fn fn_00a54190(e: &mut Engine, this: Ptr, capacity: u16, grow_by: u16) -> Ptr {
    let array = this.addr();
    e.mem.set_u16(array + 0xe, grow_by);
    e.mem.set_u32(array, ARRAY_BASE_VTABLE);
    e.mem.set_u16(array + 8, capacity);
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    let storage = if capacity != 0 {
        e.call(ALLOCATE, &args![u32::from(capacity) * 4]).u32()
    } else {
        0
    };
    e.mem.set_u32(array + 4, storage);
    this
}

// Translated from 00a543b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTSet<NiNode*,NiTMallocInterface<NiNode*>>::Add` (engine map; `thiscall`, `RET 4`:
/// address of the element): when the set is full its capacity doubles (1 for an empty
/// one) through [`ni_t_set_realloc`]; then the element is appended.
pub fn ni_t_set_add(e: &mut Engine, this: Ptr<NiTPrimitiveSet>, element: Ptr) {
    let capacity = e.get(this, NiTPrimitiveSet::m_uiAlloced);
    if e.get(this, NiTPrimitiveSet::m_uiUsed) == capacity {
        let grown = if capacity == 0 {
            1
        } else {
            capacity.wrapping_add(capacity)
        };
        ni_t_set_realloc(e, this, grown);
    }
    let used = e.get(this, NiTPrimitiveSet::m_uiUsed);
    let base = e.get(this, NiTPrimitiveSet::m_pBase);
    let value = e.mem.u32(element.addr());
    e.mem
        .set_u32(base.wrapping_add(used.wrapping_mul(4)), value);
    e.set(this, NiTPrimitiveSet::m_uiUsed, used.wrapping_add(1));
}

// Translated from 00c52880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): deletes every node set of the
/// bone array (its storage freed by `00aa10f0`, the set by the memory manager; the
/// compiler's `MOV EAX,1; AND EAX,1` test before the second free always passes), then
/// clears every slot below the array's size (16-bit counter) and both the size and the
/// count of set slots.
pub fn fn_00c52880(e: &mut Engine, this: Ptr<NiBSBoneLODController>) {
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            let storage = e.mem.u32(set);
            e.call(FREE, &args![storage]);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, set]);
        }
        index = index.wrapping_add(1);
    }
    let mut slot = 0u16;
    while u32::from(slot) < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let base = e.get(this, NiBSBoneLODController::m_pBase);
        e.mem.set_u32(base.wrapping_add(u32::from(slot) * 4), 0);
        slot = slot.wrapping_add(1);
    }
    e.set(this, NiBSBoneLODController::m_usSize, 0);
    e.set(this, NiBSBoneLODController::m_usESize, 0);
}

// Translated from 00c52960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::SetBoneLOD` (Xbox PDB; `thiscall`, `RET 4`). Refused (false)
/// without LODs, for the current LOD, and for an LOD other than -1 that is negative or not
/// below the LOD count. Otherwise walks the node sets in order; from the set at the
/// requested index on, every node gets flag 2 of its flags word (`NiAVObject` `+0x30`)
/// set, the nodes of the sets before it get it cleared (all cleared for -1). Stores the
/// LOD and answers true.
pub fn ni_bs_bone_lod_controller_set_bone_lod(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    lod: i32,
) -> bool {
    if e.get(this, NiBSBoneLODController::m_uiNumLODs) == 0 {
        return false;
    }
    if e.get(this, NiBSBoneLODController::m_iLOD) == lod as u32 {
        return false;
    }
    if lod != -1 && (lod <= -1 || lod as u32 >= e.get(this, NiBSBoneLODController::m_uiNumLODs)) {
        return false;
    }
    let mut visible = false;
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            if index == lod as u32 {
                visible = true;
            }
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let node = e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4)));
                if node != 0 {
                    // NiAVObject::m_uFlags (Xbox PDB) +0x30.
                    let flags = e.mem.u32(node + 0x30);
                    let flags = if visible { flags | 2 } else { flags & !2 };
                    e.mem.set_u32(node + 0x30, flags);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    e.set(this, NiBSBoneLODController::m_iLOD, lod as u32);
    true
}

// Translated from 00c53d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53d70 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController`'s constructor (unnamed in the engine map; `thiscall`, plain
/// `RET`): `NiTimeController`'s constructor (`00a6d220`), the class's virtual table, LOD -1,
/// no LODs, and the bone array constructed empty with grow-by 1 ([`fn_00a54190`]), its
/// virtual table then replaced by `0109a2c0`. Answers `this`.
pub fn fn_00c53d70(e: &mut Engine, this: Ptr<NiBSBoneLODController>) -> Ptr<NiBSBoneLODController> {
    e.call(TIME_CONTROLLER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), BONE_LOD_CONTROLLER_VTABLE);
    e.set(this, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    e.set(this, NiBSBoneLODController::m_uiNumLODs, 0);
    let array = this.addr() + 0x3c;
    fn_00a54190(e, Ptr::new(array), 0, 1);
    e.mem.set_u32(array, BONE_ARRAY_VTABLE);
    this
}

// Translated from 00c537f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c537f0 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// The class's create function (unnamed in the engine map; plain `RET`): a new 0x4C-byte
/// controller ([`fn_00c53d70`]), null when the allocation fails.
pub fn fn_00c537f0(e: &mut Engine) -> Ptr<NiBSBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![BONE_LOD_CONTROLLER_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_00c53d70(e, Ptr::new(block))
}

// Translated from 00c533f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process): `NiTimeController`'s copy (`00a6cfe0`), then
/// the LOD and the LOD count. The bone sets are rebuilt by `ProcessClone`.
pub fn fn_00c533f0(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    clone: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) {
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, clone, cloning]);
    let lod = e.get(this, NiBSBoneLODController::m_iLOD);
    e.set(clone, NiBSBoneLODController::m_iLOD, lod);
    let count = e.get(this, NiBSBoneLODController::m_uiNumLODs);
    e.set(clone, NiBSBoneLODController::m_uiNumLODs, count);
}

// Translated from 00c53360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53360 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process): a
/// new controller ([`fn_00c53d70`]; null when the allocation fails) filled by
/// [`fn_00c533f0`]. Answers the clone.
pub fn ni_bs_bone_lod_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) -> Ptr<NiBSBoneLODController> {
    let block = e.call(OPERATOR_NEW, &args![BONE_LOD_CONTROLLER_SIZE]).u32();
    let clone = if block == 0 {
        Ptr::new(0)
    } else {
        fn_00c53d70(e, Ptr::new(block))
    };
    fn_00c533f0(e, this, clone, cloning);
    clone
}

// Translated from 00c53430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53430 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::ProcessClone` (Xbox PDB; `thiscall`, `RET 4`: cloning process).
/// `NiTimeController::ProcessClone` (`00a6d0c0`), then the clone looked up in the cloning
/// process's map (`00a53c30`; null when absent). For each slot of the bone array the clone
/// gets a null set (for a null slot) or a new set holding, node by node, the node's clone
/// (null for a null node or one not found), stored with the array's grow-and-set. Last the
/// clone's LOD is applied again: its LOD word set to -1 and `SetBoneLOD` called with the old
/// value.
pub fn ni_bs_bone_lod_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    cloning: Ptr,
) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let clone = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        e.mem.u32(slot.addr())
    });
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        let array = clone.wrapping_add(0x3c);
        if set == 0 {
            bone_array_set_at_grow(e, array, index, 0);
        } else {
            let copy = new_bone_set(e);
            bone_array_set_at_grow(e, array, index, copy);
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let node = e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4)));
                if node == 0 {
                    add_to_set(e, copy, 0);
                } else {
                    let found = e.with_stack(4, |e, slot| {
                        e.mem.set_u32(slot.addr(), 0);
                        let map = e.mem.u32(cloning.addr());
                        if e.call(CLONE_MAP_LOOKUP, &args![map, node, slot]).bool() {
                            e.mem.u32(slot.addr())
                        } else {
                            0
                        }
                    });
                    add_to_set(e, copy, found);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    let clone = Ptr::<NiBSBoneLODController>::new(clone);
    let lod = e.get(clone, NiBSBoneLODController::m_iLOD);
    e.set(clone, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    ni_bs_bone_lod_controller_set_bone_lod(e, clone, lod as i32);
}

// Translated from 00c53870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53870 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiTimeController::LoadBinary` (`00a6d5e0`), the LOD and LOD count (`00a26df0`), the
/// number of sets, then for each set a new empty set stored with the array's grow-and-set,
/// its node count, and per node the controller itself added as a placeholder (resolved by
/// `LinkObject`) and the node's link id read (`00a645b0`).
pub fn ni_bs_bone_lod_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_WORD, &args![stream, this.addr() + 0x34]);
    e.call(READ_WORD, &args![stream, this.addr() + 0x38]);
    e.with_stack(8, |e, counts| {
        let sets_count = counts.addr();
        let nodes_count = sets_count + 4;
        e.mem.set_u32(sets_count, 0);
        e.call(READ_WORD, &args![stream, sets_count]);
        let mut index = 0u32;
        while index < e.mem.u32(sets_count) {
            let set = new_bone_set(e);
            bone_array_set_at_grow(e, this.addr() + 0x3c, index, set);
            e.mem.set_u32(nodes_count, 0);
            e.call(READ_WORD, &args![stream, nodes_count]);
            let mut node = 0u32;
            while node < e.mem.u32(nodes_count) {
                add_to_set(e, set, this.addr());
                e.call(READ_LINK_ID, &args![stream]);
                node = node.wrapping_add(1);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c53a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::LinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiTimeController::LinkObject` (`00a6d500`), then every node slot of every set is
/// replaced by the next linked object (`00a63430`), the slot index checked against the
/// set's count again after each lookup.
pub fn ni_bs_bone_lod_controller_link_object(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
                if item < e.mem.u32(set + 8) {
                    let base = e.mem.u32(set);
                    e.mem
                        .set_u32(base.wrapping_add(item.wrapping_mul(4)), object);
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c53ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::PostLinkObject` (Xbox PDB; `thiscall`, `RET 4`: stream): the
/// base `PostLinkObject` (`00a89af0`), then the LOD applied again (its word set to -1,
/// [`ni_bs_bone_lod_controller_set_bone_lod`] with the old value).
pub fn ni_bs_bone_lod_controller_post_link_object(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    let lod = e.get(this, NiBSBoneLODController::m_iLOD);
    e.set(this, NiBSBoneLODController::m_iLOD, 0xffff_ffff);
    ni_bs_bone_lod_controller_set_bone_lod(e, this, lod as i32);
}

// Translated from 00c53e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c53e40 - the compiler's exception frame (FS:[0] unwind states) is not translated
/// `NiBSBoneLODController`'s destructor (unnamed in the engine map; `thiscall`): the
/// class's virtual table, every node set deleted ([`fn_00c52880`]), the bone array's
/// destructor inlined (its virtual table `0109a2a4`, its storage freed by `00aa10f0`), then
/// `NiTimeController`'s destructor (`00a6d290`).
pub fn fn_00c53e40(e: &mut Engine, this: Ptr<NiBSBoneLODController>) {
    e.mem.set_u32(this.addr(), BONE_LOD_CONTROLLER_VTABLE);
    fn_00c52880(e, this);
    e.mem.set_u32(this.addr() + 0x3c, ARRAY_BASE_VTABLE);
    let storage = e.get(this, NiBSBoneLODController::m_pBase);
    e.call(FREE, &args![storage]);
    e.call(TIME_CONTROLLER_DESTRUCTOR, &args![this]);
}

// Translated from 00c53e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// the destructor ([`fn_00c53e40`]) and, when bit 0 of `flags` is set, the sized delete of
/// 0x4C bytes. Answers `this`.
pub fn ni_bs_bone_lod_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    flags: u32,
) -> Ptr<NiBSBoneLODController> {
    fn_00c53e40(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, BONE_LOD_CONTROLLER_SIZE]);
    }
    this
}

/// The virtual tables of `NiTMapBase<NiTPointerAllocator<unsigned int>,NiNode*,unsigned int>`
/// (`0109a264`) and of `NiTPointerMap<NiNode*,unsigned int>` (`0109a284`), the map from a node
/// to its LOD, and of `NiTPointerMap<unsigned int,bool>` (`01030f8c`), the set of LODs (all
/// three by RTTI).
const NODE_MAP_BASE_VTABLE: u32 = 0x0109_a264;
const NODE_MAP_VTABLE: u32 = 0x0109_a284;
const LOD_MAP_VTABLE: u32 = 0x0103_0f8c;
/// The node map's `RemoveAll` body (`00b601a0`, `thiscall`), its `GetNext` (`00b57f20`,
/// `thiscall`, `RET 0xC`: address of the position, of the key, of the value) and `SetAt`
/// (`00b57ea0`, `thiscall`, `RET 8`: node, LOD).
const NODE_MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
const NODE_MAP_GET_NEXT: u32 = 0x00b5_7f20;
const NODE_MAP_SET_AT: u32 = 0x00b5_7ea0;
/// The LOD set's constructor (`0057c960`, `thiscall`, `RET 4`: hash size), destructor
/// (`0057c9d0`, `thiscall`), `GetNext` (`00a53c90`, `thiscall`, `RET 0xC`: address of the
/// position, of the key, of the value byte) and `SetAt` (`0084d310`, `thiscall`, `RET 8`:
/// key, value).
const LOD_MAP_CONSTRUCTOR: u32 = 0x0057_c960;
const LOD_MAP_DESTRUCTOR: u32 = 0x0057_c9d0;
const LOD_MAP_GET_NEXT: u32 = 0x00a5_3c90;
const LOD_MAP_SET_AT: u32 = 0x0084_d310;
/// CRT functions, all `cdecl`: `memset` (`00ec61c0`), `strstr` (`00ec7750`), `strlen`
/// (`00ec6130`), `strcpy_s` (`00ec65a6`), `strtok_s` (`00ecc5f8`), `strcmp` (`00ec6da0`) and
/// `atol` (`00eca6d3`).
const MEMSET: u32 = 0x00ec_61c0;
const STRSTR: u32 = 0x00ec_7750;
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRTOK_S: u32 = 0x00ec_c5f8;
const STRCMP: u32 = 0x00ec_6da0;
const ATOL: u32 = 0x00ec_a6d3;
/// `NiStringExtraData::ms_RTTI` (`011f4a38`; the string value is at +0xC).
const NI_STRING_EXTRA_DATA_RTTI: u32 = 0x011f_4a38;
/// The texts the extra data are searched for: `"BSBoneLOD#BoneRoot#"` (`010c2998`),
/// `"BSBoneLOD#Bone#"` (`010c29b8`), the token delimiters `"#"` (`0109a2b8`) and the tokens
/// `"BSBoneLOD"` (`010c29ac`) and `"Bone"` (`0109a2b0`).
const BONE_ROOT_TAG: u32 = 0x010c_2998;
const BONE_TAG: u32 = 0x010c_29b8;
const TOKEN_DELIMITERS: u32 = 0x0109_a2b8;
const BONE_LOD_TOKEN: u32 = 0x010c_29ac;
const BONE_TOKEN: u32 = 0x0109_a2b0;
/// `NiObjectNET::RemoveController` (`00a5c480`, `thiscall`, `RET 4`).
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// `NiTimeController::RegisterStreamables` (`00a6d0f0`), `SaveBinary` (`00a6d730`) and
/// `IsEqual` (`00a6cc80`), all `thiscall`, `RET 4`.
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
/// The 4-byte stream writes (`cdecl`: stream, address of the value): `00a3da50`
/// (`NiStreamSaveBinary<float>` in the engine map, used for the LOD and for an empty set's
/// count) and `0044adf0` (`NiStreamSaveBinary<unsigned_int>`).
const SAVE_FOUR_BYTES: u32 = 0x00a3_da50;
const SAVE_UNSIGNED: u32 = 0x0044_adf0;
/// The stream's virtual slot that saves an object's link id (`+0x30`, one word).
const SLOT_SAVE_LINK_ID: u32 = 0x30;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: text), its overloads `00a7d4d0` and `00a7d510`
/// (`cdecl`: name, value), and the viewer string array's add (`0096a610`, `thiscall`,
/// `RET 4`: address of the string).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_OV7: u32 = 0x00a7_d4d0;
const VIEWER_STRING_OV8: u32 = 0x00a7_d510;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// `"m_iLOD"` (`0109a394`) and `"m_uiNumLODs"` (`0109a388`).
const LOD_TEXT: u32 = 0x0109_a394;
const NUM_LODS_TEXT: u32 = 0x0109_a388;
/// `NiBSBoneLODController::ms_RTTI` (`012031b8`, the address `00c53e00` answers); its first
/// word is the class name.
const BONE_LOD_CONTROLLER_RTTI: u32 = 0x0120_31b8;
/// `NiStream::RegisterLoader` (`00a64900`, `cdecl`: class name, create function),
/// `"NiBSBoneLODController"` (`010c1318`) and the create function (`00c537f0`).
const REGISTER_LOADER: u32 = 0x00a6_4900;
const CLASS_NAME_TEXT: u32 = 0x010c_1318;
const CREATE_FUNCTION: u32 = 0x00c5_37f0;
/// The controller's virtual slot `+0x98` (one word: the target node).
const SLOT_SET_TARGET: u32 = 0x98;
/// The object virtual slots `+0x08` (`GetRTTI`), `+0x0C` (`IsNode`) and `+0x5C` (`IsEqual`).
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_IS_NODE: u32 = 0x0c;
const SLOT_IS_EQUAL: u32 = 0x5c;

/// `NiObjectNET::m_ppkExtra` (Xbox PDB) +0x10 and `m_usExtraDataSize` +0x14; `NiNode`'s
/// children array (`m_kChildren`, Xbox PDB): elements at +0xA0, size (`m_usSize`) at +0xA6.
const EXTRA_DATA: u32 = 0x10;
const EXTRA_DATA_SIZE: u32 = 0x14;
const CHILDREN: u32 = 0xa0;
const CHILDREN_SIZE: u32 = 0xa6;

/// The extra data `extra` when it is an `NiStringExtraData` (its RTTI chain, through `+4`,
/// reaches `011f4a38`), else 0; 0 for a null pointer.
fn string_extra_data(e: &mut Engine, extra: u32) -> u32 {
    if extra == 0 {
        return 0;
    }
    let mut rtti = e.vcall(extra, SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_STRING_EXTRA_DATA_RTTI {
            return extra;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    0
}

/// The extra data at index `index` (the exe truncates it to 16 bits) of `node`.
fn extra_data_at(e: &Engine, node: u32, index: u32) -> u32 {
    let list = e.mem.u32(node.wrapping_add(EXTRA_DATA));
    e.mem.u32(list.wrapping_add((index & 0xffff) * 4))
}

/// The child at `index` of the node `node`.
fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let children = e.mem.u32(node.wrapping_add(CHILDREN));
    e.mem.u32(children.wrapping_add(index.wrapping_mul(4)))
}

/// Whether `object` is non-null and its `IsNode` (virtual `+0x0C`) answers non-zero.
fn is_node(e: &mut Engine, object: u32) -> bool {
    object != 0 && e.vcall(object, SLOT_IS_NODE, &args![]).u32() != 0
}

/// The first position of a hash map (`NiTMapBase` layout): its first non-null bucket, or 0.
fn first_position(e: &Engine, map: Ptr<NiTPointerMap>) -> u32 {
    let mut bucket = 0u32;
    while bucket < e.get(map, NiTPointerMap::m_uiHashSize) {
        let table = e.get(map, NiTPointerMap::m_ppkHashTable);
        let item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
        if item != 0 {
            return item;
        }
        bucket = bucket.wrapping_add(1);
    }
    0
}

// Translated from 00a54060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor of the node map (unnamed in the engine map; `thiscall`,
/// `RET 4`: hash size): the hash size, the base virtual table `0109a264`, no items, and a
/// table of `hash_size` null buckets (`00aa1070`, cleared by `memset`). Answers `this`.
pub fn fn_00a54060(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    let bytes = hash_size.wrapping_mul(4);
    e.mem.set_u32(this.addr(), NODE_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let table = e.call(ALLOCATE, &args![bytes]).u32();
    let size = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_mul(4);
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    e.call(MEMSET, &args![table, 0u32, size]);
    this
}

// Translated from 00a540a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The node map's destructor (`NiTPointerMap<NiNode*,unsigned int>`, unnamed in the engine
/// map; `thiscall`): its virtual table and `RemoveAll` (`00b601a0`), the base virtual table
/// and `RemoveAll` again, then the bucket table freed (`00aa10f0`).
pub fn fn_00a540a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), NODE_MAP_VTABLE);
    e.call(NODE_MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), NODE_MAP_BASE_VTABLE);
    e.call(NODE_MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE, &args![table]);
}

// Translated from 00a54110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveSet<NiNode*>`'s constructor (unnamed in the engine map; `thiscall`, `RET 4`:
/// initial size): storage for `initial_size` pointers (`00aa1070`) or none for 0, that
/// capacity, no elements. Answers `this`.
pub fn fn_00a54110(
    e: &mut Engine,
    this: Ptr<NiTPrimitiveSet>,
    initial_size: u32,
) -> Ptr<NiTPrimitiveSet> {
    if initial_size != 0 {
        let storage = e.call(ALLOCATE, &args![initial_size.wrapping_mul(4)]).u32();
        e.set(this, NiTPrimitiveSet::m_uiAlloced, initial_size);
        e.set(this, NiTPrimitiveSet::m_pBase, storage);
    } else {
        e.set(this, NiTPrimitiveSet::m_uiAlloced, initial_size);
        e.set(this, NiTPrimitiveSet::m_pBase, 0);
    }
    e.set(this, NiTPrimitiveSet::m_uiUsed, 0);
    this
}

// Translated from 00a54160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiTPrimitiveSet<NiNode*>*>::_scalar_deleting_destructor_` (engine map;
/// `thiscall`, `RET 4`): the `NiTArray` virtual table (`0109a2a4`), the storage freed
/// (`00aa10f0`) and, when bit 0 of `flags` is set, `this` given back to the memory manager.
/// Answers `this`.
pub fn ni_t_primitive_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let storage = e.mem.u32(this.addr().wrapping_add(4));
    e.mem.set_u32(this.addr(), ARRAY_BASE_VTABLE);
    e.call(FREE, &args![storage]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c52c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the bone LOD roots (unnamed in the engine map; `cdecl`: node, set). When one of
/// the node's extra data is an `NiStringExtraData` whose text contains
/// `"BSBoneLOD#BoneRoot#"`, the node is added to the set ([`ni_t_set_add`], with the address
/// of the node argument) and its children are not visited. Otherwise every child that is a
/// node (`IsNode`) is searched the same way.
pub fn fn_00c52c30(e: &mut Engine, node: Ptr, set: Ptr<NiTPrimitiveSet>) {
    let node = node.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(EXTRA_DATA_SIZE))) {
        let extra = extra_data_at(e, node, index);
        let string_data = string_extra_data(e, extra);
        if string_data != 0 {
            let text = e.mem.u32(string_data.wrapping_add(0xc));
            if text != 0 && e.call(STRSTR, &args![text, BONE_ROOT_TAG]).u32() != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), node);
                    ni_t_set_add(e, set, slot);
                });
                return;
            }
        }
        index = index.wrapping_add(1);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
        let child = child_at(e, node, index);
        if is_node(e, child) {
            fn_00c52c30(e, Ptr::new(child), set);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c52a80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c52a80 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f3e7, state variable) is not translated
/// Gives every bone LOD root under `root` a bone LOD controller (unnamed in the engine map;
/// `cdecl`, one word). The roots are collected ([`fn_00c52c30`]) into a stack
/// `NiTPrimitiveSet` (its inlined constructor's allocation for the initial size 0 is folded
/// away by the compiler: `XOR EAX,EAX; JBE`, so it never runs). No root: the set's storage is
/// freed (`00aa10f0`) and the answer is false. For each root a new controller (`NiNew` 0x4C,
/// [`fn_00c53d70`]; null when the allocation fails) gets the root as its target (virtual
/// `+0x98`) and is initialised ([`fn_00c52d80`]): on success its LOD is set to 0; on failure
/// the controller is removed from the root again (`NiObjectNET::RemoveController`), the
/// storage freed and the answer false. Answers true after the last root (storage freed).
pub fn fn_00c52a80(e: &mut Engine, root: Ptr) -> bool {
    e.with_stack(0xc, |e, set| {
        let set = set.cast::<NiTPrimitiveSet>();
        e.set(set, NiTPrimitiveSet::m_pBase, 0);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, 0);
        e.set(set, NiTPrimitiveSet::m_uiUsed, 0);
        fn_00c52c30(e, root, set);
        if e.get(set, NiTPrimitiveSet::m_uiUsed) == 0 {
            let storage = e.get(set, NiTPrimitiveSet::m_pBase);
            e.call(FREE, &args![storage]);
            return false;
        }
        let mut index = 0u32;
        while index < e.get(set, NiTPrimitiveSet::m_uiUsed) {
            let storage = e.get(set, NiTPrimitiveSet::m_pBase);
            let node = e.mem.u32(storage.wrapping_add(index.wrapping_mul(4)));
            let block = e.call(OPERATOR_NEW, &args![BONE_LOD_CONTROLLER_SIZE]).u32();
            let controller = if block == 0 {
                Ptr::new(0)
            } else {
                fn_00c53d70(e, Ptr::new(block))
            };
            e.vcall(controller.addr(), SLOT_SET_TARGET, &args![node]);
            let initialised = fn_00c52d80(e, controller, Ptr::new(node));
            if initialised {
                ni_bs_bone_lod_controller_set_bone_lod(e, controller, 0);
            }
            if !initialised {
                e.call(REMOVE_CONTROLLER, &args![node, controller]);
                let storage = e.get(set, NiTPrimitiveSet::m_pBase);
                e.call(FREE, &args![storage]);
                return false;
            }
            index = index.wrapping_add(1);
        }
        let storage = e.get(set, NiTPrimitiveSet::m_pBase);
        e.call(FREE, &args![storage]);
        true
    })
}

// Translated from 00c53030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the bones of a LOD tree (unnamed in the engine map; `thiscall`, `RET 0xC`: node,
/// node map, LOD set). The first `NiStringExtraData` of the node whose text contains
/// `"BSBoneLOD#Bone#"` is copied (`strlen` + 1 bytes from the memory manager, `strcpy_s`) and
/// split at `"#"` (`strtok_s`): for `"BSBoneLOD"`, `"Bone"` and a third token, the node is
/// mapped to `atol` of that token (`00b57ea0`) and that LOD is entered in the LOD set with
/// true (`0084d310`); the copy is given back to the memory manager and the other extra data
/// are not looked at. Then every child that is a node is visited the same way.
pub fn fn_00c53030(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    node: Ptr,
    nodes: Ptr,
    lods: Ptr,
) {
    let node = node.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(EXTRA_DATA_SIZE))) {
        let extra = extra_data_at(e, node, index);
        let string_data = string_extra_data(e, extra);
        if string_data != 0 {
            let text = e.mem.u32(string_data.wrapping_add(0xc));
            if text != 0 && e.call(STRSTR, &args![text, BONE_TAG]).u32() != 0 {
                let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
                let copy = e
                    .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
                    .u32();
                e.call(STRCPY_S, &args![copy, size, text]);
                e.with_stack(4, |e, context| {
                    let token = e
                        .call(STRTOK_S, &args![copy, TOKEN_DELIMITERS, context])
                        .u32();
                    if token == 0 || e.call(STRCMP, &args![token, BONE_LOD_TOKEN]).u32() != 0 {
                        return;
                    }
                    let token = e
                        .call(STRTOK_S, &args![0u32, TOKEN_DELIMITERS, context])
                        .u32();
                    if token == 0 || e.call(STRCMP, &args![token, BONE_TOKEN]).u32() != 0 {
                        return;
                    }
                    let token = e
                        .call(STRTOK_S, &args![0u32, TOKEN_DELIMITERS, context])
                        .u32();
                    if token == 0 {
                        return;
                    }
                    let lod = e.call(ATOL, &args![token]).u32();
                    e.call(NODE_MAP_SET_AT, &args![nodes, node, lod]);
                    e.call(LOD_MAP_SET_AT, &args![lods, lod, 1u32]);
                });
                e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, copy]);
                break;
            }
        }
        index = index.wrapping_add(1);
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
        let child = child_at(e, node, index);
        if is_node(e, child) {
            fn_00c53030(e, this, Ptr::new(child), nodes, lods);
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 00c53280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gives every node of a LOD tree its LOD (unnamed in the engine map; `thiscall`, `RET 0xC`:
/// node, node map, address of the LOD word). Each child that is a node is visited first with
/// a copy of the incoming LOD, and a higher answer raises the word. Then the node's own entry
/// (`00a53c30`): a higher mapped LOD raises the word; without an entry the node is mapped to
/// the word's value (`00b57ea0`).
pub fn fn_00c53280(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    node: Ptr,
    nodes: Ptr,
    lod: Ptr,
) {
    let node = node.addr();
    let lod = lod.addr();
    let inherited = e.mem.u32(lod);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node.wrapping_add(CHILDREN_SIZE))) {
        let child = child_at(e, node, index);
        if is_node(e, child) {
            e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), inherited);
                fn_00c53280(e, this, Ptr::new(child), nodes, local);
                let found = e.mem.u32(local.addr());
                if found > e.mem.u32(lod) {
                    e.mem.set_u32(lod, found);
                }
            });
        }
        index = index.wrapping_add(1);
    }
    e.with_stack(4, |e, mapped| {
        if e.call(CLONE_MAP_LOOKUP, &args![nodes, node, mapped]).u8() != 0 {
            let value = e.mem.u32(mapped.addr());
            if value > e.mem.u32(lod) {
                e.mem.set_u32(lod, value);
            }
        } else {
            let value = e.mem.u32(lod);
            e.call(NODE_MAP_SET_AT, &args![nodes, node, value]);
        }
    });
}

// Translated from 00c52d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c52d80 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2f41d, state variable) is not translated
/// Builds the controller's bone sets from the tree under `root` (unnamed in the engine map;
/// `thiscall`, `RET 4`). The old sets are deleted ([`fn_00c52880`]); two stack maps of 0x25
/// buckets are made: node to LOD ([`fn_00a54060`], virtual table `0109a284`) and the LOD set
/// (`0057c960`, virtual table `01030f8c`), and filled by [`fn_00c53030`]. The LOD count
/// becomes the highest LOD in the set plus one; the bone array is sized to it (`00a90180`)
/// and every slot set to null (`00ba3aa0`). Each mapped node is then added
/// ([`ni_t_set_add`]) to the set of its LOD, made when missing (0xC bytes from the memory
/// manager, [`fn_00a54110`] with 0; null when that fails) and stored in the array. Every
/// node gets its LOD ([`fn_00c53280`] from 0), the LOD is set to -1, and both maps are
/// destroyed (`0057c9d0`, [`fn_00a540a0`]). Answers true.
pub fn fn_00c52d80(e: &mut Engine, this: Ptr<NiBSBoneLODController>, root: Ptr) -> bool {
    fn_00c52880(e, this);
    e.with_stack(0x40, |e, locals| {
        let nodes = locals.cast::<NiTPointerMap>();
        let lods = Ptr::<NiTPointerMap>::new(locals.addr() + 0x10);
        let position = locals.addr() + 0x20;
        let key = locals.addr() + 0x24;
        let value = locals.addr() + 0x28;
        let node = locals.addr() + 0x2c;
        let node_lod = locals.addr() + 0x30;
        let set_slot = locals.addr() + 0x34;
        let highest = locals.addr() + 0x38;
        fn_00a54060(e, nodes, 0x25);
        e.mem.set_u32(nodes.addr(), NODE_MAP_VTABLE);
        e.call(LOD_MAP_CONSTRUCTOR, &args![lods, 0x25u32]);
        e.mem.set_u32(lods.addr(), LOD_MAP_VTABLE);
        fn_00c53030(e, this, root, nodes.cast(), lods.cast());
        let mut max = 0u32;
        e.mem.set_u32(position, first_position(e, lods));
        while e.mem.u32(position) != 0 {
            e.call(LOD_MAP_GET_NEXT, &args![lods, position, key, value]);
            let lod = e.mem.u32(key);
            if lod > max {
                max = lod;
            }
        }
        e.set(
            this,
            NiBSBoneLODController::m_uiNumLODs,
            max.wrapping_add(1),
        );
        let count = e.get(this, NiBSBoneLODController::m_uiNumLODs);
        let array = this.addr() + 0x3c;
        e.call(ARRAY_SET_SIZE, &args![array, count]);
        let mut index = 0u32;
        while index < e.get(this, NiBSBoneLODController::m_uiNumLODs) {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 0);
                e.call(ARRAY_SET_AT, &args![array, index, slot]);
            });
            index = index.wrapping_add(1);
        }
        e.mem.set_u32(position, first_position(e, nodes));
        while e.mem.u32(position) != 0 {
            e.call(NODE_MAP_GET_NEXT, &args![nodes, position, node, node_lod]);
            let lod = e.mem.u32(node_lod);
            let mut set = bone_set(e, this, lod);
            e.mem.set_u32(set_slot, set);
            if set == 0 {
                let block = e
                    .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0xcu32])
                    .u32();
                set = if block == 0 {
                    0
                } else {
                    fn_00a54110(e, Ptr::new(block), 0).addr()
                };
                e.mem.set_u32(set_slot, set);
                e.call(ARRAY_SET_AT, &args![array, lod, set_slot]);
            }
            ni_t_set_add(e, Ptr::new(set), Ptr::new(node));
        }
        e.mem.set_u32(highest, 0);
        fn_00c53280(e, this, root, nodes.cast(), Ptr::new(highest));
        ni_bs_bone_lod_controller_set_bone_lod(e, this, -1);
        e.call(LOD_MAP_DESTRUCTOR, &args![lods]);
        fn_00a540a0(e, nodes);
    });
    true
}

// Translated from 00c536a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::RegisterStreamables` (engine map; `thiscall`, `RET 4`: stream):
/// whether `NiTimeController::RegisterStreamables` (`00a6d0f0`) answers a non-zero byte.
pub fn ni_bs_bone_lod_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) -> bool {
    e.call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c536d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::SaveBinary` (engine map; `thiscall`, `RET 4`: stream):
/// `NiTimeController::SaveBinary` (`00a6d730`), the LOD (`00a3da50`) and the LOD count
/// (`0044adf0`), the bone array's size (16 bits, saved as a word), then for each slot: a
/// set's node count and the link id of each node (the stream's virtual `+0x30`), or a 0
/// count (`00a3da50`) for a null slot.
pub fn ni_bs_bone_lod_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    stream: Ptr,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    e.call(SAVE_FOUR_BYTES, &args![stream, this.addr() + 0x34]);
    e.call(SAVE_UNSIGNED, &args![stream, this.addr() + 0x38]);
    e.with_stack(0xc, |e, locals| {
        let size = locals.addr();
        let nodes = size + 4;
        let empty = size + 8;
        let count = u32::from(e.get(this, NiBSBoneLODController::m_usSize));
        e.mem.set_u32(size, count);
        e.call(SAVE_UNSIGNED, &args![stream, size]);
        let mut index = 0u32;
        while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
            let set = bone_set(e, this, index);
            if set != 0 {
                let count = e.mem.u32(set + 8);
                e.mem.set_u32(nodes, count);
                e.call(SAVE_UNSIGNED, &args![stream, nodes]);
                let mut item = 0u32;
                while item < e.mem.u32(set + 8) {
                    let object = e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4)));
                    e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![object]);
                    item = item.wrapping_add(1);
                }
            } else {
                e.mem.set_u32(empty, 0);
                e.call(SAVE_FOUR_BYTES, &args![stream, empty]);
            }
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c53af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::IsEqual` (engine map; `thiscall`, `RET 4`: the other
/// controller): false when `NiTimeController::IsEqual` (`00a6cc80`) is, when the LOD, the LOD
/// count or the bone array's size differ, when only one of two slots holds a set, when two
/// sets differ in count, or when a node's `IsEqual` (virtual `+0x5C`) with the other set's
/// node answers false; otherwise true.
pub fn ni_bs_bone_lod_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    other: Ptr<NiBSBoneLODController>,
) -> bool {
    if e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    if e.get(this, NiBSBoneLODController::m_iLOD) != e.get(other, NiBSBoneLODController::m_iLOD)
        || e.get(this, NiBSBoneLODController::m_uiNumLODs)
            != e.get(other, NiBSBoneLODController::m_uiNumLODs)
        || e.get(this, NiBSBoneLODController::m_usSize)
            != e.get(other, NiBSBoneLODController::m_usSize)
    {
        return false;
    }
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let mine = bone_set(e, this, index);
        let theirs = bone_set(e, other, index);
        if (mine != 0 && theirs == 0) || (mine == 0 && theirs != 0) {
            return false;
        }
        if mine != 0 && theirs != 0 {
            if e.mem.u32(mine + 8) != e.mem.u32(theirs + 8) {
                return false;
            }
            let mut item = 0u32;
            while item < e.mem.u32(mine + 8) {
                let node = e
                    .mem
                    .u32(e.mem.u32(mine).wrapping_add(item.wrapping_mul(4)));
                let other_node = e
                    .mem
                    .u32(e.mem.u32(theirs).wrapping_add(item.wrapping_mul(4)));
                if e.vcall(node, SLOT_IS_EQUAL, &args![other_node]).u8() == 0 {
                    return false;
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    true
}

/// Adds the viewer string `text` (by address, as the exe passes it) to `strings`.
fn add_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c53c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the viewer
/// string array): the class name (the first word of `012031b8`, through `NiGetViewerString`
/// `00a7d330`), `"m_iLOD"` with the LOD (`00a7d4d0`) and `"m_uiNumLODs"` with the LOD count
/// (`00a7d510`), each added to the array (`0096a610`). The base class's strings are not
/// added.
pub fn ni_bs_bone_lod_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
    strings: Ptr,
) {
    let name = e.mem.u32(BONE_LOD_CONTROLLER_RTTI);
    let text = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let lod = e.get(this, NiBSBoneLODController::m_iLOD);
    let text = e.call(VIEWER_STRING_OV7, &args![LOD_TEXT, lod]).u32();
    add_viewer_string(e, strings, text);
    let count = e.get(this, NiBSBoneLODController::m_uiNumLODs);
    let text = e
        .call(VIEWER_STRING_OV8, &args![NUM_LODS_TEXT, count])
        .u32();
    add_viewer_string(e, strings, text);
}

// Translated from 00c53cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the class's loader (unnamed in the engine map; no arguments):
/// `NiStream::RegisterLoader("NiBSBoneLODController", 00c537f0)`.
pub fn fn_00c53cd0(e: &mut Engine) {
    e.call(REGISTER_LOADER, &args![CLASS_NAME_TEXT, CREATE_FUNCTION]);
}

// Translated from 00c53cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any set of the bone array holds a non-null node (unnamed in the engine map;
/// `thiscall`, plain `RET`).
pub fn fn_00c53cf0(e: &mut Engine, this: Ptr<NiBSBoneLODController>) -> bool {
    let mut index = 0u32;
    while index < u32::from(e.get(this, NiBSBoneLODController::m_usSize)) {
        let set = bone_set(e, this, index);
        if set != 0 {
            let mut item = 0u32;
            while item < e.mem.u32(set + 8) {
                if e.mem.u32(e.mem.u32(set).wrapping_add(item.wrapping_mul(4))) != 0 {
                    return true;
                }
                item = item.wrapping_add(1);
            }
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c53ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSBoneLODController::TargetIsRequiredType` (engine map; `thiscall`, plain `RET`):
/// whether the target (`m_pkTarget`) is non-null and its `IsNode` (virtual `+0x0C`) answers
/// non-zero.
pub fn ni_bs_bone_lod_controller_target_is_required_type(
    e: &mut Engine,
    this: Ptr<NiBSBoneLODController>,
) -> bool {
    let target = e.get(this, NiBSBoneLODController::m_pkTarget);
    is_node(e, target)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_4060,
            fn_00a54060(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00a5_40a0, fn_00a540a0(Ptr<NiTPointerMap>)),
        entry!(
            0x00a5_4110,
            fn_00a54110(Ptr<NiTPrimitiveSet>, u32) -> Ptr<NiTPrimitiveSet>
        ),
        entry!(
            0x00a5_4160,
            ni_t_primitive_array_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_2a80, fn_00c52a80(Ptr) -> bool),
        entry!(0x00c5_2c30, fn_00c52c30(Ptr, Ptr<NiTPrimitiveSet>)),
        entry!(
            0x00c5_2d80,
            fn_00c52d80(Ptr<NiBSBoneLODController>, Ptr) -> bool
        ),
        entry!(
            0x00c5_3030,
            fn_00c53030(Ptr<NiBSBoneLODController>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00c5_3280,
            fn_00c53280(Ptr<NiBSBoneLODController>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00c5_36a0,
            ni_bs_bone_lod_controller_register_streamables(Ptr<NiBSBoneLODController>, Ptr) -> bool
        ),
        entry!(
            0x00c5_36d0,
            ni_bs_bone_lod_controller_save_binary(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3af0,
            ni_bs_bone_lod_controller_is_equal(
                Ptr<NiBSBoneLODController>,
                Ptr<NiBSBoneLODController>,
            ) -> bool
        ),
        entry!(
            0x00c5_3c50,
            ni_bs_bone_lod_controller_get_viewer_strings(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(0x00c5_3cd0, fn_00c53cd0()),
        entry!(0x00c5_3cf0, fn_00c53cf0(Ptr<NiBSBoneLODController>) -> bool),
        entry!(
            0x00c5_3ed0,
            ni_bs_bone_lod_controller_target_is_required_type(Ptr<NiBSBoneLODController>) -> bool
        ),
        entry!(0x00c5_3e00, fn_00c53e00(Ptr) -> u32),
        entry!(0x00a3_11e0, ni_t_set_realloc(Ptr<NiTPrimitiveSet>, u32)),
        entry!(0x00a5_4190, fn_00a54190(Ptr, u16, u16) -> Ptr),
        entry!(0x00a5_43b0, ni_t_set_add(Ptr<NiTPrimitiveSet>, Ptr)),
        entry!(0x00c5_2880, fn_00c52880(Ptr<NiBSBoneLODController>)),
        entry!(
            0x00c5_2960,
            ni_bs_bone_lod_controller_set_bone_lod(Ptr<NiBSBoneLODController>, i32) -> bool
        ),
        entry!(
            0x00c5_3d70,
            fn_00c53d70(Ptr<NiBSBoneLODController>) -> Ptr<NiBSBoneLODController>
        ),
        entry!(0x00c5_37f0, fn_00c537f0() -> Ptr<NiBSBoneLODController>),
        entry!(
            0x00c5_33f0,
            fn_00c533f0(Ptr<NiBSBoneLODController>, Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3360,
            ni_bs_bone_lod_controller_create_clone(
                Ptr<NiBSBoneLODController>,
                Ptr,
            ) -> Ptr<NiBSBoneLODController>
        ),
        entry!(
            0x00c5_3430,
            ni_bs_bone_lod_controller_process_clone(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3870,
            ni_bs_bone_lod_controller_load_binary(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3a10,
            ni_bs_bone_lod_controller_link_object(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(
            0x00c5_3ab0,
            ni_bs_bone_lod_controller_post_link_object(Ptr<NiBSBoneLODController>, Ptr)
        ),
        entry!(0x00c5_3e40, fn_00c53e40(Ptr<NiBSBoneLODController>)),
        entry!(
            0x00c5_3e10,
            ni_bs_bone_lod_controller_scalar_deleting_destructor(
                Ptr<NiBSBoneLODController>,
                u32,
            )
                -> Ptr<NiBSBoneLODController>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(capacity: u32, items: &[u32]) -> (Engine, Ptr<NiTPrimitiveSet>, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let set = e.new_object::<NiTPrimitiveSet>();
        let base = e.mem.alloc(capacity * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(set, NiTPrimitiveSet::m_pBase, base);
        e.set(set, NiTPrimitiveSet::m_uiAlloced, capacity);
        e.set(set, NiTPrimitiveSet::m_uiUsed, items.len() as u32);
        (e, set, base)
    }

    #[test]
    fn the_current_capacity_changes_nothing() {
        let (mut e, set, base) = fixture(4, &[1, 2]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 4u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), base);
    }

    #[test]
    fn a_new_capacity_copies_the_used_elements_and_frees_the_old_block() {
        let (mut e, set, base) = fixture(2, &[0x11, 0x22]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 8u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![32u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 8);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiUsed), 2);
        let new = e.get(set, NiTPrimitiveSet::m_pBase);
        assert_ne!(new, base);
        assert_eq!([e.mem.u32(new), e.mem.u32(new + 4)], [0x11, 0x22]);
    }

    #[test]
    fn capacity_zero_leaves_a_null_pointer() {
        let (mut e, set, base) = fixture(2, &[0x11]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_11e0, &args![set, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_pBase), 0);
        assert_eq!(e.get(set, NiTPrimitiveSet::m_uiAlloced), 0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c53e00_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_3e00, &args![0x1000u32]).u32(), 0x0120_31b8);
        assert_eq!(e.call(0x00c5_3e00, &args![0u32]).u32(), 0x0120_31b8);
    }
}

#[cfg(test)]
mod b037_tests {
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        for address in [
            TIME_CONTROLLER_CONSTRUCTOR,
            TIME_CONTROLLER_DESTRUCTOR,
            TIME_CONTROLLER_COPY_MEMBERS,
            TIME_CONTROLLER_PROCESS_CLONE,
            TIME_CONTROLLER_LOAD_BINARY,
            TIME_CONTROLLER_LINK_OBJECT,
            BASE_POST_LINK_OBJECT,
            SIZED_DELETE,
            READ_LINK_ID,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        // The array's SetSize grows the storage; SetAt stores the element.
        e.register(ARRAY_SET_SIZE, |e, a| {
            let storage = e.mem.alloc(4 * a[1] + 16);
            let old = e.mem.u32(a[0] + 4);
            for index in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let value = e.mem.u32(old + 4 * index);
                e.mem.set_u32(storage + 4 * index, value);
            }
            e.mem.set_u32(a[0] + 4, storage);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            let storage = e.mem.u32(a[0] + 4);
            e.mem.set_u32(storage + 4 * a[1], value);
            if a[1] >= u32::from(e.mem.u16(a[0] + 0xa)) {
                e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A controller whose bone array holds the given sets of nodes (0 = null set).
    fn controller(e: &mut Engine, sets: &[&[u32]], lods: u32) -> u32 {
        let this = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        let base = e.mem.alloc(4 * sets.len() as u32 + 4);
        for (index, nodes) in sets.iter().enumerate() {
            if nodes.is_empty() {
                continue;
            }
            let set = e.mem.alloc(0xc);
            let storage = e.mem.alloc(4 * nodes.len() as u32);
            for (slot, node) in nodes.iter().enumerate() {
                e.mem.set_u32(storage + 4 * slot as u32, *node);
            }
            e.mem.set_u32(set, storage);
            e.mem.set_u32(set + 4, nodes.len() as u32);
            e.mem.set_u32(set + 8, nodes.len() as u32);
            e.mem.set_u32(base + 4 * index as u32, set);
        }
        e.mem.set_u32(this + 0x40, base);
        e.mem.set_u16(this + 0x44, sets.len() as u16);
        e.mem.set_u16(this + 0x46, sets.len() as u16);
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        e.mem.set_u32(this + 0x38, lods);
        this
    }

    fn node(e: &mut Engine, flags: u32) -> u32 {
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x30, flags);
        node
    }

    #[test]
    fn set_bone_lod_flags_the_sets_from_the_lod_on() {
        let mut e = engine();
        let nodes: Vec<u32> = (0..3).map(|_| node(&mut e, 0x10)).collect();
        let this = controller(&mut e, &[&[nodes[0]], &[], &[nodes[1], 0, nodes[2]]], 3);
        let set_lod =
            |e: &mut Engine, lod: i32| e.call(0x00c5_2960, &args![this, lod as u32]).bool();
        assert!(set_lod(&mut e, 2));
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert_eq!(e.mem.u32(nodes[0] + 0x30), 0x10);
        assert_eq!(e.mem.u32(nodes[1] + 0x30), 0x12);
        assert_eq!(e.mem.u32(nodes[2] + 0x30), 0x12);
        // The same LOD, one out of range and a negative one other than -1 are refused.
        assert!(!set_lod(&mut e, 2));
        assert!(!set_lod(&mut e, 3));
        assert!(!set_lod(&mut e, -2));
        assert!(set_lod(&mut e, 0));
        assert_eq!(e.mem.u32(nodes[0] + 0x30), 0x12);
        assert!(set_lod(&mut e, -1));
        assert!(nodes.iter().all(|n| e.mem.u32(n + 0x30) == 0x10));
        e.mem.set_u32(this + 0x38, 0);
        assert!(!set_lod(&mut e, 1));
    }

    #[test]
    fn deleting_the_sets_frees_them_and_clears_the_array() {
        let mut e = engine();
        let this = controller(&mut e, &[&[0x100], &[], &[0x200]], 1);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(base);
        let last = e.mem.u32(base + 8);
        e.mem.set_u16(this + 0x48, 2);
        e.call(0x00c5_2880, &args![this]);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, first], vec![MEMORY_MANAGER, last]]
        );
        assert_eq!(calls(&e, FREE).len(), 2);
        assert_eq!((e.mem.u32(base), e.mem.u32(base + 8)), (0, 0));
        assert_eq!((e.mem.u16(this + 0x46), e.mem.u16(this + 0x48)), (0, 0));
    }

    #[test]
    fn set_add_doubles_a_full_set() {
        let mut e = engine();
        let set = e.mem.alloc(0xc);
        let value = e.mem.alloc(4);
        for item in 1..=3u32 {
            e.mem.set_u32(value, item * 0x10);
            e.call(0x00a5_43b0, &args![set, value]);
        }
        assert_eq!((e.mem.u32(set + 4), e.mem.u32(set + 8)), (4, 3));
        let storage = e.mem.u32(set);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [0x10, 0x20, 0x30]
        );
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4], vec![8], vec![16]]);
    }

    #[test]
    fn array_constructor_allocates_only_a_capacity() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x55);
        assert_eq!(e.call(0x00a5_4190, &args![array, 0u32, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_BASE_VTABLE);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!((e.mem.u16(array + 8), e.mem.u16(array + 0xe)), (0, 1));
        e.call(0x00a5_4190, &args![array, 0x10003u32, 2u32]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![12]]);
        assert_eq!(e.mem.u16(array + 8), 3);
        assert_ne!(e.mem.u32(array + 4), 0);
    }

    #[test]
    fn constructor_and_create_function_build_an_empty_controller() {
        let mut e = engine();
        let block = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x4c);
            block.into_ret()
        });
        assert_eq!(e.call(0x00c5_37f0, &args![]).u32(), block);
        assert_eq!(calls(&e, TIME_CONTROLLER_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(e.mem.u32(block), BONE_LOD_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(block + 0x34), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 0x38), 0);
        assert_eq!(e.mem.u32(block + 0x3c), BONE_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(block + 0x4a), 1);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c5_37f0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_lod_words() {
        let mut e = engine();
        let block = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        let this = controller(&mut e, &[], 4);
        e.mem.set_u32(this + 0x34, 2);
        assert_eq!(e.call(0x00c5_3360, &args![this, 0x700u32]).u32(), block);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this, block, 0x700]]
        );
        assert_eq!((e.mem.u32(block + 0x34), e.mem.u32(block + 0x38)), (2, 4));
    }

    #[test]
    fn process_clone_rebuilds_the_sets_with_cloned_nodes() {
        let mut e = engine();
        let original = node(&mut e, 0);
        let missing = node(&mut e, 0);
        let copy = node(&mut e, 0);
        let this = controller(&mut e, &[&[original, 0, missing], &[]], 2);
        let clone = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        e.mem.set_u32(clone + 0x34, 0);
        e.mem.set_u32(clone + 0x38, 2);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            let found = if a[1] == this {
                clone
            } else if a[1] == original {
                copy
            } else {
                return Ret::default();
            };
            e.mem.set_u32(a[2], found);
            1u32.into_ret()
        });
        let cloning = e.mem.alloc(4);
        e.mem.set_u32(cloning, 0x9900);
        e.call(0x00c5_3430, &args![this, cloning]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_PROCESS_CLONE),
            vec![vec![this, cloning]]
        );
        let set_at = calls(&e, ARRAY_SET_AT);
        assert_eq!(set_at.len(), 2);
        assert_eq!((set_at[0][0], set_at[0][1]), (clone + 0x3c, 0));
        assert_eq!((set_at[1][0], set_at[1][1]), (clone + 0x3c, 1));
        let cloned_base = e.mem.u32(clone + 0x40);
        let cloned_set = e.mem.u32(cloned_base);
        assert_eq!(e.mem.u32(cloned_base + 4), 0);
        assert_eq!(e.mem.u32(cloned_set + 8), 3);
        let storage = e.mem.u32(cloned_set);
        assert_eq!(
            [
                e.mem.u32(storage),
                e.mem.u32(storage + 4),
                e.mem.u32(storage + 8)
            ],
            [copy, 0, 0]
        );
        // The LOD is applied again: set 0 holds the copy, which becomes visible.
        assert_eq!(e.mem.u32(clone + 0x34), 0);
        assert_eq!(e.mem.u32(copy + 0x30), 2);
    }

    #[test]
    fn load_binary_reads_the_sets_with_placeholders() {
        let mut e = engine();
        let this = e.mem.alloc(BONE_LOD_CONTROLLER_SIZE);
        // Words read: LOD 1, LOD count 2, two sets of 2 and 1 nodes.
        let mut words = vec![1u32, 2, 2, 2, 1];
        words.reverse();
        e.register_double(READ_WORD, move |e, a| {
            e.mem.set_u32(a[1], words.pop().unwrap());
            Ret::default()
        });
        e.call(0x00c5_3870, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LOAD_BINARY),
            vec![vec![this, 0x900]]
        );
        assert_eq!((e.mem.u32(this + 0x34), e.mem.u32(this + 0x38)), (1, 2));
        assert_eq!(e.mem.u16(this + 0x46), 2);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(base);
        let second = e.mem.u32(base + 4);
        assert_eq!((e.mem.u32(first + 8), e.mem.u32(second + 8)), (2, 1));
        assert_eq!(e.mem.u32(e.mem.u32(first)), this);
        assert_eq!(calls(&e, READ_LINK_ID).len(), 3);
        assert_eq!(calls(&e, ARRAY_SET_SIZE).len(), 2);
    }

    #[test]
    fn link_object_resolves_every_slot() {
        let mut e = engine();
        let this = controller(&mut e, &[&[1, 2], &[], &[3]], 1);
        let mut next = 0x500u32;
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next += 0x10;
            next.into_ret()
        });
        e.call(0x00c5_3a10, &args![this, 0x900u32]);
        let base = e.mem.u32(this + 0x40);
        let first = e.mem.u32(e.mem.u32(base));
        let last = e.mem.u32(e.mem.u32(base + 8));
        assert_eq!(
            [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(last)],
            [0x510, 0x520, 0x530]
        );
        assert_eq!(
            calls(&e, TIME_CONTROLLER_LINK_OBJECT),
            vec![vec![this, 0x900]]
        );
    }

    #[test]
    fn post_link_object_applies_the_lod_again() {
        let mut e = engine();
        let shown = node(&mut e, 0);
        let this = controller(&mut e, &[&[shown]], 1);
        e.mem.set_u32(this + 0x34, 0);
        e.call(0x00c5_3ab0, &args![this, 0x900u32]);
        assert_eq!(calls(&e, BASE_POST_LINK_OBJECT), vec![vec![this, 0x900]]);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(shown + 0x30), 2);
    }

    #[test]
    fn destructors_delete_the_sets_and_the_storage() {
        let mut e = engine();
        let this = controller(&mut e, &[&[0x100]], 1);
        let base = e.mem.u32(this + 0x40);
        e.call(0x00c5_3e10, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), BONE_LOD_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), ARRAY_BASE_VTABLE);
        assert_eq!(calls(&e, FREE).last().unwrap(), &vec![base]);
        assert_eq!(calls(&e, TIME_CONTROLLER_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x4c]]);
        e.call(0x00c5_3e10, &args![this, 0u32]);
        assert_eq!(calls(&e, SIZED_DELETE).len(), 1);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles every callee in `addresses` with a recorder answering `eax`.
    fn record(e: &mut Engine, log: &Log, addresses: &[u32], eax: u32) {
        for &address in addresses {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                eax.into_ret()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The `IsNode` / `GetRTTI` doubles the tree tests use.
    const NODE_VTABLE: u32 = 0x0300_0000;
    const STRING_DATA_VTABLE: u32 = 0x0300_1000;
    const IS_NODE: u32 = 0x00ee_7000;
    const STRING_RTTI: u32 = 0x00ee_7010;

    fn tree_engine() -> Engine {
        let mut e = Engine::new();
        let mut node_slots = vec![0u32; 0x18];
        node_slots[3] = IS_NODE;
        e.put_vtable(NODE_VTABLE, &node_slots);
        e.put_vtable(STRING_DATA_VTABLE, &[0, 0, STRING_RTTI]);
        e.register(IS_NODE, |_, _| 1u32.into_ret());
        // A chain of two records: a derived one whose base (+4) is NiStringExtraData's.
        let derived = e.mem.alloc(8);
        e.map(NI_STRING_EXTRA_DATA_RTTI, 8);
        e.mem.set_u32(derived + 4, NI_STRING_EXTRA_DATA_RTTI);
        e.register_double(STRING_RTTI, move |_, _| derived.into_ret());
        e.register(STRSTR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let needle = e.mem.cstr(a[1]);
            let found = text
                .windows(needle.len())
                .position(|w| w == needle.as_slice());
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.map(BONE_ROOT_TAG, 0x20);
        e.mem.set_cstr(BONE_ROOT_TAG, b"BSBoneLOD#BoneRoot#");
        e.map(BONE_TAG, 0x10);
        e.mem.set_cstr(BONE_TAG, b"BSBoneLOD#Bone#");
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e
    }

    /// A node with the given extra-data texts and children.
    fn tree_node(e: &mut Engine, texts: &[&[u8]], children: &[u32]) -> u32 {
        let node = e.mem.alloc(0xb0);
        e.mem.set_u32(node, NODE_VTABLE);
        let extras = e.mem.alloc(4 * texts.len() as u32 + 4);
        for (i, text) in texts.iter().enumerate() {
            let data = e.mem.alloc(0x10);
            e.mem.set_u32(data, STRING_DATA_VTABLE);
            let chars = e.mem.alloc(text.len() as u32 + 1);
            e.mem.set_cstr(chars, text);
            e.mem.set_u32(data + 0xc, chars);
            e.mem.set_u32(extras + 4 * i as u32, data);
        }
        e.mem.set_u32(node + 0x10, extras);
        e.mem.set_u16(node + 0x14, texts.len() as u16);
        let list = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *child);
        }
        e.mem.set_u32(node + 0xa0, list);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        node
    }

    fn set_items(e: &Engine, set: u32) -> Vec<u32> {
        let base = e.mem.u32(set);
        (0..e.mem.u32(set + 8))
            .map(|i| e.mem.u32(base + 4 * i))
            .collect()
    }

    #[test]
    fn node_map_constructor_and_destructor() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        record(&mut e, &log, &[MEMSET, FREE], 0);
        let vtables = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = vtables.clone();
        e.register_double(NODE_MAP_REMOVE_ALL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a5_4060, &args![map, 0x25u32]).u32(), map);
        let table = e.mem.u32(map + 8);
        assert_eq!(e.mem.u32(map), NODE_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![table, 0, 0x94]]);
        e.call(0x00a5_40a0, &args![map]);
        assert_eq!(
            *vtables.borrow(),
            vec![NODE_MAP_VTABLE, NODE_MAP_BASE_VTABLE]
        );
        assert_eq!(calls_to(&log, FREE), vec![vec![table]]);
    }

    #[test]
    fn node_set_constructor_allocates_only_for_a_size() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        record(&mut e, &log, &[ALLOCATE], 0x5000);
        let set = e.mem.alloc(0xc);
        e.mem.set_u32(set + 8, 9);
        assert_eq!(e.call(0x00a5_4110, &args![set, 0u32]).u32(), set);
        assert_eq!(e.mem.u32(set), 0);
        assert_eq!(e.mem.u32(set + 4), 0);
        assert_eq!(e.mem.u32(set + 8), 0);
        assert!(calls_to(&log, ALLOCATE).is_empty());
        e.call(0x00a5_4110, &args![set, 3u32]);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![12]]);
        assert_eq!(e.mem.u32(set), 0x5000);
        assert_eq!(e.mem.u32(set + 4), 3);
    }

    #[test]
    fn set_array_deleting_destructor() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        record(&mut e, &log, &[FREE, MEMORY_MANAGER_FREE], 0);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x7000);
        assert_eq!(e.call(0x00a5_4160, &args![array, 0u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&log, FREE), vec![vec![0x7000]]);
        assert!(calls_to(&log, MEMORY_MANAGER_FREE).is_empty());
        e.call(0x00a5_4160, &args![array, 1u32]);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, array]]
        );
    }

    #[test]
    fn bone_roots_are_collected_without_their_children() {
        let mut e = tree_engine();
        let inner = tree_node(&mut e, &[b"BSBoneLOD#BoneRoot#"], &[]);
        let below_root = tree_node(&mut e, &[b"BSBoneLOD#BoneRoot#"], &[]);
        let root_child = tree_node(
            &mut e,
            &[b"other", b"x BSBoneLOD#BoneRoot# y"],
            &[below_root],
        );
        let top = tree_node(&mut e, &[b"nothing"], &[0, root_child, inner]);
        let set = e.mem.alloc(0xc);
        e.call(0x00c5_2c30, &args![top, set]);
        assert_eq!(set_items(&e, set), vec![root_child, inner]);
    }

    #[test]
    fn controllers_without_bone_roots_answer_false() {
        let mut e = tree_engine();
        let log: Log = Default::default();
        record(&mut e, &log, &[OPERATOR_NEW], 0);
        let top = tree_node(&mut e, &[b"nothing"], &[]);
        assert!(!e.call(0x00c5_2a80, &args![top]).bool());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn every_bone_root_gets_an_initialised_controller() {
        let mut e = tree_engine();
        let log: Log = Default::default();
        let root = tree_node(&mut e, &[b"BSBoneLOD#BoneRoot#"], &[]);
        let block = e.mem.alloc(0x4c);
        record(&mut e, &log, &[OPERATOR_NEW], block);
        record(
            &mut e,
            &log,
            &[
                TIME_CONTROLLER_CONSTRUCTOR,
                MEMSET,
                NODE_MAP_REMOVE_ALL,
                LOD_MAP_DESTRUCTOR,
                NODE_MAP_SET_AT,
                CLONE_MAP_LOOKUP,
                ARRAY_SET_AT,
                REMOVE_CONTROLLER,
            ],
            0,
        );
        let mut slots = vec![0u32; 0x27];
        slots[0x98 / 4] = 0x00ee_7020;
        e.put_vtable(BONE_LOD_CONTROLLER_VTABLE, &slots);
        record(&mut e, &log, &[0x00ee_7020], 0);
        e.register(LOD_MAP_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            let table = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 8, table);
            a[0].into_ret()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1] + 4);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16);
            Ret::default()
        });
        assert!(e.call(0x00c5_2a80, &args![root]).bool());
        assert_eq!(calls_to(&log, 0x00ee_7020), vec![vec![block, root]]);
        assert!(calls_to(&log, REMOVE_CONTROLLER).is_empty());
        // No bone: one LOD, its slot cleared; the LOD ends at 0.
        assert_eq!(e.mem.u32(block + 0x38), 1);
        assert_eq!(calls_to(&log, ARRAY_SET_AT).len(), 1);
        assert_eq!(e.mem.u32(block + 0x34), 0);
        assert_eq!(
            calls_to(&log, NODE_MAP_SET_AT),
            vec![vec![calls_to(&log, NODE_MAP_SET_AT)[0][0], root, 0]]
        );
    }

    #[test]
    fn bone_extra_data_maps_the_node_to_its_lod() {
        let mut e = tree_engine();
        let log: Log = Default::default();
        let child = tree_node(&mut e, &[b"BSBoneLOD#Bone#5"], &[]);
        let skipped = tree_node(&mut e, &[b"Bone#BSBoneLOD#Bone#1"], &[]);
        let top = tree_node(&mut e, &[b"none"], &[child, skipped]);
        record(
            &mut e,
            &log,
            &[NODE_MAP_SET_AT, LOD_MAP_SET_AT, MEMORY_MANAGER_FREE],
            0,
        );
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            0u32.into_ret()
        });
        // strtok_s on '#': keeps the rest in the context word.
        e.register(STRTOK_S, |e, a| {
            let start = if a[0] != 0 { a[0] } else { e.mem.u32(a[2]) };
            if start == 0 || e.mem.u8(start) == 0 {
                return 0u32.into_ret();
            }
            let mut end = start;
            while e.mem.u8(end) != 0 && e.mem.u8(end) != b'#' {
                end += 1;
            }
            if e.mem.u8(end) == b'#' {
                e.mem.set_u8(end, 0);
                e.mem.set_u32(a[2], end + 1);
            } else {
                e.mem.set_u32(a[2], end);
            }
            start.into_ret()
        });
        e.register(STRCMP, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        e.register(ATOL, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            text.parse::<u32>().unwrap().into_ret()
        });
        e.map(TOKEN_DELIMITERS, 4);
        e.map(BONE_LOD_TOKEN, 0x10);
        e.mem.set_cstr(BONE_LOD_TOKEN, b"BSBoneLOD");
        e.map(BONE_TOKEN, 8);
        e.mem.set_cstr(BONE_TOKEN, b"Bone");
        e.call(0x00c5_3030, &args![0x100u32, top, 0x200u32, 0x300u32]);
        assert_eq!(calls_to(&log, NODE_MAP_SET_AT), vec![vec![0x200, child, 5]]);
        assert_eq!(calls_to(&log, LOD_MAP_SET_AT), vec![vec![0x300, 5, 1]]);
        // Both copies (the matching one and the one whose first token is wrong) are freed.
        assert_eq!(calls_to(&log, MEMORY_MANAGER_FREE).len(), 2);
    }

    #[test]
    fn lods_propagate_up_from_the_children() {
        let mut e = tree_engine();
        let log: Log = Default::default();
        let child = tree_node(&mut e, &[], &[]);
        let top = tree_node(&mut e, &[], &[child]);
        record(&mut e, &log, &[NODE_MAP_SET_AT], 0);
        e.register_double(CLONE_MAP_LOOKUP, move |e, a| {
            if a[1] == child {
                e.mem.set_u32(a[2], 3);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let lod = e.mem.alloc(4);
        e.mem.set_u32(lod, 1);
        e.call(0x00c5_3280, &args![0x100u32, top, 0x200u32, lod]);
        assert_eq!(e.mem.u32(lod), 3);
        assert_eq!(calls_to(&log, NODE_MAP_SET_AT), vec![vec![0x200, top, 3]]);
    }

    #[test]
    fn initialisation_builds_a_set_per_lod() {
        let mut e = tree_engine();
        let log: Log = Default::default();
        let bone = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x4c);
        let root = tree_node(&mut e, &[], &[]);
        record(
            &mut e,
            &log,
            &[
                NODE_MAP_REMOVE_ALL,
                LOD_MAP_DESTRUCTOR,
                NODE_MAP_SET_AT,
                CLONE_MAP_LOOKUP,
            ],
            0,
        );
        // The node map's bucket table gets one item.
        e.register(MEMSET, |e, a| {
            e.mem.set_u32(a[0], 0x7777);
            Ret::default()
        });
        e.register(LOD_MAP_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 4, 1);
            let table = e.mem.alloc(4);
            e.mem.set_u32(table, 0x8888);
            e.mem.set_u32(a[0] + 8, table);
            a[0].into_ret()
        });
        e.register(LOD_MAP_GET_NEXT, |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], 1);
            Ret::default()
        });
        e.register_double(NODE_MAP_GET_NEXT, move |e, a| {
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], bone);
            e.mem.set_u32(a[3], 1);
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16);
            Ret::default()
        });
        let sets = log.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            sets.borrow_mut()
                .push((ARRAY_SET_AT, vec![a[1], e.mem.u32(a[2])]));
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            Ret::default()
        });
        let set = e.mem.alloc(0xc);
        record(&mut e, &log, &[MEMORY_MANAGER_ALLOCATE], set);
        e.mem.set_u32(this + 0x34, 0);
        assert!(e.call(0x00c5_2d80, &args![this, root]).bool());
        assert_eq!(e.mem.u32(this + 0x38), 2);
        assert_eq!(
            calls_to(&log, ARRAY_SET_AT),
            vec![vec![0, 0], vec![1, 0], vec![1, set]]
        );
        assert_eq!(set_items(&e, set), vec![bone]);
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_ffff);
        assert_eq!(calls_to(&log, LOD_MAP_DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&log, NODE_MAP_REMOVE_ALL).len(), 2);
    }

    /// A controller with the given sets (an empty slice is a null slot).
    fn controller(e: &mut Engine, sets: &[&[u32]], lod: u32, count: u32) -> u32 {
        let this = e.mem.alloc(0x4c);
        let base = e.mem.alloc(4 * sets.len() as u32 + 4);
        for (i, items) in sets.iter().enumerate() {
            if items.is_empty() {
                continue;
            }
            let set = e.mem.alloc(0xc);
            let storage = e.mem.alloc(4 * items.len() as u32);
            for (j, item) in items.iter().enumerate() {
                e.mem.set_u32(storage + 4 * j as u32, *item);
            }
            e.mem.set_u32(set, storage);
            e.mem.set_u32(set + 4, items.len() as u32);
            e.mem.set_u32(set + 8, items.len() as u32);
            e.mem.set_u32(base + 4 * i as u32, set);
        }
        e.mem.set_u32(this + 0x40, base);
        e.mem.set_u16(this + 0x46, sets.len() as u16);
        e.mem.set_u32(this + 0x34, lod);
        e.mem.set_u32(this + 0x38, count);
        this
    }

    #[test]
    fn register_streamables_answers_the_base_byte() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, a| a[1].into_ret());
        assert!(e.call(0x00c5_36a0, &args![0x100u32, 0x201u32]).bool());
        assert!(!e.call(0x00c5_36a0, &args![0x100u32, 0x200u32]).bool());
    }

    #[test]
    fn save_binary_writes_the_sets() {
        let mut e = Engine::new();
        let this = controller(&mut e, &[&[0x11, 0x22], &[]], 4, 2);
        let log: Log = Default::default();
        record(&mut e, &log, &[TIME_CONTROLLER_SAVE_BINARY], 0);
        let values = log.clone();
        let saves = move |address: u32| {
            let values = values.clone();
            move |e: &mut Engine, a: &[u32]| {
                values
                    .borrow_mut()
                    .push((address, vec![a[1], e.mem.u32(a[1])]));
                Ret::default()
            }
        };
        e.register_double(SAVE_FOUR_BYTES, saves(SAVE_FOUR_BYTES));
        e.register_double(SAVE_UNSIGNED, saves(SAVE_UNSIGNED));
        let stream = e.mem.alloc(4);
        e.put_vtable(0x0300_2000, &[0; 13]);
        e.mem.set_u32(0x0300_2000 + 0x30, 0x00ee_7030);
        e.mem.set_u32(stream, 0x0300_2000);
        record(&mut e, &log, &[0x00ee_7030], 0);
        e.call(0x00c5_36d0, &args![this, stream]);
        let four: Vec<u32> = calls_to(&log, SAVE_FOUR_BYTES)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(four, vec![4, 0]);
        assert_eq!(calls_to(&log, SAVE_FOUR_BYTES)[0][0], this + 0x34);
        let unsigned: Vec<u32> = calls_to(&log, SAVE_UNSIGNED).iter().map(|c| c[1]).collect();
        assert_eq!(unsigned, vec![2, 2, 2]);
        assert_eq!(
            calls_to(&log, 0x00ee_7030),
            vec![vec![stream, 0x11], vec![stream, 0x22]]
        );
    }

    #[test]
    fn is_equal_compares_the_sets_node_by_node() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| 1u32.into_ret());
        let node_vtable = 0x0300_3000;
        let mut slots = vec![0u32; 0x18];
        slots[0x5c / 4] = 0x00ee_7040;
        e.put_vtable(node_vtable, &slots);
        e.register(0x00ee_7040, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)).into_ret()
        });
        let node = |e: &mut Engine, tag: u32| {
            let n = e.mem.alloc(8);
            e.mem.set_u32(n, node_vtable);
            e.mem.set_u32(n + 4, tag);
            n
        };
        let (a1, a2, b1, b2) = (
            node(&mut e, 1),
            node(&mut e, 2),
            node(&mut e, 1),
            node(&mut e, 2),
        );
        let this = controller(&mut e, &[&[a1, a2], &[]], 1, 2);
        let same = controller(&mut e, &[&[b1, b2], &[]], 1, 2);
        assert!(e.call(0x00c5_3af0, &args![this, same]).bool());
        let swapped = controller(&mut e, &[&[b2, b1], &[]], 1, 2);
        assert!(!e.call(0x00c5_3af0, &args![this, swapped]).bool());
        let shorter = controller(&mut e, &[&[b1], &[]], 1, 2);
        assert!(!e.call(0x00c5_3af0, &args![this, shorter]).bool());
        let holes = controller(&mut e, &[&[], &[b1]], 1, 2);
        assert!(!e.call(0x00c5_3af0, &args![this, holes]).bool());
        let other_lod = controller(&mut e, &[&[b1, b2], &[]], 0, 2);
        assert!(!e.call(0x00c5_3af0, &args![this, other_lod]).bool());
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c5_3af0, &args![this, same]).bool());
    }

    #[test]
    fn viewer_strings_name_the_class_and_both_counts() {
        let mut e = Engine::new();
        let this = controller(&mut e, &[], 3, 4);
        e.map(BONE_LOD_CONTROLLER_RTTI, 8);
        e.mem.set_u32(BONE_LOD_CONTROLLER_RTTI, 0x0400_0000);
        e.register(VIEWER_STRING, |_, _| 0x51u32.into_ret());
        e.register(VIEWER_STRING_OV7, |_, a| (0x60 + a[1]).into_ret());
        e.register(VIEWER_STRING_OV8, |_, a| (0x70 + a[1]).into_ret());
        let log: Log = Default::default();
        let added = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            added
                .borrow_mut()
                .push((VIEWER_STRINGS_ADD, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_3c50, &args![this, 0x900u32]);
        let made = e.call_log.take().unwrap();
        assert!(made.contains(&(VIEWER_STRING, vec![0x0400_0000])));
        assert!(made.contains(&(VIEWER_STRING_OV7, vec![LOD_TEXT, 3])));
        assert!(made.contains(&(VIEWER_STRING_OV8, vec![NUM_LODS_TEXT, 4])));
        assert_eq!(
            calls_to(&log, VIEWER_STRINGS_ADD),
            vec![vec![0x900, 0x51], vec![0x900, 0x63], vec![0x900, 0x74]]
        );
    }

    #[test]
    fn loader_registration_names_the_create_function() {
        let mut e = Engine::new();
        let log: Log = Default::default();
        record(&mut e, &log, &[REGISTER_LOADER], 0);
        e.call(0x00c5_3cd0, &args![]);
        assert_eq!(
            calls_to(&log, REGISTER_LOADER),
            vec![vec![CLASS_NAME_TEXT, CREATE_FUNCTION]]
        );
    }

    #[test]
    fn any_bone_needs_a_non_null_node() {
        let mut e = Engine::new();
        let empty = controller(&mut e, &[&[], &[0, 0]], 0, 1);
        assert!(!e.call(0x00c5_3cf0, &args![empty]).bool());
        let full = controller(&mut e, &[&[], &[0, 0x40]], 0, 1);
        assert!(e.call(0x00c5_3cf0, &args![full]).bool());
    }

    #[test]
    fn the_target_must_be_a_node() {
        let mut e = tree_engine();
        let this = controller(&mut e, &[], 0, 0);
        assert!(!e.call(0x00c5_3ed0, &args![this]).bool());
        let target = tree_node(&mut e, &[], &[]);
        e.mem.set_u32(this + 0x2c, target);
        assert!(e.call(0x00c5_3ed0, &args![this]).bool());
        e.register(IS_NODE, |_, _| 0x100u32.into_ret());
        assert!(e.call(0x00c5_3ed0, &args![this]).bool());
        e.register(IS_NODE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c5_3ed0, &args![this]).bool());
    }
}
