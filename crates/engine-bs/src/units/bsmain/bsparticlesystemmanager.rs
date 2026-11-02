//! `BSMain/bsparticlesystemmanager.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5ea70` (named `NiNode::GetViewerStrings` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const VIEWER_STRINGS_TARGET_00A5EA70: u32 = 0x00a5_ea70;

// Translated from 00c51570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::GetViewerStrings` (engine map vtable name, vtable word 010c245c):
/// forwards `this` and the string array to `00a5ea70` (`NiNode::GetViewerStrings`). Returns
/// nothing: Xbox PDB `?GetViewerStrings@NiObject@@UAAXPAV?$NiTPrimitiveArray@PAD@@@Z` is void;
/// `strings` is the `NiTPrimitiveArray<char *>` pointer. Parameter count from the `RET 4` of
/// both bodies.
pub fn bs_particle_system_manager_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(VIEWER_STRINGS_TARGET_00A5EA70, &args![this, strings]);
}

// Translated from 00c516f0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01203110 without reading the receiver. The map names this body `BSParticleSystemManager::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c516f0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_3110
}

/// `NiNode`'s constructor (`00a5ecb0`, `thiscall`, one word: the child array's size).
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `BSParticleSystemManager`'s vtable (`010c23fc`, RTTI `.?AVBSParticleSystemManager@@`).
const BS_PARTICLE_SYSTEM_MANAGER_VTABLE: u32 = 0x010c_23fc;
/// The memory manager singleton (`011f6238`) and its allocate (`00aa3e40`, `thiscall`, size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// CRT `memset` (`00ec61c0`).
const MEMSET: u32 = 0x00ec_61c0;
/// `BSTaskletManager::QInstance` (engine map, `00b00a00`, no arguments) and
/// `BSTaskletManager::CreateTaskGroup` (engine map, `00b00a80`, `thiscall`, `RET 4`: the
/// group word's address).
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;
const TASKLET_MANAGER_CREATE_TASK_GROUP: u32 = 0x00b0_0a80;

// Translated from 00c503d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::BSParticleSystemManager` (Xbox PDB), `thiscall`, no stack
/// words; answers `this`. Constructs the `NiNode` (child array size 0), sets the vtable, the
/// count at `+0xb0` to 10 and the 0x28-byte array at `+0xac` (memory manager), fills
/// `count * 4` bytes of it with 0xff, makes a one-word block (cleared; null when the
/// allocation fails) at `+0xb4` and has the tasklet manager create a task group into it
/// (`CreateTaskGroup` with the word at `+0xb4`).
// Translation incomplete: 00c503d0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn bs_particle_system_manager_bs_particle_system_manager(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_NODE_CONSTRUCT, &args![base, 0u32]);
    e.mem.set_u32(base, BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
    e.mem.set_u32(base.wrapping_add(0xb0), 10);
    let array = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x28u32])
        .u32();
    e.mem.set_u32(base.wrapping_add(0xac), array);
    let bytes = e.mem.u32(base.wrapping_add(0xb0)).wrapping_shl(2);
    let array = e.mem.u32(base.wrapping_add(0xac));
    e.call(MEMSET, &args![array, 0xffff_ffffu32, bytes]);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let block = if block != 0 {
        e.mem.set_u32(block, 0);
        block
    } else {
        0
    };
    e.mem.set_u32(base.wrapping_add(0xb4), block);
    let group = e.mem.u32(base.wrapping_add(0xb4));
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    e.call(TASKLET_MANAGER_CREATE_TASK_GROUP, &args![manager, group]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5_03d0,
            bs_particle_system_manager_bs_particle_system_manager(Ptr) -> Ptr
        ),
        entry!(0x00c5_16f0, fn_00c516f0(Ptr) -> u32),
        entry!(
            0x00c5_1570,
            bs_particle_system_manager_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c5_0ec0, bs_particle_system_manager_find_child_index(Ptr, u32) -> u32),
        entry!(0x00c5_0f30, fn_00c50f30(Ptr, u32) -> u32),
        entry!(
            0x00c5_05c0,
            bs_particle_system_manager_add_emitter(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c5_09f0,
            bs_particle_system_manager_attach_child(Ptr, Ptr, u32)
        ),
        entry!(0x00c5_04e0, bs_particle_system_manager_destructor(Ptr)),
        entry!(
            0x00c5_1590,
            bs_particle_system_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c5_0610, fn_00c50610(Ptr, u32)),
        entry!(0x00c5_0930, fn_00c50930(Ptr)),
        entry!(0x00c5_09a0, fn_00c509a0(Ptr, u32)),
        entry!(
            0x00c5_0b80,
            bs_particle_system_manager_detach_child(Ptr, Ptr)
        ),
        entry!(
            0x00c5_0c30,
            bs_particle_system_manager_detach_child_ov2(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00c5_0d20,
            bs_particle_system_manager_detach_child_at(Ptr, u32)
        ),
        entry!(
            0x00c5_0dd0,
            bs_particle_system_manager_detach_child_at_ov2(Ptr, u32, Ptr) -> u32
        ),
        entry!(
            0x00c5_0df0,
            bs_particle_system_manager_set_at(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c5_0ea0,
            bs_particle_system_manager_set_at_ov2(Ptr, u32, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00c5_0f90,
            bs_particle_system_manager_check_emitter_recurse(Ptr, u32) -> bool
        ),
        entry!(0x00c5_11f0, fn_00c511f0(Ptr) -> bool),
        entry!(0x00c5_1250, fn_00c51250(Ptr, u32) -> bool),
        entry!(0x00c5_13f0, fn_00c513f0(Ptr) -> u32),
        entry!(
            0x00c5_14c0,
            bs_particle_system_manager_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00c5_1550, fn_00c51550(Ptr, Ptr, Ptr)),
        entry!(
            0x00c5_15d0,
            update_master_psys_tasklet_data_vector_deleting_destructor(Ptr, u32) -> u32
        ),
        entry!(0x00c5_1640, fn_00c51640(Ptr) -> Ptr),
        entry!(0x00c5_16a0, update_master_psys_tasklet_data_process(Ptr)),
        entry!(0x00c5_16d0, fn_00c516d0(Ptr)),
        entry!(0x00c5_1700, fn_00c51700(Ptr, u32)),
    ]
}

// ---- Batch b012 --------------------------------------------------------------------------

/// `BSParticleSystemManager` on PC: `NiNode::m_kChildren` elements at `+0xa0`, size
/// `+0xa6`, non-null count `+0xa8`; the index table (one child index per master system,
/// -1 for none) at `+0xac` with its length at `+0xb0`; the task group word at `+0xb4`.
const CHILDREN: u32 = 0xa0;
const CHILD_COUNT: u32 = 0xa6;
const CHILDREN_USED: u32 = 0xa8;
const INDEX_TABLE: u32 = 0xac;
const INDEX_COUNT: u32 = 0xb0;
const TASK_GROUP: u32 = 0xb4;
/// The memory manager's free (`00aa4060`, `thiscall`, one word).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// CRT `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiNode::AttachChild` (`00a5ed10`, `RET 8`) and the `NiNode` destructor body
/// (`00a5e930`).
const NI_NODE_ATTACH_CHILD: u32 = 0x00a5_ed10;
const NI_NODE_DESTRUCTOR: u32 = 0x00a5_e930;
/// `BSMasterParticleSystem::GetRTTI` answers this record (`0120436c`).
const MASTER_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_436c;
/// `00c5dcd0` (`thiscall`, `RET 4`): adds an emitter to a master particle system.
const MASTER_SYSTEM_ADD_EMITTER: u32 = 0x00c5_dcd0;
/// `005b5e40` (`cdecl`, variable arguments; engine-map unit `tesscriptfunctions.cpp`): the
/// debug print `AddEmitter` reports a missing system with, and its format (`010c2500`,
/// "DEFAULT: Trying to add an emitter (%s) to a master particle system (idx: %d) ...").
const DEBUG_PRINT: u32 = 0x005b_5e40;
const MISSING_SYSTEM_FORMAT: u32 = 0x010c_2500;
/// `BSTaskletManager::DestroyTaskGroup` (`00b00c20`) and the other task-group calls of
/// `00c50610` (`00b00ae0`, `00b00b40`, `00b00bc0`), all `thiscall` on the manager.
const TASKLET_MANAGER_DESTROY_TASK_GROUP: u32 = 0x00b0_0c20;
const TASK_GROUP_00B00AE0: u32 = 0x00b0_0ae0;
const TASK_GROUP_00B00B40: u32 = 0x00b0_0b40;
const TASK_GROUP_00B00BC0: u32 = 0x00b0_0bc0;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`, five words), the
/// element constructor and destructor `00c50610` gives it (`00c51640`, `00c516d0`) and the
/// `BSTasklet` vtable of its stack tasklet (`0106c5d8`).
const EH_VECTOR_CONSTRUCTOR: u32 = 0x00ec_782f;
const TASK_ELEMENT_CONSTRUCTOR: u32 = 0x00c5_1640;
const TASK_ELEMENT_DESTRUCTOR: u32 = 0x00c5_16d0;
const TASKLET_VTABLE: u32 = 0x0106_c5d8;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized delete.
const SIZED_DELETE: u32 = 0x00aa_1460;

// Translated from 00c50ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::FindChildIndex` (Xbox PDB; `thiscall`, `RET 4`): the first
/// index below the non-null child count (`+0xa8`, read once) whose child is `child`, or
/// `0xffffffff`.
pub fn bs_particle_system_manager_find_child_index(e: &mut Engine, this: Ptr, child: u32) -> u32 {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + CHILDREN_USED));
    (0..count)
        .find(|index| e.mem.u32(e.mem.u32(base + CHILDREN) + 4 * index) == child)
        .unwrap_or(0xffff_ffff)
}

// Translated from 00c50f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsparticlesystemmanager.obj`; `thiscall`, `RET 4`): the
/// child the index table names for master system `index`, or 0 when `index` is past the
/// table or its entry is -1.
pub fn fn_00c50f30(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    if index >= e.mem.u32(base + INDEX_COUNT) {
        return 0;
    }
    let child_index = e
        .mem
        .u32(e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index));
    if child_index == 0xffff_ffff {
        return 0;
    }
    e.mem.u32(
        e.mem
            .u32(base + CHILDREN)
            .wrapping_add(child_index.wrapping_mul(4)),
    )
}

// Translated from 00c505c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::AddEmitter` (Xbox PDB; `thiscall`, `RET 8`): hands `emitter`
/// to the master system at `index` (`00c50f30`, then `00c5dcd0`); without one, the debug
/// print `005b5e40` reports the emitter's name (its word at `+8`) and the index.
pub fn bs_particle_system_manager_add_emitter(e: &mut Engine, this: Ptr, index: u32, emitter: Ptr) {
    let system = fn_00c50f30(e, this, index);
    if system != 0 {
        e.call(MASTER_SYSTEM_ADD_EMITTER, &args![system, emitter]);
    } else {
        let name = e.mem.u32(emitter.addr() + 8);
        e.call(DEBUG_PRINT, &args![MISSING_SYSTEM_FORMAT, name, index]);
    }
}

// Translated from 00c509f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::AttachChild` (Xbox PDB; `thiscall`, `RET 8`, the child and a
/// byte): only a `BSMasterParticleSystem` (its `GetRTTI`, slot 8, answers `0120436c`)
/// with an index (`+0xc4`, not -1) is attached, and only when its table entry is -1 or
/// past the table. A table too short grows to `((index - length + 1) / 10 + 1) * 10 +
/// length` entries (memory manager; the byte count saturates to `0xffffffff` on overflow),
/// filled with -1, the old entries copied over and the old table freed. Then
/// `NiNode::AttachChild` and the child's index (`FindChildIndex`) stored in the entry.
pub fn bs_particle_system_manager_attach_child(e: &mut Engine, this: Ptr, child: Ptr, flag: u32) {
    let base = this.addr();
    let is_master =
        !child.is_null() && e.vcall(child.addr(), 8, &[]).u32() == MASTER_PARTICLE_SYSTEM_RTTI;
    if !is_master {
        return;
    }
    let index = e.mem.u32(child.addr() + 0xc4);
    if index == 0xffff_ffff {
        return;
    }
    let length = e.mem.u32(base + INDEX_COUNT);
    if index < length {
        if e.mem
            .u32(e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index))
            != 0xffff_ffff
        {
            return;
        }
    } else {
        let blocks = index.wrapping_sub(length).wrapping_add(1) / 10 + 1;
        let new_length = blocks
            .wrapping_mul(10)
            .wrapping_add(e.mem.u32(base + INDEX_COUNT));
        let bytes = if u64::from(new_length) * 4 > u64::from(u32::MAX) {
            0xffff_ffff
        } else {
            new_length * 4
        };
        let table = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        e.call(MEMSET, &args![table, 0xffff_ffffu32, new_length << 2]);
        let old_bytes = e.mem.u32(base + INDEX_COUNT) << 2;
        let old = e.mem.u32(base + INDEX_TABLE);
        e.call(MEMCPY, &args![table, old, old_bytes]);
        let old = e.mem.u32(base + INDEX_TABLE);
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, old]);
        e.mem.set_u32(base + INDEX_TABLE, table);
        e.mem.set_u32(base + INDEX_COUNT, new_length);
    }
    e.call(NI_NODE_ATTACH_CHILD, &args![this, child, flag & 0xff]);
    let child_index = bs_particle_system_manager_find_child_index(e, this, child.addr());
    let entry = e.mem.u32(base + INDEX_TABLE).wrapping_add(4 * index);
    e.mem.set_u32(entry, child_index);
}

// Translated from 00c504e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::~BSParticleSystemManager` (Xbox PDB; `thiscall`): sets the
/// vtable, frees the index table (memory manager) and clears it and its length, has the
/// tasklet manager destroy the task group (`DestroyTaskGroup` with the word at `+0xb4`),
/// frees the group's block when there is one (behind a constant-true test), and ends in
/// the `NiNode` destructor body (`00a5e930`).
// Translation incomplete: 00c504e0 - the C++ exception frame (FS:[0] chain, unwind state 0) and the /GS cookie word are not translated.
pub fn bs_particle_system_manager_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
    let table = e.mem.u32(base + INDEX_TABLE);
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, table]);
    e.mem.set_u32(base + INDEX_TABLE, 0);
    e.mem.set_u32(base + INDEX_COUNT, 0);
    let group = e.mem.u32(base + TASK_GROUP);
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
    e.call(TASKLET_MANAGER_DESTROY_TASK_GROUP, &args![manager, group]);
    let block = e.mem.u32(base + TASK_GROUP);
    // `MOV EDX,1; AND EDX,1; JZ`: the free always runs for a block.
    if block != 0 && std::hint::black_box(1u32) & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
    }
    e.call(NI_NODE_DESTRUCTOR, &args![this]);
}

// Translated from 00c51590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`):
/// the destructor (`00c504e0`), then the sized delete of 0xb8 bytes when bit 0 of `flags`
/// is set. Answers `this`.
pub fn bs_particle_system_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    bs_particle_system_manager_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xb8u32]);
    }
    this
}

// Translated from 00c50610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bsparticlesystemmanager.obj`; `thiscall`, `RET 4`):
/// updates the children with `time`, then runs the manager's own slot `0xfc`. Each
/// non-null child gets its slot `0xa4` with `(time, 0)`. A local flag the exe sets to
/// false (so the rest never runs; translated as the exe has it) selects a threaded form:
/// an array of 0x20-byte task entries (count word first; `eh vector constructor iterator`
/// with `00c51640`/`00c516d0`), the task group reset (its object's slot 8 with 0, then
/// `00b00ae0`), one `BSTasklet` per child queued with `00b00b40` (the entry holding the
/// child at `+0x18` and `time` at `+0x1c`), then `00b00bc0`, the group object's slot 8 with
/// 99 and slot 4 with 0 (when its byte at `+4` is clear) after queuing, and the array
/// deleted (its slot 0 with 3, or a plain free when the count word is 0).
// Translation incomplete: 00c50610 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1) and the /GS cookie word are not translated.
pub fn fn_00c50610(e: &mut Engine, this: Ptr, time: u32) {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + CHILD_COUNT));
    let threaded = std::hint::black_box(false);
    let mut queued = 0u32;
    let mut entries = 0u32;
    if threaded {
        let bytes = if u64::from(count) * 0x20 > u64::from(u32::MAX) {
            0xffff_ffff
        } else {
            count * 0x20
        };
        let bytes = bytes.saturating_add(4);
        let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        if block != 0 {
            e.mem.set_u32(block, count);
            e.call(
                EH_VECTOR_CONSTRUCTOR,
                &args![
                    block + 4,
                    0x20u32,
                    count,
                    TASK_ELEMENT_CONSTRUCTOR,
                    TASK_ELEMENT_DESTRUCTOR
                ],
            );
            entries = block + 4;
        }
        // The word at +0xb4 is the block holding the group object.
        let group = e.mem.u32(base + TASK_GROUP);
        let object = e.mem.u32(group);
        if object != 0 {
            e.vcall(object, 8, &args![0u32]);
        }
        let group = e.mem.u32(base + TASK_GROUP);
        let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
        e.call(TASK_GROUP_00B00AE0, &args![manager, group]);
    }
    let mut index = 0u32;
    while index < count {
        let child = e.mem.u32(e.mem.u32(base + CHILDREN) + 4 * index);
        if child != 0 {
            if threaded {
                e.with_stack(8, |e, tasklet| {
                    e.mem.set_u32(tasklet.addr(), TASKLET_VTABLE);
                    e.mem.set_u32(tasklet.addr() + 4, 0);
                    let entry = entries.wrapping_add(index << 5);
                    e.mem.set_u32(entry + 0x18, child);
                    e.mem.set_u32(entry + 0x1c, time);
                    e.mem.set_u32(tasklet.addr() + 4, entry);
                    let group = e.mem.u32(base + TASK_GROUP);
                    let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
                    e.call(TASK_GROUP_00B00B40, &args![manager, group, tasklet, 0u32]);
                    queued += 1;
                    e.mem.set_u32(tasklet.addr(), TASKLET_VTABLE);
                });
            } else {
                e.vcall(child, 0xa4, &args![time, 0u32]);
            }
        }
        index += 1;
    }
    if threaded {
        let group = e.mem.u32(base + TASK_GROUP);
        let manager = e.call(TASKLET_MANAGER_INSTANCE, &args![]).u32();
        e.call(TASK_GROUP_00B00BC0, &args![manager, group]);
        if queued != 0 {
            let group = e.mem.u32(base + TASK_GROUP);
            if e.mem.u32(group) != 0 {
                e.vcall(e.mem.u32(group), 8, &args![99u32]);
            }
            let group = e.mem.u32(base + TASK_GROUP);
            let object = e.mem.u32(group);
            if object != 0 && e.mem.u8(object + 4) == 0 {
                e.vcall(object, 4, &args![0u32]);
            }
        }
        if entries != 0 {
            if e.mem.u32(entries - 4) != 0 {
                e.vcall(entries, 0, &args![3u32]);
            } else {
                e.call(MEMORY_FREE, &args![MEMORY_MANAGER, entries - 4]);
            }
        }
    }
    e.vcall(base, 0xfc, &[]);
}

// ---- Batch n067 ---------------------------------------------------------------------------

/// The imported `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiNode` vtable slots the manager calls on itself: `AttachChild` (0xdc), `DetachChild`
/// with an out pointer (0xe4), `DetachChildAt` with an out pointer (0xec), `SetAt` with an
/// out pointer (0xf4).
const SLOT_ATTACH_CHILD: u32 = 0xdc;
const SLOT_DETACH_CHILD: u32 = 0xe4;
const SLOT_DETACH_CHILD_AT: u32 = 0xec;
const SLOT_SET_AT: u32 = 0xf4;
/// `NiNode::DetachChild_ov2` (Xbox PDB; `this`, the child, the out pointer's address).
const NI_NODE_DETACH_CHILD: u32 = 0x00a5_e780;
/// An `NiPointer` assignment from another `NiPointer` (`006e5cc0`, map name
/// `NiPointer<PathingDebugGeometryData>::operator=`; `this` the target, the source's
/// address) and from a raw pointer (`0066b0d0`, unnamed; `this` the target, the pointer;
/// returns `this`).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
const NI_POINTER_ASSIGN_RAW: u32 = 0x0066_b0d0;
/// `00c5dea0` (`bsmasterparticlesystem.obj`, unnamed; `thiscall` on a child, no stack
/// words).
const MASTER_SYSTEM_00C5DEA0: u32 = 0x00c5_dea0;
/// The debug print's formats: removing a master system that is not loaded (`010c2580`),
/// a system not emitting in world space (`010c2678`), a system not pointing to the proper
/// root (`010c2628`), a system without a multi-target emitter (`010c25d8`).
const NOT_LOADED_FORMAT: u32 = 0x010c_2580;
const NOT_WORLD_SPACE_FORMAT: u32 = 0x010c_2678;
const WRONG_ROOT_FORMAT: u32 = 0x010c_2628;
const NO_EMITTER_FORMAT: u32 = 0x010c_25d8;
/// The `NiRTTI` records of `NiParticleSystem` (`012024e0`), `BSPSysMultiTargetEmitterCtlr`
/// (`01203340`) and `NiPSysVolumeEmitter` (`012027d8`): their names' static initializers
/// (`00fbaf30`, `00fbc7e0`, `00fbb680`) name them.
const NI_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_24e0;
const MULTI_TARGET_EMITTER_RTTI: u32 = 0x0120_3340;
const VOLUME_EMITTER_RTTI: u32 = 0x0120_27d8;
/// `operator new(size)` (cdecl), the `BSMasterParticleSystem` constructor (`00c5da70`) and
/// `BSMasterParticleSystem::UpdateParticleSystemList` (`00c5de70`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MASTER_SYSTEM_CONSTRUCT: u32 = 0x00c5_da70;
const MASTER_SYSTEM_UPDATE_LIST: u32 = 0x00c5_de70;
/// The manager's size.
const BS_PARTICLE_SYSTEM_MANAGER_SIZE: u32 = 0xb8;
/// `00a5dc80` (unnamed, `ninode.cpp` range): the `NiNode` member copy the manager's
/// `CopyMembers` forwards to.
const NI_NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, `stdcall`: the array, the
/// element size, the count, the destructor) and the tasklet-data destructor body
/// (`00b00ec0`, `thiscall`).
const EH_VECTOR_DESTRUCTOR: u32 = 0x00ec_5fce;
const TASKLET_DATA_DESTRUCT: u32 = 0x00b0_0ec0;
/// The vtables the task element's constructor stores in turn (`BSTaskletData`,
/// `BSWin32TaskletData`, `UpdateMasterPsysTaskletData`, by their RTTI locators).
const TASKLET_DATA_VTABLE: u32 = 0x0106_c814;
const WIN32_TASKLET_DATA_VTABLE: u32 = 0x0106_c800;
const UPDATE_MASTER_PSYS_TASKLET_DATA_VTABLE: u32 = 0x010c_256c;

/// The `NiPointer` local the detach functions pass by address: it starts null (the
/// exe's increment for a non-null start is behind a test of that null, so it never runs;
/// translated with the null black-boxed), `body` runs with its address, and the pointer
/// left in it is released (`InterlockedDecrement` on its count at +4, its slot 4 when that
/// reaches 0).
fn with_node_pointer(e: &mut Engine, body: impl FnOnce(&mut Engine, u32)) {
    e.with_stack(4, |e, local| {
        let start = std::hint::black_box(0u32);
        e.mem.set_u32(local.addr(), start);
        if start != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![start + 4]);
        }
        body(e, local.addr());
        let pointer = e.mem.u32(local.addr());
        if pointer != 0 && e.call(INTERLOCKED_DECREMENT, &args![pointer + 4]).u32() == 0 {
            e.vcall(pointer, 4, &[]);
        }
    });
}

// Translated from 00c50930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `00c5dea0` on each non-null
/// child below the child array's size (`+0xa6`, read once).
pub fn fn_00c50930(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = u32::from(e.mem.u16(base + CHILD_COUNT));
    for index in 0..count {
        let child = e
            .mem
            .u32(e.mem.u32(base + CHILDREN).wrapping_add(4 * index));
        if child != 0 {
            e.call(MASTER_SYSTEM_00C5DEA0, &args![child]);
        }
    }
}

// Translated from 00c509a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): removes the master system at `index`
/// ([`fn_00c50f30`] finds it, [`fn_00c51700`] detaches it); without one, the debug print
/// `005b5e40` reports the index (format `010c2580`).
pub fn fn_00c509a0(e: &mut Engine, this: Ptr, index: u32) {
    let system = fn_00c50f30(e, this, index);
    if system != 0 {
        fn_00c51700(e, this, system);
    } else {
        e.call(DEBUG_PRINT, &args![NOT_LOADED_FORMAT, index]);
    }
}

// Translated from 00c50b80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c50b80 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSParticleSystemManager::DetachChild` (Xbox PDB; `RET 4`): the manager's own slot 0xe4
/// (`DetachChild` into a local `NiPointer`), the detached child then released.
pub fn bs_particle_system_manager_detach_child(e: &mut Engine, this: Ptr, child: Ptr) {
    with_node_pointer(e, |e, local| {
        e.vcall(this.addr(), SLOT_DETACH_CHILD, &args![child, local]);
    });
}

// Translated from 00c50c30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c50c30 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSParticleSystemManager::DetachChild_ov2` (Xbox PDB; `RET 8`): `NiNode::DetachChild_ov2`
/// into a local `NiPointer`; a detached child's entry in the index table (its index at
/// `+0xc4`) becomes -1; the child is assigned to the `NiPointer` at `result` (`006e5cc0`)
/// and the local released.
pub fn bs_particle_system_manager_detach_child_ov2(
    e: &mut Engine,
    this: Ptr,
    child: Ptr,
    result: Ptr,
) {
    with_node_pointer(e, |e, local| {
        e.call(NI_NODE_DETACH_CHILD, &args![this, child, local]);
        let detached = e.mem.u32(local);
        if detached != 0 {
            let index = e.mem.u32(detached + 0xc4);
            let table = e.mem.u32(this.addr() + INDEX_TABLE);
            e.mem
                .set_u32(table.wrapping_add(index.wrapping_mul(4)), 0xffff_ffff);
        }
        e.call(NI_POINTER_ASSIGN, &args![result, local]);
    });
}

// Translated from 00c50d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c50d20 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSParticleSystemManager::DetachChildAt` (Xbox PDB; `RET 4`): the manager's own slot
/// 0xec (`DetachChildAt` into a local `NiPointer`), the detached child then released.
pub fn bs_particle_system_manager_detach_child_at(e: &mut Engine, this: Ptr, index: u32) {
    with_node_pointer(e, |e, local| {
        e.vcall(this.addr(), SLOT_DETACH_CHILD_AT, &args![index, local]);
    });
}

// Translated from 00c50dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::DetachChildAt_ov2` (Xbox PDB; `RET 8`): detaches nothing; the
/// `NiPointer` at `result` is set to null (`0066b0d0`), whose answer (`result`) is
/// returned. `index` is not read.
pub fn bs_particle_system_manager_detach_child_at_ov2(
    e: &mut Engine,
    _this: Ptr,
    _index: u32,
    result: Ptr,
) -> u32 {
    e.call(NI_POINTER_ASSIGN_RAW, &args![result, 0u32]).u32()
}

// Translated from 00c50df0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c50df0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSParticleSystemManager::SetAt` (Xbox PDB; `RET 8`): the manager's own slot 0xf4
/// (`SetAt` with the index, the child and a local `NiPointer` for the one replaced), that
/// one then released.
pub fn bs_particle_system_manager_set_at(e: &mut Engine, this: Ptr, index: u32, child: Ptr) {
    with_node_pointer(e, |e, local| {
        e.vcall(this.addr(), SLOT_SET_AT, &args![index, child, local]);
    });
}

// Translated from 00c50ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::SetAt_ov2` (Xbox PDB; `RET 0xc`): sets nothing; the
/// `NiPointer` at `result` is set to null (`0066b0d0`), whose answer (`result`) is
/// returned. `index` and `child` are not read.
pub fn bs_particle_system_manager_set_at_ov2(
    e: &mut Engine,
    _this: Ptr,
    _index: u32,
    _child: Ptr,
    result: Ptr,
) -> u32 {
    e.call(NI_POINTER_ASSIGN_RAW, &args![result, 0u32]).u32()
}

/// Whether `object` is not null and its `GetRTTI` (slot 8) answers `rtti`.
fn rtti_is(e: &mut Engine, object: u32, rtti: u32) -> bool {
    if object == 0 {
        false
    } else {
        e.vcall(object, 8, &[]).u32() == rtti
    }
}

/// The node's children (`IsNode`, slot 0xc, when the node is not null) passed one by one
/// to `visit` while it answers true: false at the first that answers false, true
/// otherwise (also without children). The child count (`+0xa6`) is read once.
fn children_all(
    e: &mut Engine,
    node: u32,
    mut visit: impl FnMut(&mut Engine, u32) -> bool,
) -> bool {
    let as_node = if node != 0 {
        e.vcall(node, 0xc, &[]).u32()
    } else {
        0
    };
    if as_node != 0 {
        let count = u32::from(e.mem.u16(as_node + CHILD_COUNT));
        for index in 0..count {
            let child = e
                .mem
                .u32(e.mem.u32(as_node + CHILDREN).wrapping_add(4 * index));
            if child != 0 && !visit(e, child) {
                return false;
            }
        }
    }
    true
}

// Translated from 00c50f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSParticleSystemManager::CheckEmitterRecurse` (Xbox PDB; cdecl: a node and a root, or
/// 0 for none). False for a null node. A node whose slot 0x30 (`IsParticlesGeom`) answers
/// non-zero must be an `NiParticleSystem` (`GetRTTI` `012024e0`) emitting in world space
/// (byte `+0xc4`, else reported with `010c2678`), whose controller list (`+0xc`, next at
/// `+0x30`) holds a `BSPSysMultiTargetEmitterCtlr` (`01203340`): with a root, its word at
/// `+0x5c` must be the root (else reported with `010c2628`); then true when its emitter
/// (`+0x3c`) is not null and its `NiRTTI` chain (slot 8, base at +4) reaches
/// `NiPSysVolumeEmitter` (`012027d8`). Without such a controller (or not a particle
/// system) it is reported with `010c25d8`: false. Any other node: every child must pass
/// (recursively, same root).
pub fn bs_particle_system_manager_check_emitter_recurse(
    e: &mut Engine,
    node: Ptr,
    root: u32,
) -> bool {
    let node = node.addr();
    if node == 0 {
        return false;
    }
    if e.vcall(node, 0x30, &[]).u32() == 0 {
        return children_all(e, node, |e, child| {
            bs_particle_system_manager_check_emitter_recurse(e, Ptr::new(child), root)
        });
    }
    if rtti_is(e, node, NI_PARTICLE_SYSTEM_RTTI) {
        if e.mem.u8(node + 0xc4) == 0 {
            let name = e.mem.u32(node + 8);
            e.call(DEBUG_PRINT, &args![NOT_WORLD_SPACE_FORMAT, name]);
            return false;
        }
        let mut controller = e.mem.u32(node + 0xc);
        while controller != 0 {
            if rtti_is(e, controller, MULTI_TARGET_EMITTER_RTTI) {
                let emitter = e.mem.u32(controller + 0x3c);
                if root != 0 && e.mem.u32(controller + 0x5c) != root {
                    let name = e.mem.u32(node + 8);
                    e.call(DEBUG_PRINT, &args![WRONG_ROOT_FORMAT, name]);
                    return false;
                }
                if emitter == 0 {
                    return false;
                }
                let mut rtti = e.vcall(emitter, 8, &[]).u32();
                while rtti != 0 {
                    if rtti == VOLUME_EMITTER_RTTI {
                        return true;
                    }
                    rtti = e.mem.u32(rtti + 4);
                }
                return false;
            }
            controller = e.mem.u32(controller + 0x30);
        }
    }
    let name = e.mem.u32(node + 8);
    e.call(DEBUG_PRINT, &args![NO_EMITTER_FORMAT, name]);
    false
}

// Translated from 00c511f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): for a `BSMasterParticleSystem` (its
/// `GetRTTI` answers `0120436c`) the emitter check with itself as the root
/// ([`bs_particle_system_manager_check_emitter_recurse`]); false otherwise.
pub fn fn_00c511f0(e: &mut Engine, node: Ptr) -> bool {
    if rtti_is(e, node.addr(), MASTER_PARTICLE_SYSTEM_RTTI) {
        bs_particle_system_manager_check_emitter_recurse(e, node, node.addr())
    } else {
        false
    }
}

// Translated from 00c51250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a node and a master system): the counterpart of the
/// emitter check that links them. False for a null node. A node whose slot 0x30 answers
/// non-zero: an `NiParticleSystem` gets its byte `+0xc4` set and its first
/// `BSPSysMultiTargetEmitterCtlr` controller its word `+0x5c` set to `system`: true;
/// without one (or not a particle system) it is reported with `010c25d8`: false. Any other
/// node: every child must pass (recursively).
pub fn fn_00c51250(e: &mut Engine, node: Ptr, system: u32) -> bool {
    let node = node.addr();
    if node == 0 {
        return false;
    }
    if e.vcall(node, 0x30, &[]).u32() == 0 {
        return children_all(e, node, |e, child| fn_00c51250(e, Ptr::new(child), system));
    }
    if rtti_is(e, node, NI_PARTICLE_SYSTEM_RTTI) {
        e.mem.set_u8(node + 0xc4, 1);
        let mut controller = e.mem.u32(node + 0xc);
        while controller != 0 {
            if rtti_is(e, controller, MULTI_TARGET_EMITTER_RTTI) {
                e.mem.set_u32(controller + 0x5c, system);
                return true;
            }
            controller = e.mem.u32(controller + 0x30);
        }
    }
    let name = e.mem.u32(node + 8);
    e.call(DEBUG_PRINT, &args![NO_EMITTER_FORMAT, name]);
    false
}

// Translated from 00c513f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c513f0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// Unnamed in the engine map (cdecl, one word): when `node` passes the emitter check
/// without a root, a new `BSMasterParticleSystem` (`operator new(0xe0)`, `00c5da70`; null
/// when the allocation fails) is linked to it ([`fn_00c51250`]), attaches it (its slot 0xdc
/// with the node and 1) and updates its list (`00c5de70`). Returns the new system, or null
/// when the check fails.
pub fn fn_00c513f0(e: &mut Engine, node: Ptr) -> u32 {
    if !bs_particle_system_manager_check_emitter_recurse(e, node, 0) {
        return 0;
    }
    let block = e.call(OPERATOR_NEW, &args![0xe0u32]).u32();
    let system = if block == 0 {
        0
    } else {
        e.call(MASTER_SYSTEM_CONSTRUCT, &args![block]).u32()
    };
    fn_00c51250(e, node, system);
    e.vcall(system, SLOT_ATTACH_CHILD, &args![node, 1u32]);
    e.call(MASTER_SYSTEM_UPDATE_LIST, &args![system]);
    system
}

// Translated from 00c514c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c514c0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSParticleSystemManager::CreateClone` (Xbox PDB; `RET 4`): a new manager
/// (`operator new(0xb8)`, constructed unless the allocation failed), then this manager's
/// members copied into it ([`fn_00c51550`], which runs on a null clone too). Returns the
/// clone.
pub fn bs_particle_system_manager_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e
        .call(OPERATOR_NEW, &args![BS_PARTICLE_SYSTEM_MANAGER_SIZE])
        .u32();
    let clone = if block == 0 {
        Ptr::NULL
    } else {
        bs_particle_system_manager_bs_particle_system_manager(e, Ptr::new(block))
    };
    fn_00c51550(e, this, clone, cloning);
    clone
}

// Translated from 00c51550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the manager's `CopyMembers` as `CreateClone` calls
/// it, forwarding `this`, the clone and the cloning process to `00a5dc80` (the `NiNode`
/// part).
pub fn fn_00c51550(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(NI_NODE_COPY_MEMBERS, &args![this, clone, cloning]);
}

// Translated from 00c515d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UpdateMasterPsysTaskletData::_vector_deleting_destructor_` (Xbox PDB; `RET 4`). With bit
/// 1 of `flags`: the array whose count word sits just before `this` is destroyed element
/// by element (`eh vector destructor iterator`, 0x20-byte elements, `00c516d0`), the block
/// (from the count word) freed with bit 0, and the block's address returned. Without it:
/// the tasklet-data destructor (`00b00ec0`), `this` freed with bit 0 (memory manager), and
/// `this` returned.
pub fn update_master_psys_tasklet_data_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> u32 {
    let base = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(base.wrapping_sub(4));
        e.call(
            EH_VECTOR_DESTRUCTOR,
            &args![base, 0x20u32, count, TASK_ELEMENT_DESTRUCTOR],
        );
        if flags & 1 != 0 {
            e.call(MEMORY_FREE, &args![MEMORY_MANAGER, base.wrapping_sub(4)]);
        }
        base.wrapping_sub(4)
    } else {
        e.call(TASKLET_DATA_DESTRUCT, &args![base]);
        if flags & 1 != 0 {
            e.call(MEMORY_FREE, &args![MEMORY_MANAGER, base]);
        }
        base
    }
}

// Translated from 00c51640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the task element's constructor (`thiscall`; the element the
/// `eh vector constructor iterator` of `00c50610` builds). Stores the vtables of
/// `BSTaskletData`, then `BSWin32TaskletData` (with its byte `+4` cleared between them),
/// the word `+8` 0, the byte `+0xc` 1, the word `+0x10` 0, and finally the
/// `UpdateMasterPsysTaskletData` vtable. Returns `this`.
pub fn fn_00c51640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, TASKLET_DATA_VTABLE);
    e.mem.set_u8(base + 4, 0);
    e.mem.set_u32(base, WIN32_TASKLET_DATA_VTABLE);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u8(base + 0xc, 1);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base, UPDATE_MASTER_PSYS_TASKLET_DATA_VTABLE);
    this
}

// Translated from 00c516a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `UpdateMasterPsysTaskletData::Process` (Xbox PDB): the child at `+0x18` gets its slot
/// 0xa4 with the time at `+0x1c` and 0 (the call `00c50610` makes directly when it does
/// not queue tasklets).
pub fn update_master_psys_tasklet_data_process(e: &mut Engine, this: Ptr) {
    let time = e.mem.u32(this.addr() + 0x1c);
    let child = e.mem.u32(this.addr() + 0x18);
    e.vcall(child, 0xa4, &args![time, 0u32]);
}

// Translated from 00c516d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the task element's destructor (`thiscall`), the
/// tasklet-data destructor body `00b00ec0`.
pub fn fn_00c516d0(e: &mut Engine, this: Ptr) {
    e.call(TASKLET_DATA_DESTRUCT, &args![this]);
}

// Translated from 00c51700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c51700 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// Unnamed in the engine map (`RET 4`; [`fn_00c509a0`] calls it with a master system): the
/// manager's own slot 0xe4 (`DetachChild` into a local `NiPointer`), the detached child
/// then released. The same code as [`bs_particle_system_manager_detach_child`].
pub fn fn_00c51700(e: &mut Engine, this: Ptr, child: u32) {
    with_node_pointer(e, |e, local| {
        e.vcall(this.addr(), SLOT_DETACH_CHILD, &args![child, local]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_particle_system_manager_get_viewer_strings_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(VIEWER_STRINGS_TARGET_00A5EA70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c5_1570, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == VIEWER_STRINGS_TARGET_00A5EA70)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c516f0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_16f0, &args![0x1000u32]).u32(), 0x0120_3110);
        assert_eq!(e.call(0x00c5_16f0, &args![0u32]).u32(), 0x0120_3110);
    }
}

#[cfg(test)]
mod b000_tests {
    use super::*;

    fn rig(block_fails: bool) -> Engine {
        let mut e = Engine::new();
        e.register(NI_NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            if a[1] == 4 && block_fails {
                0u32.into_ret()
            } else {
                let block = e.mem.alloc(a[1]);
                e.mem.set_u32(block, 0xdddd_dddd);
                block.into_ret()
            }
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| 0x0777_0000u32.into_ret());
        e.register(TASKLET_MANAGER_CREATE_TASK_GROUP, |_, _| Ret::default());
        e
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_fills_the_array_and_creates_the_task_group() {
        let mut e = rig(false);
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_03d0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, NI_NODE_CONSTRUCT), vec![vec![this, 0]]);
        assert_eq!(e.mem.u32(this), BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb0), 10);
        let array = e.mem.u32(this + 0xac);
        assert_eq!(calls(&log, MEMSET), vec![vec![array, 0xffff_ffff, 40]]);
        assert_eq!(e.mem.u32(array + 36), 0xffff_ffff);
        let block = e.mem.u32(this + 0xb4);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(
            calls(&log, TASKLET_MANAGER_CREATE_TASK_GROUP),
            vec![vec![0x0777_0000, block]]
        );
    }

    #[test]
    fn constructor_passes_a_null_group_when_its_allocation_fails() {
        let mut e = rig(true);
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        e.call(0x00c5_03d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(
            calls(&log, TASKLET_MANAGER_CREATE_TASK_GROUP),
            vec![vec![0x0777_0000, 0]]
        );
    }
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    const CHILD_VTABLE: u32 = 0x0300_0000;
    const CHILD_RTTI: u32 = 0x0300_1000;
    const CHILD_UPDATE: u32 = 0x0300_2000;
    const OWN_VTABLE: u32 = 0x0300_3000;
    const OWN_SLOT_FC: u32 = 0x0300_4000;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            MEMORY_FREE,
            MEMCPY,
            NI_NODE_ATTACH_CHILD,
            NI_NODE_DESTRUCTOR,
            MASTER_SYSTEM_ADD_EMITTER,
            DEBUG_PRINT,
            TASKLET_MANAGER_DESTROY_TASK_GROUP,
            SIZED_DELETE,
            CHILD_UPDATE,
            OWN_SLOT_FC,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| 0x7100u32.into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMSET, |e, a| {
            for at in 0..a[2] {
                e.mem.set_u8(a[0] + at, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CHILD_RTTI, |_, _| MASTER_PARTICLE_SYSTEM_RTTI.into_ret());
        let mut slots = vec![0u32; 0x2a];
        slots[2] = CHILD_RTTI;
        slots[0xa4 / 4] = CHILD_UPDATE;
        e.put_vtable(CHILD_VTABLE, &slots);
        let mut own = vec![0u32; 0x40];
        own[0xfc / 4] = OWN_SLOT_FC;
        e.put_vtable(OWN_VTABLE, &own);
        e.call_log = Some(vec![]);
        e
    }

    /// A manager with `children` and an index table of `table`.
    fn manager(e: &mut Engine, children: &[u32], table: &[u32]) -> u32 {
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this, OWN_VTABLE);
        let elements = e.mem.alloc(4 * (children.len() as u32).max(1));
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(elements + 4 * index as u32, *child);
        }
        e.mem.set_u32(this + CHILDREN, elements);
        e.mem.set_u16(this + CHILD_COUNT, children.len() as u16);
        e.mem.set_u16(this + CHILDREN_USED, children.len() as u16);
        let entries = e.mem.alloc(4 * (table.len() as u32).max(1));
        for (index, value) in table.iter().enumerate() {
            e.mem.set_u32(entries + 4 * index as u32, *value);
        }
        e.mem.set_u32(this + INDEX_TABLE, entries);
        e.mem.set_u32(this + INDEX_COUNT, table.len() as u32);
        this
    }

    fn child(e: &mut Engine, index: u32) -> u32 {
        let at = e.mem.alloc(0xd0);
        e.mem.set_u32(at, CHILD_VTABLE);
        e.mem.set_u32(at + 0xc4, index);
        at
    }

    #[test]
    fn finding_and_naming_children() {
        let mut e = engine();
        let (a, b) = (child(&mut e, 0), child(&mut e, 1));
        let this = manager(&mut e, &[a, b], &[1, 0xffff_ffff]);
        assert_eq!(e.call(0x00c5_0ec0, &args![this, b]).u32(), 1);
        assert_eq!(
            e.call(0x00c5_0ec0, &args![this, 0x9999u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.call(0x00c5_0f30, &args![this, 0u32]).u32(), b);
        assert_eq!(e.call(0x00c5_0f30, &args![this, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00c5_0f30, &args![this, 2u32]).u32(), 0);
    }

    #[test]
    fn emitters_go_to_their_system_or_are_reported() {
        let mut e = engine();
        let a = child(&mut e, 0);
        let this = manager(&mut e, &[a], &[0, 0xffff_ffff]);
        let emitter = e.mem.alloc(0x10);
        e.mem.set_u32(emitter + 8, 0x4242);
        e.call(0x00c5_05c0, &args![this, 0u32, emitter]);
        assert_eq!(calls(&e, MASTER_SYSTEM_ADD_EMITTER), vec![vec![a, emitter]]);
        e.call(0x00c5_05c0, &args![this, 1u32, emitter]);
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![MISSING_SYSTEM_FORMAT, 0x4242, 1]]
        );
    }

    #[test]
    fn attaching_fills_the_index_table() {
        let mut e = engine();
        let a = child(&mut e, 1);
        let this = manager(&mut e, &[], &[0xffff_ffff, 0xffff_ffff]);
        e.register(NI_NODE_ATTACH_CHILD, |e, a| {
            // The node puts the child at index 0.
            let elements = e.mem.alloc(4);
            e.mem.set_u32(elements, a[1]);
            e.mem.set_u32(a[0] + CHILDREN, elements);
            e.mem.set_u16(a[0] + CHILDREN_USED, 1);
            Ret::default()
        });
        e.call(0x00c5_09f0, &args![this, a, 0x101u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD), vec![vec![this, a, 1]]);
        let table = e.mem.u32(this + INDEX_TABLE);
        assert_eq!(e.mem.u32(table + 4), 0);
        // Already present: nothing more.
        e.call(0x00c5_09f0, &args![this, a, 0u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD).len(), 1);
        // Past the table: it grows to 2 + 10 entries, -1 filled, old entries kept.
        let far = child(&mut e, 5);
        e.call(0x00c5_09f0, &args![this, far, 0u32]);
        assert_eq!(e.mem.u32(this + INDEX_COUNT), 12);
        let grown = e.mem.u32(this + INDEX_TABLE);
        assert_ne!(grown, table);
        assert_eq!(calls(&e, MEMCPY), vec![vec![grown, table, 8]]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, table]]);
        assert_eq!(e.mem.u32(grown + 4 * 11), 0xffff_ffff);
        // No index, or not a master system: ignored.
        let none = child(&mut e, 0xffff_ffff);
        e.call(0x00c5_09f0, &args![this, none, 0u32]);
        e.register(CHILD_RTTI, |_, _| 0x1234u32.into_ret());
        let other = child(&mut e, 3);
        e.call(0x00c5_09f0, &args![this, other, 0u32]);
        e.call(0x00c5_09f0, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&e, NI_NODE_ATTACH_CHILD).len(), 2);
    }

    #[test]
    fn the_destructors_free_the_table_and_the_group() {
        let mut e = engine();
        let this = manager(&mut e, &[], &[0xffff_ffff]);
        let table = e.mem.u32(this + INDEX_TABLE);
        e.mem.set_u32(this + TASK_GROUP, 0x6600);
        assert_eq!(e.call(0x00c5_1590, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
        assert_eq!(
            (e.mem.u32(this + INDEX_TABLE), e.mem.u32(this + INDEX_COUNT)),
            (0, 0)
        );
        assert_eq!(
            calls(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, table], vec![MEMORY_MANAGER, 0x6600]]
        );
        assert_eq!(
            calls(&e, TASKLET_MANAGER_DESTROY_TASK_GROUP),
            vec![vec![0x7100, 0x6600]]
        );
        assert_eq!(calls(&e, NI_NODE_DESTRUCTOR), vec![vec![this]]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0xb8]]);
        e.mem.set_u32(this + TASK_GROUP, 0);
        e.call(0x00c5_04e0, &args![this]);
        assert_eq!(calls(&e, MEMORY_FREE).len(), 3);
    }

    #[test]
    fn updating_runs_each_child_then_slot_0xfc() {
        let mut e = engine();
        let (a, b) = (child(&mut e, 0), child(&mut e, 1));
        let this = manager(&mut e, &[a, 0, b], &[]);
        e.call(0x00c5_0610, &args![this, 0x3f80_0000u32]);
        assert_eq!(
            calls(&e, CHILD_UPDATE),
            vec![vec![a, 0x3f80_0000, 0], vec![b, 0x3f80_0000, 0]]
        );
        assert_eq!(calls(&e, OWN_SLOT_FC), vec![vec![this]]);
    }
}

#[cfg(test)]
mod n067_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Mock objects share one vtable whose `GetRTTI` (8), `IsNode` (0xc) and
    /// `IsParticlesGeom` (0x30) answer the words at +0x100, +0x104 and +0x108, and whose
    /// slot 4 (`DeleteThis`) is logged.
    const MOCK_RTTI: u32 = 0x0090_1000;
    const MOCK_IS_NODE: u32 = 0x0090_1004;
    const MOCK_IS_PSYS: u32 = 0x0090_1008;
    const MOCK_DELETE: u32 = 0x0090_100c;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(MOCK_RTTI, |e, a| e.mem.u32(a[0] + 0x100).into_ret());
        e.register(MOCK_IS_NODE, |e, a| e.mem.u32(a[0] + 0x104).into_ret());
        e.register(MOCK_IS_PSYS, |e, a| e.mem.u32(a[0] + 0x108).into_ret());
        e.register(MOCK_DELETE, nothing);
        e.register(DEBUG_PRINT, nothing);
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in [
            (4, MOCK_DELETE),
            (8, MOCK_RTTI),
            (0xc, MOCK_IS_NODE),
            (0x30, MOCK_IS_PSYS),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32, rtti: u32) -> u32 {
        let object = e.mem.alloc(0x110);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x100, rtti);
        object
    }

    /// A node (answering itself to `IsNode`) with the given children.
    fn node(e: &mut Engine, vtable: u32, children: &[u32]) -> u32 {
        let node = object(e, vtable, 0x5000);
        e.mem.set_u32(node + 0x104, node);
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in (0u32..).zip(children) {
            e.mem.set_u32(array + 4 * i, *child);
        }
        e.mem.set_u32(node + CHILDREN, array);
        e.mem.set_u16(node + CHILD_COUNT, children.len() as u16);
        node
    }

    /// A world-space particle system whose controller list holds an unrelated controller,
    /// then a multi-target emitter controller pointing at `root` with an emitter whose
    /// RTTI chain reaches `NiPSysVolumeEmitter`. Returns the system and that controller.
    fn particle_system(e: &mut Engine, vtable: u32, root: u32) -> (u32, u32) {
        let system = object(e, vtable, NI_PARTICLE_SYSTEM_RTTI);
        e.mem.set_u32(system + 0x108, 1);
        e.mem.set_u8(system + 0xc4, 1);
        e.mem.set_u32(system + 8, 0x7777);
        let other = object(e, vtable, 0x5004);
        let controller = object(e, vtable, MULTI_TARGET_EMITTER_RTTI);
        e.mem.set_u32(system + 0xc, other);
        e.mem.set_u32(other + 0x30, controller);
        e.mem.set_u32(controller + 0x5c, root);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, VOLUME_EMITTER_RTTI);
        let emitter = object(e, vtable, record);
        e.mem.set_u32(controller + 0x3c, emitter);
        (system, controller)
    }

    #[test]
    fn emitter_check_walks_nodes_and_controllers() {
        let (mut e, vtable) = engine();
        let root = 0x4444u32;
        let (system, controller) = particle_system(&mut e, vtable, root);
        let tree = node(&mut e, vtable, &[0, system]);
        assert!(e.call(0x00c5_0f90, &args![tree, root]).bool());
        assert!(e.call(0x00c5_0f90, &args![tree, 0u32]).bool());
        assert!(calls(&e, DEBUG_PRINT).is_empty());
        assert!(!e.call(0x00c5_0f90, &args![tree, 0x4445u32]).bool());
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![WRONG_ROOT_FORMAT, 0x7777]]
        );
        // The emitter's RTTI chain ends without the volume emitter.
        let emitter = e.mem.u32(controller + 0x3c);
        let record = e.mem.u32(emitter + 0x100);
        e.mem.set_u32(record + 4, 0);
        assert!(!e.call(0x00c5_0f90, &args![system, root]).bool());
        e.mem.set_u32(controller + 0x3c, 0);
        assert!(!e.call(0x00c5_0f90, &args![system, root]).bool());
        assert_eq!(calls(&e, DEBUG_PRINT).len(), 1);
        e.mem.set_u8(system + 0xc4, 0);
        assert!(!e.call(0x00c5_0f90, &args![system, root]).bool());
        assert_eq!(
            calls(&e, DEBUG_PRINT)[1],
            vec![NOT_WORLD_SPACE_FORMAT, 0x7777]
        );
        e.mem.set_u8(system + 0xc4, 1);
        e.mem.set_u32(controller + 0x100, 0x5008);
        assert!(!e.call(0x00c5_0f90, &args![system, root]).bool());
        assert_eq!(calls(&e, DEBUG_PRINT)[2], vec![NO_EMITTER_FORMAT, 0x7777]);
        assert!(!e.call(0x00c5_0f90, &args![0u32, root]).bool());
        // A master system checks itself as the root.
        let master = node(&mut e, vtable, &[]);
        e.mem.set_u32(master + 0x100, MASTER_PARTICLE_SYSTEM_RTTI);
        assert!(e.call(0x00c5_11f0, &args![master]).bool());
        assert!(!e.call(0x00c5_11f0, &args![tree]).bool());
        assert!(!e.call(0x00c5_11f0, &args![0u32]).bool());
    }

    #[test]
    fn linking_sets_world_space_and_the_root() {
        let (mut e, vtable) = engine();
        let (system, controller) = particle_system(&mut e, vtable, 0);
        e.mem.set_u8(system + 0xc4, 0);
        let tree = node(&mut e, vtable, &[system]);
        assert!(e.call(0x00c5_1250, &args![tree, 0x9000u32]).bool());
        assert_eq!(e.mem.u8(system + 0xc4), 1);
        assert_eq!(e.mem.u32(controller + 0x5c), 0x9000);
        e.mem.set_u32(controller + 0x100, 0x5008);
        assert!(!e.call(0x00c5_1250, &args![tree, 0x9000u32]).bool());
        assert_eq!(
            calls(&e, DEBUG_PRINT),
            vec![vec![NO_EMITTER_FORMAT, 0x7777]]
        );
        assert!(!e.call(0x00c5_1250, &args![0u32, 0x9000u32]).bool());
    }

    #[test]
    fn making_a_master_system() {
        let (mut e, vtable) = engine();
        let attach = 0x0090_1010;
        e.register(attach, nothing);
        e.register(MASTER_SYSTEM_UPDATE_LIST, nothing);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let system_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(system_vtable + SLOT_ATTACH_CHILD, attach);
        e.register_double(MASTER_SYSTEM_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], system_vtable);
            a[0].into_ret()
        });
        let tree = node(&mut e, vtable, &[]);
        let system = e.call(0x00c5_13f0, &args![tree]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0xe0]]);
        assert_eq!(calls(&e, attach), vec![vec![system, tree, 1]]);
        assert_eq!(calls(&e, MASTER_SYSTEM_UPDATE_LIST), vec![vec![system]]);
        assert_eq!(e.call(0x00c5_13f0, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, OPERATOR_NEW).len(), 1);
    }

    /// A manager whose slots 0xe4, 0xec and 0xf4 store `released` into their out pointer
    /// (the last word).
    fn manager_with_slots(e: &mut Engine, released: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in [SLOT_DETACH_CHILD, SLOT_DETACH_CHILD_AT, SLOT_SET_AT] {
            let target = 0x0091_0000 + slot;
            e.register_double(target, move |e, a| {
                e.mem.set_u32(a[a.len() - 1], released);
                Ret::default()
            });
            e.mem.set_u32(vtable + slot, target);
        }
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn detaching_releases_the_child() {
        let (mut e, vtable) = engine();
        let child = object(&mut e, vtable, 0x5000);
        let this = manager_with_slots(&mut e, child);
        e.mem.set_u32(child + 4, 2);
        e.call(0x00c5_0b80, &args![this, 0x1234u32]);
        let detach = calls(&e, 0x0091_0000 + SLOT_DETACH_CHILD);
        assert_eq!((detach[0][0], detach[0][1]), (this, 0x1234));
        assert_eq!(e.mem.u32(child + 4), 1);
        assert!(calls(&e, MOCK_DELETE).is_empty());
        e.call(0x00c5_1700, &args![this, 0x1234u32]);
        assert_eq!(calls(&e, MOCK_DELETE), vec![vec![child]]);
        e.mem.set_u32(child + 4, 1);
        e.call(0x00c5_0d20, &args![this, 3u32]);
        assert_eq!(calls(&e, 0x0091_0000 + SLOT_DETACH_CHILD_AT)[0][1], 3);
        assert_eq!(calls(&e, MOCK_DELETE).len(), 2);
        e.mem.set_u32(child + 4, 1);
        e.call(0x00c5_0df0, &args![this, 3u32, 0x55u32]);
        let set = calls(&e, 0x0091_0000 + SLOT_SET_AT);
        assert_eq!((set[0][1], set[0][2]), (3, 0x55));
        assert_eq!(calls(&e, MOCK_DELETE).len(), 3);
        assert!(calls(&e, INTERLOCKED_INCREMENT).is_empty());
        // Nothing detached: nothing released.
        let empty = manager_with_slots(&mut e, 0);
        e.call(0x00c5_0b80, &args![empty, 0x1234u32]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 4);
    }

    #[test]
    fn detaching_into_a_pointer_clears_the_table_entry() {
        let (mut e, vtable) = engine();
        let child = object(&mut e, vtable, 0x5000);
        e.mem.set_u32(child + 4, 1);
        e.mem.set_u32(child + 0xc4, 2);
        let this = e.mem.alloc(0xb8);
        let table = e.mem.alloc(16);
        e.mem.set_u32(this + INDEX_TABLE, table);
        e.register_double(NI_NODE_DETACH_CHILD, move |e, a| {
            e.mem.set_u32(a[2], child);
            Ret::default()
        });
        e.register(NI_POINTER_ASSIGN, nothing);
        e.call(0x00c5_0c30, &args![this, child, 0x6000u32]);
        assert_eq!(e.mem.u32(table + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(table + 4), 0);
        let detach = calls(&e, NI_NODE_DETACH_CHILD);
        let assign = calls(&e, NI_POINTER_ASSIGN);
        assert_eq!(assign, vec![vec![0x6000, detach[0][2]]]);
        assert_eq!(calls(&e, MOCK_DELETE), vec![vec![child]]);
        e.register(NI_NODE_DETACH_CHILD, nothing);
        e.call(0x00c5_0c30, &args![this, child, 0x6000u32]);
        assert_eq!(calls(&e, MOCK_DELETE).len(), 1);
        e.register(NI_POINTER_ASSIGN_RAW, |_, a| a[0].into_ret());
        assert_eq!(
            e.call(0x00c5_0dd0, &args![this, 1u32, 0x6100u32]).u32(),
            0x6100
        );
        assert_eq!(
            e.call(0x00c5_0ea0, &args![this, 1u32, 2u32, 0x6200u32])
                .u32(),
            0x6200
        );
        assert_eq!(
            calls(&e, NI_POINTER_ASSIGN_RAW),
            vec![vec![0x6100, 0], vec![0x6200, 0]]
        );
    }

    #[test]
    fn children_and_master_removal() {
        let (mut e, _) = engine();
        e.register(MASTER_SYSTEM_00C5DEA0, nothing);
        let (a, b) = (0x7100u32, 0x7200u32);
        let this = manager_with_slots(&mut e, 0);
        let children = e.mem.alloc(12);
        for (i, child) in (0u32..).zip([a, 0, b]) {
            e.mem.set_u32(children + 4 * i, child);
        }
        e.mem.set_u32(this + CHILDREN, children);
        e.mem.set_u16(this + CHILD_COUNT, 3);
        e.call(0x00c5_0930, &args![this]);
        assert_eq!(calls(&e, MASTER_SYSTEM_00C5DEA0), vec![vec![a], vec![b]]);
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, 2);
        e.mem.set_u32(table + 4, 0xffff_ffff);
        e.mem.set_u32(this + INDEX_TABLE, table);
        e.mem.set_u32(this + INDEX_COUNT, 2);
        e.call(0x00c5_09a0, &args![this, 0u32]);
        assert_eq!(calls(&e, 0x0091_0000 + SLOT_DETACH_CHILD)[0][1], b);
        e.call(0x00c5_09a0, &args![this, 1u32]);
        assert_eq!(calls(&e, DEBUG_PRINT), vec![vec![NOT_LOADED_FORMAT, 1]]);
    }

    #[test]
    fn clone_and_copy_members() {
        let (mut e, _) = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for address in [
            NI_NODE_CONSTRUCT,
            MEMSET,
            TASKLET_MANAGER_CREATE_TASK_GROUP,
            NI_NODE_COPY_MEMBERS,
        ] {
            e.register(address, nothing);
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(TASKLET_MANAGER_INSTANCE, |_, _| 0x7000u32.into_ret());
        let clone = e.call(0x00c5_14c0, &args![0x100u32, 0x200u32]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0xb8]]);
        assert_eq!(e.mem.u32(clone), BS_PARTICLE_SYSTEM_MANAGER_VTABLE);
        assert_eq!(
            calls(&e, NI_NODE_COPY_MEMBERS),
            vec![vec![0x100, clone, 0x200]]
        );
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c5_14c0, &args![0x100u32, 0x200u32]).u32(), 0);
        assert_eq!(calls(&e, NI_NODE_COPY_MEMBERS)[1], vec![0x100, 0, 0x200]);
    }

    #[test]
    fn tasklet_data_construct_process_and_destroy() {
        let (mut e, _) = engine();
        e.register(TASKLET_DATA_DESTRUCT, nothing);
        e.register(EH_VECTOR_DESTRUCTOR, nothing);
        e.register(MEMORY_FREE, nothing);
        let data = e.mem.alloc(0x20);
        e.mem.write(data, &[0xaa; 0x20]);
        assert_eq!(e.call(0x00c5_1640, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), UPDATE_MASTER_PSYS_TASKLET_DATA_VTABLE);
        assert_eq!(e.mem.u8(data + 4), 0);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u8(data + 0xc), 1);
        assert_eq!(e.mem.u32(data + 0x10), 0);
        assert_eq!(e.mem.u8(data + 0x14), 0xaa);
        let child_vtable = e.mem.alloc(0xa8);
        e.mem.set_u32(child_vtable + 0xa4, 0x0090_2000);
        e.register(0x0090_2000, nothing);
        let child = e.mem.alloc(4);
        e.mem.set_u32(child, child_vtable);
        e.mem.set_u32(data + 0x18, child);
        e.mem.set_u32(data + 0x1c, 0x3f00_0000);
        e.call(0x00c5_16a0, &args![data]);
        assert_eq!(calls(&e, 0x0090_2000), vec![vec![child, 0x3f00_0000, 0]]);
        e.call(0x00c5_16d0, &args![data]);
        assert_eq!(calls(&e, TASKLET_DATA_DESTRUCT), vec![vec![data]]);
        assert_eq!(e.call(0x00c5_15d0, &args![data, 1u32]).u32(), data);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, data]]);
        assert_eq!(e.call(0x00c5_15d0, &args![data, 0u32]).u32(), data);
        assert_eq!(calls(&e, MEMORY_FREE).len(), 1);
        let block = e.mem.alloc(0x44);
        e.mem.set_u32(block, 2);
        assert_eq!(e.call(0x00c5_15d0, &args![block + 4, 3u32]).u32(), block);
        assert_eq!(
            calls(&e, EH_VECTOR_DESTRUCTOR),
            vec![vec![block + 4, 0x20, 2, TASK_ELEMENT_DESTRUCTOR]]
        );
        assert_eq!(calls(&e, MEMORY_FREE)[1], vec![MEMORY_MANAGER, block]);
        assert_eq!(e.call(0x00c5_15d0, &args![block + 4, 2u32]).u32(), block);
        assert_eq!(calls(&e, MEMORY_FREE).len(), 2);
    }
}
