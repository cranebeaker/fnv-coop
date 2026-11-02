//! `NiParticle/nimeshpsysdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c30bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c309d0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x98 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_mesh_p_sys_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_09d0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x98u32]);
    }
    this
}

// Translated from 00c30a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27c0.
pub fn ni_mesh_p_sys_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27c0
}

layout! {
    /// `NiMeshPSysData` (Xbox PDB, 0x98 bytes; the PC code uses the same offsets): the fields
    /// its own functions use.
    pub struct NiMeshPSysData: 0x98 {
        /// `m_spParticleMeshes` (Xbox PDB): `NiPointer<NiNode>`.
        0x7c m_spParticleMeshes: u32,
        /// `m_bFillPoolsOnLoad` (Xbox PDB).
        0x80 m_bFillPoolsOnLoad: u8,
        /// `m_uiDefaultPoolSize` (Xbox PDB).
        0x84 m_uiDefaultPoolSize: u32,
    }
}

/// `m_kPools` (Xbox PDB, an `NiTPrimitiveArray` of `NiTObjectArray<NiPointer<NiAVObject>> *`
/// at +0x88): its vtable, `m_pBase` (+0x8c), `m_usMaxSize` (+0x90), `m_usSize` (+0x92),
/// `m_usESize` (+0x94) and `m_usGrowBy` (+0x96).
const POOLS: u32 = 0x88;
const POOLS_BASE: u32 = 0x8c;
const POOLS_MAX_SIZE: u32 = 0x90;
const POOLS_SIZE: u32 = 0x92;
const POOLS_USED: u32 = 0x94;
const POOLS_GROW_BY: u32 = 0x96;
/// `NiMeshPSysData`'s vtable and size; the vtables of the pool arrays
/// (`NiTPrimitiveArray<NiTObjectArray<NiPointer<NiAVObject>> *>`, `NiTArray<...>` set by the
/// destructor) and of a pool (`NiTObjectArray<NiPointer<NiAVObject>>`).
const NI_MESH_P_SYS_DATA_VTABLE: u32 = 0x010c_07ec;
const NI_MESH_P_SYS_DATA_SIZE: u32 = 0x98;
const POOLS_VTABLE: u32 = 0x010c_08a0;
const POOLS_ARRAY_VTABLE: u32 = 0x010c_07e0;
const POOL_VTABLE: u32 = 0x0101_fbd4;
/// `NiPSysData`'s functions it builds on (`thiscall`): the constructor (`00c24c90`), the
/// destructor (`00c24ef0`), `RemoveParticle` (`00c24cc0`, one word), `ResolveAddedParticles`
/// (`00c24d20`), `IsEqual` (`00c24de0`), `CopyMembers` (`00c24f20`, two words), `LoadBinary`
/// (`00c25340`), `SaveBinary` (`00c25560`), `GetViewerStrings` (`00c255b0`),
/// `RegisterStreamables` (`00a75f00`) and `LinkObject` (`00a7b7d0`) (one word each), and
/// `NiObject::ProcessClone` (`00a5d430`, one word).
const PSYS_DATA_CONSTRUCT_00C24C90: u32 = 0x00c2_4c90;
const PSYS_DATA_DESTRUCT_00C24EF0: u32 = 0x00c2_4ef0;
const PSYS_DATA_REMOVE_PARTICLE_00C24CC0: u32 = 0x00c2_4cc0;
const PSYS_DATA_RESOLVE_ADDED_PARTICLES_00C24D20: u32 = 0x00c2_4d20;
const PSYS_DATA_IS_EQUAL_00C24DE0: u32 = 0x00c2_4de0;
const PSYS_DATA_COPY_MEMBERS_00C24F20: u32 = 0x00c2_4f20;
const PSYS_DATA_LOAD_BINARY_00C25340: u32 = 0x00c2_5340;
const PSYS_DATA_SAVE_BINARY_00C25560: u32 = 0x00c2_5560;
const PSYS_DATA_GET_VIEWER_STRINGS_00C255B0: u32 = 0x00c2_55b0;
const PSYS_DATA_REGISTER_STREAMABLES_00A75F00: u32 = 0x00a7_5f00;
const PSYS_DATA_LINK_OBJECT_00A7B7D0: u32 = 0x00a7_b7d0;
const OBJECT_PROCESS_CLONE_00A5D430: u32 = 0x00a5_d430;
/// `NiStream::GetObjectFromLinkID` (`00a63430`) and the link step `00a645b0` that ends the
/// load (both `thiscall` on the stream, no stack words).
const GET_OBJECT_FROM_LINK_ID_00A63430: u32 = 0x00a6_3430;
const STREAM_LINK_00A645B0: u32 = 0x00a6_45b0;
/// The pool array's set-size (`00a90180`, one word) and set-at (`00ba3aa0`, two words: the
/// index and the address of the value); a pool's resize (`0060bef0`, one word) and add
/// (`004afc50`, one word: the address of an `NiPointer`); the allocator at `011f6238` with its
/// allocation (`00aa3e40`, one word: the size) and free (`00aa4060`, one word: the block).
const POOLS_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const POOLS_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const POOL_RESIZE_0060BEF0: u32 = 0x0060_bef0;
const POOL_ADD_004AFC50: u32 = 0x004a_fc50;
const POOL_ALLOCATOR_011F6238: u32 = 0x011f_6238;
const POOL_ALLOCATE_00AA3E40: u32 = 0x00aa_3e40;
const POOL_FREE_00AA4060: u32 = 0x00aa_4060;
/// The Ni allocation (`00aa13e0`, `cdecl`: size), the plain free (`00aa10f0`, `cdecl`) and the
/// interlocked imports (`InterlockedDecrement`, `InterlockedIncrement`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The stream's file version (+0xd8), its input and output binary streams (+0x24c, +0x250)
/// with their read (word +8) and write (word +0xc) functions (`cdecl`: the object, the
/// buffer, the count, the address of the component size and the component count); the bytes
/// moved are added to the object's word +4. Versions up to `0a010000` have no pool data.
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT_BINARY_STREAM: u32 = 0x24c;
const STREAM_OUTPUT_BINARY_STREAM: u32 = 0x250;
const POOL_DATA_VERSION: u32 = 0x0a01_0000;
/// `NiMeshPSysData`'s RTTI record (its first word the class name), `NiGetViewerString`
/// (`00a7d330`, `cdecl`) and the string array's grow (`0096ad30`) and set-at (`0096ae90`).
const NI_MESH_P_SYS_DATA_RTTI: u32 = 0x0120_27c0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// Releases a reference-counted object (`InterlockedDecrement` on +4; virtual slot +4 at 0).
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `NiPointer` assignment of `object` to the word at `slot` (the old object released, the new
/// one held), skipped when they are the same.
fn assign_pointer(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

/// The pools' last entry removed while the count is above `keep`: the count goes down, the
/// slot is cleared and a set pool's used count goes down and the pool is deleted (its
/// virtual slot 0 with 1).
fn drop_pools_down_to(e: &mut Engine, base: u32, keep: u32) {
    loop {
        let size = e.mem.u16(base + POOLS_SIZE);
        if size != 0 {
            let index = size - 1;
            e.mem.set_u16(base + POOLS_SIZE, index);
            let slot = e
                .mem
                .u32(base + POOLS_BASE)
                .wrapping_add(u32::from(index) * 4);
            let pool = e.mem.u32(slot);
            e.mem.set_u32(slot, 0);
            if pool != 0 {
                let used = e.mem.u16(base + POOLS_USED).wrapping_add(0xffff);
                e.mem.set_u16(base + POOLS_USED, used);
                e.vcall(pool, 0, &args![1u32]);
            }
        }
        if u32::from(e.mem.u16(base + POOLS_SIZE)) <= keep {
            break;
        }
    }
}

/// A new pool (16 bytes from the allocator at `011f6238`; 0 when that fails): the
/// `NiTObjectArray` vtable, no entries, growth 1.
fn new_pool(e: &mut Engine) -> u32 {
    let pool = e
        .call(
            POOL_ALLOCATE_00AA3E40,
            &args![POOL_ALLOCATOR_011F6238, 0x10u32],
        )
        .u32();
    if pool == 0 {
        return 0;
    }
    e.mem.set_u16(pool + 0xe, 1);
    e.mem.set_u16(pool + 8, 0);
    e.mem.set_u16(pool + 0xa, 0);
    e.mem.set_u16(pool + 0xc, 0);
    e.mem.set_u32(pool + 4, 0);
    e.mem.set_u32(pool, POOL_VTABLE);
    pool
}

/// Makes a pool of `size` entries (`0060bef0`) and stores it at `index` of the pool array
/// (`00ba3aa0` with the address of the stack word `slot` holding it).
fn add_pool(e: &mut Engine, base: u32, index: u32, size: u32, slot: u32) {
    let pool = new_pool(e);
    e.mem.set_u32(slot, pool);
    e.call(POOL_RESIZE_0060BEF0, &args![pool, size]);
    e.call(POOLS_SET_AT_00BA3AA0, &args![base + POOLS, index, slot]);
}

/// Moves `count` bytes between `field` and the stream's binary stream (`which`: the stream
/// offset and the function's word), the component size through `size_slot`.
fn stream_bytes(
    e: &mut Engine,
    stream: u32,
    which: (u32, u32),
    field: u32,
    count: u32,
    size_slot: u32,
) {
    let binary = e.mem.u32(stream + which.0);
    let function = e.mem.u32(binary + which.1);
    e.mem.set_u32(size_slot, count);
    let moved = e
        .call(function, &args![binary, field, count, size_slot, 1u32])
        .u32();
    let total = e.mem.u32(binary + 4).wrapping_add(moved);
    e.mem.set_u32(binary + 4, total);
}

// Translated from 00c308d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::SetActiveVertexCount` (vtable slot +0x8c, `thiscall`, one word, its low 16
/// bits the count): each particle from the count up to `m_usActive` is detached from the mesh
/// node (its virtual slot +0xf0 with the index); then `m_usActive` takes the count, at most
/// `m_usVertices`.
pub fn ni_mesh_p_sys_data_set_active_vertex_count(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    count: u32,
) {
    let base = this.addr();
    let count = count as u16;
    let mut index = count;
    if index < e.mem.u16(base + 0x50) {
        loop {
            let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
            e.vcall(meshes, 0xf0, &args![u32::from(index)]);
            index = index.wrapping_add(1);
            if index >= e.mem.u16(base + 0x50) {
                break;
            }
        }
    }
    let vertices = e.mem.u16(base + 8);
    let active = if count > vertices { vertices } else { count };
    e.mem.set_u16(base + 0x50, active);
}

// Translated from 00c30920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::ProcessClone` (vtable slot +0x68, `thiscall`, one word: the cloning
/// process): `NiObject::ProcessClone`, then the mesh node's virtual slot +0x68 with it.
pub fn ni_mesh_p_sys_data_process_clone(e: &mut Engine, this: Ptr<NiMeshPSysData>, cloning: u32) {
    e.call(OBJECT_PROCESS_CLONE_00A5D430, &args![this, cloning]);
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    e.vcall(meshes, 0x68, &args![cloning]);
}

// Translated from 00c30940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::RegisterStreamables` (vtable slot +0x54, `thiscall`, one word: the
/// stream): `NiPSysData::RegisterStreamables` first (answers 0 when that fails), then the
/// mesh node's slot +0x54. Answers 1.
pub fn ni_mesh_p_sys_data_register_streamables(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    stream: u32,
) -> bool {
    if !e
        .call(
            PSYS_DATA_REGISTER_STREAMABLES_00A75F00,
            &args![this, stream],
        )
        .bool()
    {
        return false;
    }
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    e.vcall(meshes, 0x54, &args![stream]);
    true
}

// Translated from 00c30970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::IsEqual` (vtable slot +0x5c, `thiscall`, one word: the other data):
/// `NiPSysData::IsEqual`, then the mesh node's slot +0x5c with the other's mesh node.
pub fn ni_mesh_p_sys_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    other: Ptr<NiMeshPSysData>,
) -> bool {
    if !e
        .call(PSYS_DATA_IS_EQUAL_00C24DE0, &args![this, other])
        .bool()
    {
        return false;
    }
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    let theirs = e.get(other, NiMeshPSysData::m_spParticleMeshes);
    e.vcall(meshes, 0x5c, &args![theirs]).bool()
}

// Translated from 00c309a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the pool array classes (slot 0 of both
/// `NiTPrimitiveArray<NiTObjectArray<NiPointer<NiAVObject>> *>` and its `NiTArray` base;
/// `thiscall`, one word: the flags): the `NiTArray` vtable, the buffer (`+4`) freed
/// (`00aa10f0`), and with bit 0 the object freed through the allocator at `011f6238`
/// (`00aa4060`). Answers `this`.
pub fn fn_00c309a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), POOLS_ARRAY_VTABLE);
    e.call(FREE_00AA10F0, &args![buffer]);
    if flags & 1 != 0 {
        e.call(POOL_FREE_00AA4060, &args![POOL_ALLOCATOR_011F6238, this]);
    }
    this
}

// Translated from 00c309d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData`'s destructor (`thiscall`): the vtable, every pool removed and deleted
/// (from the last), the pool array's `NiTArray` vtable and its buffer freed, the mesh node
/// released, then the `NiPSysData` destructor (`00c24ef0`, a jump).
pub fn ni_mesh_p_sys_data_destructor(e: &mut Engine, this: Ptr<NiMeshPSysData>) {
    let base = this.addr();
    e.mem.set_u32(base, NI_MESH_P_SYS_DATA_VTABLE);
    if e.mem.u16(base + POOLS_SIZE) > 0 {
        drop_pools_down_to(e, base, 0);
    }
    let buffer = e.mem.u32(base + POOLS_BASE);
    e.mem.set_u32(base + POOLS, POOLS_ARRAY_VTABLE);
    e.call(FREE_00AA10F0, &args![buffer]);
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    if meshes != 0 {
        release(e, meshes);
    }
    e.call(PSYS_DATA_DESTRUCT_00C24EF0, &args![this]);
}

// Translated from 00c30a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::ResolveAddedParticles` (vtable slot +0xac, `thiscall`): bit 0 of the
/// flags (`+0x30`) of each mesh node child (the node's array at `+0xa0`) from
/// `m_usAddedParticlesBase` (+0x7a) for `m_usNumAddedParticles` (+0x78) is cleared (null
/// children skipped; the range re-read each step, the index kept to 16 bits); then
/// `NiPSysData::ResolveAddedParticles` (a jump).
pub fn ni_mesh_p_sys_data_resolve_added_particles(e: &mut Engine, this: Ptr<NiMeshPSysData>) {
    let base = this.addr();
    let first = u32::from(e.mem.u16(base + 0x7a));
    let mut index = first;
    let mut end = first + u32::from(e.mem.u16(base + 0x78));
    while (index as i32) < end as i32 {
        let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
        let children = e.mem.u32(meshes + 0xa0);
        let child = e.mem.u32(children.wrapping_add(index * 4));
        if child != 0 {
            let flags = e.mem.u32(child + 0x30) & 0xffff_fffe;
            e.mem.set_u32(child + 0x30, flags);
        }
        index = (index + 1) & 0xffff;
        end = u32::from(e.mem.u16(base + 0x7a)) + u32::from(e.mem.u16(base + 0x78));
    }
    e.call(PSYS_DATA_RESOLVE_ADDED_PARTICLES_00C24D20, &args![this]);
}

// Translated from 00c30ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::CopyMembers` (`thiscall`, two words: the clone and the cloning process):
/// `NiPSysData::CopyMembers`, the clone's mesh node set to a clone of this one's (its virtual
/// slot +0x48) and `m_bFillPoolsOnLoad` copied.
pub fn ni_mesh_p_sys_data_copy_members(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    clone: Ptr<NiMeshPSysData>,
    cloning: u32,
) {
    e.call(
        PSYS_DATA_COPY_MEMBERS_00C24F20,
        &args![this, clone, cloning],
    );
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    let copy = e.vcall(meshes, 0x48, &args![cloning]).u32();
    assign_pointer(e, clone.addr() + 0x7c, copy);
    let fill = e.get(this, NiMeshPSysData::m_bFillPoolsOnLoad);
    e.set(clone, NiMeshPSysData::m_bFillPoolsOnLoad, fill);
}

// Translated from 00c30b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::LinkObject` (vtable slot +0x50, `thiscall`, one word: the stream):
/// `NiPSysData::LinkObject`, then the mesh node set to the stream's next linked object
/// (`NiStream::GetObjectFromLinkID`).
pub fn ni_mesh_p_sys_data_link_object(e: &mut Engine, this: Ptr<NiMeshPSysData>, stream: u32) {
    e.call(PSYS_DATA_LINK_OBJECT_00A7B7D0, &args![this, stream]);
    let object = e
        .call(GET_OBJECT_FROM_LINK_ID_00A63430, &args![stream])
        .u32();
    assign_pointer(e, this.addr() + 0x7c, object);
}

// Translated from 00c30c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData`'s constructor (`thiscall`): the `NiPSysData` constructor, the vtable, no
/// mesh node and an empty pool array (growth 1). Answers `this`.
pub fn ni_mesh_p_sys_data_constructor(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
) -> Ptr<NiMeshPSysData> {
    let base = this.addr();
    e.call(PSYS_DATA_CONSTRUCT_00C24C90, &args![this]);
    e.mem.set_u32(base, NI_MESH_P_SYS_DATA_VTABLE);
    e.mem.set_u32(base + 0x7c, 0);
    e.mem.set_u16(base + POOLS_GROW_BY, 1);
    e.mem.set_u32(base + POOLS_BASE, 0);
    e.mem.set_u16(base + POOLS_MAX_SIZE, 0);
    e.mem.set_u16(base + POOLS_SIZE, 0);
    e.mem.set_u16(base + POOLS_USED, 0);
    e.mem.set_u32(base + POOLS, POOLS_VTABLE);
    this
}

// Translated from 00c30c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::CreateClone` (vtable slot +0x48, `thiscall`, one word: the cloning
/// process): 0x98 bytes from `00aa13e0` constructed ([`ni_mesh_p_sys_data_constructor`]),
/// then [`ni_mesh_p_sys_data_copy_members`] into it (0 when the allocation failed, as the exe
/// does). Answers the clone.
pub fn ni_mesh_p_sys_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    cloning: u32,
) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![NI_MESH_P_SYS_DATA_SIZE]).u32();
    let clone = if block != 0 {
        ni_mesh_p_sys_data_constructor(e, Ptr::new(block)).addr()
    } else {
        0
    };
    ni_mesh_p_sys_data_copy_members(e, this, Ptr::new(clone), cloning);
    clone
}

// Translated from 00c30ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The streaming factory of `NiMeshPSysData` (no function in the Ghidra project; read with the
/// byte disassembler; no arguments): 0x98 bytes from `00aa13e0` constructed (a jump to
/// [`ni_mesh_p_sys_data_constructor`]), or 0 when the allocation fails.
pub fn fn_00c30ca0(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![NI_MESH_P_SYS_DATA_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    ni_mesh_p_sys_data_constructor(e, Ptr::new(block)).addr()
}

// Translated from 00c30cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::SaveBinary` (vtable slot +0x58, `thiscall`, one word: the stream):
/// `NiPSysData::SaveBinary`, then through the stream's output binary stream
/// `m_uiDefaultPoolSize` (4 bytes), `m_bFillPoolsOnLoad` (1 byte, from the low byte of the
/// argument's stack word), the pool count (4 bytes) and each pool's size (its 16-bit word +8,
/// or 0 for an empty slot; 4 bytes), then the mesh node's link ID (the stream's virtual slot
/// +0x30).
pub fn ni_mesh_p_sys_data_save_binary(e: &mut Engine, this: Ptr<NiMeshPSysData>, stream: Ptr) {
    let base = this.addr();
    e.call(PSYS_DATA_SAVE_BINARY_00C25560, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        // The locals at ESP+0x14 and ESP+0x18 and the argument's stack word.
        let (first, second, argument) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        let output = (STREAM_OUTPUT_BINARY_STREAM, 0xc);
        let stream = stream.addr();
        e.mem.set_u32(argument, stream);
        stream_bytes(e, stream, output, base + 0x84, 4, argument);
        let fill = e.get(this, NiMeshPSysData::m_bFillPoolsOnLoad);
        e.mem.set_u8(argument, fill);
        stream_bytes(e, stream, output, argument, 1, first);
        let count = u32::from(e.mem.u16(base + POOLS_SIZE));
        e.mem.set_u32(first, count);
        stream_bytes(e, stream, output, first, 4, argument);
        let mut index = 0u32;
        if e.mem.u16(base + POOLS_SIZE) != 0 {
            loop {
                let pool = e
                    .mem
                    .u32(e.mem.u32(base + POOLS_BASE).wrapping_add(index * 4));
                let buffer = if pool != 0 {
                    e.mem.set_u32(first, u32::from(e.mem.u16(pool + 8)));
                    first
                } else {
                    e.mem.set_u32(second, 0);
                    second
                };
                stream_bytes(e, stream, output, buffer, 4, argument);
                index += 1;
                if index >= u32::from(e.mem.u16(base + POOLS_SIZE)) {
                    break;
                }
            }
        }
    });
    let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
    e.vcall(stream.addr(), 0x30, &args![meshes]);
}

// Translated from 00c30df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: a pool index and an `NiPointer` taken by
/// value, its object word): gives the object back to its pool — added to the pool
/// (`004afc50` with the argument's address) when the index is below the pool count, the pool
/// is set and its size (+0xa) below its capacity (+8); in every case the argument's reference
/// is released at the end.
pub fn fn_00c30df0(e: &mut Engine, this: Ptr<NiMeshPSysData>, pool_index: u32, object: u32) {
    let base = this.addr();
    e.with_stack(4, |e, argument| {
        let argument = argument.addr();
        e.mem.set_u32(argument, object);
        if pool_index < u32::from(e.mem.u16(base + POOLS_SIZE)) {
            let slot = e
                .mem
                .u32(base + POOLS_BASE)
                .wrapping_add(pool_index.wrapping_mul(4));
            if e.mem.u32(slot) != 0 {
                let pool = e.mem.u32(slot);
                if e.mem.u16(pool + 0xa) < e.mem.u16(pool + 8) {
                    let pool = e.mem.u32(slot);
                    e.call(POOL_ADD_004AFC50, &args![pool, argument]);
                }
            }
        }
        let object = e.mem.u32(argument);
        if object != 0 {
            release(e, object);
        }
    });
}

// Translated from 00c30e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::RemoveParticle` (vtable slot +0xa4, `thiscall`, one word: the particle
/// index, 16 bits): notes the particle's generation (`m_pkParticleInfo` +0x70, its 16-bit
/// word +0x18), runs `NiPSysData::RemoveParticle`, then detaches the particle's mesh from the
/// mesh node (slot +0xec into the argument's stack word). Unless it was the last particle, the
/// last particle's mesh is detached first (into a local `NiPointer`) and set at the removed
/// particle's index (slot +0xf8), and the local released. The removed mesh goes back to the
/// pool of its generation ([`fn_00c30df0`] with a held copy), and the argument's reference is
/// released.
pub fn ni_mesh_p_sys_data_remove_particle(e: &mut Engine, this: Ptr<NiMeshPSysData>, index: u32) {
    let base = this.addr();
    let last = u32::from(e.mem.u16(base + 0x50).wrapping_sub(1));
    let particle = index & 0xffff;
    let info = e.mem.u32(base + 0x70);
    let generation = e.mem.u16(info.wrapping_add(particle * 0x1c) + 0x18);
    e.call(PSYS_DATA_REMOVE_PARTICLE_00C24CC0, &args![this, index]);
    e.with_stack(8, |e, frame| {
        // The local `NiPointer` (ESP+0x10) and the argument's stack word (ESP+0x1c).
        let (moved, removed) = (frame.addr(), frame.addr() + 4);
        let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
        e.mem.set_u32(removed, 0);
        if particle == last {
            e.vcall(meshes, 0xec, &args![last, removed]);
        } else {
            e.mem.set_u32(moved, 0);
            e.vcall(meshes, 0xec, &args![last, moved]);
            let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
            e.vcall(meshes, 0xec, &args![particle, removed]);
            let meshes = e.get(this, NiMeshPSysData::m_spParticleMeshes);
            let child = e.mem.u32(moved);
            e.vcall(meshes, 0xf8, &args![particle, child]);
            let child = e.mem.u32(moved);
            if child != 0 {
                release(e, child);
            }
        }
        let object = e.mem.u32(removed);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
        fn_00c30df0(e, this, u32::from(generation), object);
        let object = e.mem.u32(removed);
        if object != 0 {
            release(e, object);
        }
    });
}

// Translated from 00c30f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a pool count): fewer pools than now
/// removes and deletes the last ones down to the count; more grows the pool array
/// (`00a90180`) and adds a new pool of `m_uiDefaultPoolSize` entries for each new index
/// (`0060bef0`, then `00ba3aa0` with the pool kept in the argument's stack word).
pub fn fn_00c30f90(e: &mut Engine, this: Ptr<NiMeshPSysData>, count: u32) {
    let base = this.addr();
    let current = u32::from(e.mem.u16(base + POOLS_SIZE));
    if count < current {
        // The exe tests `current <= count` again here (never true after the test above).
        if current <= count {
            return;
        }
        drop_pools_down_to(e, base, count);
        return;
    }
    if count == current {
        return;
    }
    e.call(POOLS_SET_SIZE_00A90180, &args![base + POOLS, count]);
    e.with_stack(4, |e, slot| {
        let mut index = current;
        while index < count {
            let size = e.get(this, NiMeshPSysData::m_uiDefaultPoolSize);
            add_pool(e, base, index, size, slot.addr());
            index += 1;
        }
    });
}

// Translated from 00c31090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::LoadBinary` (vtable slot +0x4c, `thiscall`, one word: the stream):
/// `NiPSysData::LoadBinary`; files up to version `0a010000` take `m_usVertices` as the
/// default pool size and one pool ([`fn_00c30f90`]); newer ones read the default pool size
/// (4 bytes), `m_bFillPoolsOnLoad` (1 byte, into the argument's stack word, stored as
/// non-zero), the pool count (4 bytes; the array grown with `00a90180`) and each pool's size
/// (4 bytes; a new pool of that size added for a size above 0). Both end with the stream's
/// link step (`00a645b0`).
pub fn ni_mesh_p_sys_data_load_binary(e: &mut Engine, this: Ptr<NiMeshPSysData>, stream: Ptr) {
    let base = this.addr();
    let stream = stream.addr();
    e.call(PSYS_DATA_LOAD_BINARY_00C25340, &args![this, stream]);
    if e.mem.u32(stream + STREAM_VERSION) <= POOL_DATA_VERSION {
        let vertices = u32::from(e.mem.u16(base + 8));
        e.set(this, NiMeshPSysData::m_uiDefaultPoolSize, vertices);
        fn_00c30f90(e, this, 1);
        e.call(STREAM_LINK_00A645B0, &args![stream]);
        return;
    }
    e.with_stack(0x10, |e, frame| {
        // The locals at EBP-0xc (the pool count), EBP-8 (a pool's size), EBP-4 (the
        // component size, then the new pool) and the argument's stack word.
        let (count_slot, size_slot, component_slot, argument) = (
            frame.addr(),
            frame.addr() + 4,
            frame.addr() + 8,
            frame.addr() + 0xc,
        );
        let input = (STREAM_INPUT_BINARY_STREAM, 8);
        e.mem.set_u32(argument, stream);
        stream_bytes(e, stream, input, base + 0x84, 4, component_slot);
        stream_bytes(e, stream, input, argument, 1, component_slot);
        let fill = u8::from(e.mem.u8(argument) != 0);
        e.set(this, NiMeshPSysData::m_bFillPoolsOnLoad, fill);
        stream_bytes(e, stream, input, count_slot, 4, component_slot);
        let count = e.mem.u32(count_slot);
        e.call(POOLS_SET_SIZE_00A90180, &args![base + POOLS, count]);
        let mut index = 0u32;
        while index < e.mem.u32(count_slot) {
            stream_bytes(e, stream, input, size_slot, 4, component_slot);
            let size = e.mem.u32(size_slot);
            if size != 0 {
                add_pool(e, base, index, size, component_slot);
            }
            index += 1;
        }
    });
    e.call(STREAM_LINK_00A645B0, &args![stream]);
}

// Translated from 00c31210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::GetViewerStrings` (vtable slot +0x60, `thiscall`, one word: the string
/// array): `NiPSysData::GetViewerStrings`, then the class name (RTTI `012027c0`) added to the
/// array (grown to growth + size when full; the string kept in the argument's stack word).
pub fn ni_mesh_p_sys_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiMeshPSysData>,
    strings: Ptr,
) {
    e.call(PSYS_DATA_GET_VIEWER_STRINGS_00C255B0, &args![this, strings]);
    let strings = strings.addr();
    let name = e.global::<u32>(NI_MESH_P_SYS_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c30a80, ni_mesh_p_sys_data_get_r_t_t_i() -> u32),
        entry!(0x00c30bd0, ni_mesh_p_sys_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c3_08d0,
            ni_mesh_p_sys_data_set_active_vertex_count(Ptr<NiMeshPSysData>, u32)
        ),
        entry!(
            0x00c3_0920,
            ni_mesh_p_sys_data_process_clone(Ptr<NiMeshPSysData>, u32)
        ),
        entry!(
            0x00c3_0940,
            ni_mesh_p_sys_data_register_streamables(Ptr<NiMeshPSysData>, u32) -> bool
        ),
        entry!(
            0x00c3_0970,
            ni_mesh_p_sys_data_is_equal(Ptr<NiMeshPSysData>, Ptr<NiMeshPSysData>) -> bool
        ),
        entry!(0x00c3_09a0, fn_00c309a0(Ptr, u32) -> Ptr),
        entry!(
            0x00c3_09d0,
            ni_mesh_p_sys_data_destructor(Ptr<NiMeshPSysData>)
        ),
        entry!(
            0x00c3_0a90,
            ni_mesh_p_sys_data_resolve_added_particles(Ptr<NiMeshPSysData>)
        ),
        entry!(
            0x00c3_0ae0,
            ni_mesh_p_sys_data_copy_members(Ptr<NiMeshPSysData>, Ptr<NiMeshPSysData>, u32)
        ),
        entry!(
            0x00c3_0b70,
            ni_mesh_p_sys_data_link_object(Ptr<NiMeshPSysData>, u32)
        ),
        entry!(
            0x00c3_0c00,
            ni_mesh_p_sys_data_constructor(Ptr<NiMeshPSysData>) -> Ptr<NiMeshPSysData>
        ),
        entry!(
            0x00c3_0c50,
            ni_mesh_p_sys_data_create_clone(Ptr<NiMeshPSysData>, u32) -> u32
        ),
        entry!(0x00c3_0ca0, fn_00c30ca0() -> u32),
        entry!(
            0x00c3_0cc0,
            ni_mesh_p_sys_data_save_binary(Ptr<NiMeshPSysData>, Ptr)
        ),
        entry!(0x00c3_0df0, fn_00c30df0(Ptr<NiMeshPSysData>, u32, u32)),
        entry!(
            0x00c3_0e80,
            ni_mesh_p_sys_data_remove_particle(Ptr<NiMeshPSysData>, u32)
        ),
        entry!(0x00c3_0f90, fn_00c30f90(Ptr<NiMeshPSysData>, u32)),
        entry!(
            0x00c3_1090,
            ni_mesh_p_sys_data_load_binary(Ptr<NiMeshPSysData>, Ptr)
        ),
        entry!(
            0x00c3_1210,
            ni_mesh_p_sys_data_get_viewer_strings(Ptr<NiMeshPSysData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_mesh_p_sys_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_09d0, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00c30bd0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c30bd0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x98]]);
        assert_eq!(calls_to(&e, 0x00c3_09d0), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_mesh_p_sys_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_0a80, &args![]).u32(), 0x0120_27c0);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    /// Slot `o` of the test vtable calls `0x0e10_0000 + o`.
    const SLOTS: u32 = 0x0e10_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// An object of `size` bytes with the test vtable and reference count `count`.
    fn object(e: &mut Engine, size: u32, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in 0..0x40u32 {
            e.mem.set_u32(vtable + 4 * slot, SLOTS + 4 * slot);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// Counting interlocked doubles.
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    /// Data with a mesh node and `pools` pool slots (each set pool 0x10 bytes with size and
    /// capacity words).
    fn data(e: &mut Engine, pools: &[u32]) -> (Ptr<NiMeshPSysData>, u32) {
        let this: Ptr<NiMeshPSysData> = e.new_object();
        let meshes = object(e, 0xb0, 1);
        e.set(this, NiMeshPSysData::m_spParticleMeshes, meshes);
        let base = e.mem.alloc(4 * pools.len() as u32 + 4);
        for (index, pool) in pools.iter().enumerate() {
            e.mem.set_u32(base + 4 * index as u32, *pool);
        }
        e.mem.set_u32(this.addr() + POOLS_BASE, base);
        e.mem.set_u16(this.addr() + POOLS_SIZE, pools.len() as u16);
        e.mem.set_u16(this.addr() + POOLS_USED, pools.len() as u16);
        (this, meshes)
    }

    #[test]
    fn set_active_vertex_count_detaches_and_clamps() {
        let mut e = Engine::new();
        noop(&mut e, &[SLOTS + 0xf0]);
        let (this, meshes) = data(&mut e, &[]);
        e.mem.set_u16(this.addr() + 8, 6);
        e.mem.set_u16(this.addr() + 0x50, 5);
        e.call_log = Some(vec![]);
        e.call(0x00c3_08d0, &args![this, 0x0001_0003u32]);
        assert_eq!(
            calls(&e, SLOTS + 0xf0),
            vec![vec![meshes, 3], vec![meshes, 4]]
        );
        assert_eq!(e.mem.u16(this.addr() + 0x50), 3);
        e.call(0x00c3_08d0, &args![this, 9u32]);
        assert_eq!(e.mem.u16(this.addr() + 0x50), 6);
    }

    #[test]
    fn clone_streaming_and_equality_reach_the_mesh_node() {
        let mut e = Engine::new();
        interlocked(&mut e);
        noop(
            &mut e,
            &[
                OBJECT_PROCESS_CLONE_00A5D430,
                SLOTS + 0x68,
                SLOTS + 0x54,
                PSYS_DATA_LINK_OBJECT_00A7B7D0,
                PSYS_DATA_COPY_MEMBERS_00C24F20,
                PSYS_DATA_CONSTRUCT_00C24C90,
            ],
        );
        e.register(PSYS_DATA_REGISTER_STREAMABLES_00A75F00, |_, _| {
            1u32.into_ret()
        });
        e.register(PSYS_DATA_IS_EQUAL_00C24DE0, |_, _| 1u32.into_ret());
        e.register(SLOTS + 0x5c, |_, a| u32::from(a[1] == 0x77).into_ret());
        let (this, meshes) = data(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0920, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, OBJECT_PROCESS_CLONE_00A5D430),
            vec![vec![this.addr(), 0x55]]
        );
        assert_eq!(calls(&e, SLOTS + 0x68), vec![vec![meshes, 0x55]]);
        assert!(e.call(0x00c3_0940, &args![this, 0x66u32]).bool());
        assert_eq!(calls(&e, SLOTS + 0x54), vec![vec![meshes, 0x66]]);
        e.register(PSYS_DATA_REGISTER_STREAMABLES_00A75F00, |_, _| {
            0u32.into_ret()
        });
        assert!(!e.call(0x00c3_0940, &args![this, 0x66u32]).bool());
        let (other, _) = data(&mut e, &[]);
        e.set(other, NiMeshPSysData::m_spParticleMeshes, 0x77);
        assert!(e.call(0x00c3_0970, &args![this, other]).bool());
        e.set(other, NiMeshPSysData::m_spParticleMeshes, 0x78);
        assert!(!e.call(0x00c3_0970, &args![this, other]).bool());
        // Link: the stream's object replaces the mesh node (old released, new held).
        let linked = object(&mut e, 0x10, 1);
        e.register_double(GET_OBJECT_FROM_LINK_ID_00A63430, move |_, _| {
            linked.into_ret()
        });
        noop(&mut e, &[SLOTS + 4]);
        e.call(0x00c3_0b70, &args![this, 0x88u32]);
        assert_eq!(e.get(this, NiMeshPSysData::m_spParticleMeshes), linked);
        assert_eq!(e.mem.u32(linked + 4), 2);
        assert_eq!(e.mem.u32(meshes + 4), 0);
        // Clone: constructed, the mesh node cloned (slot +0x48), the flag copied.
        let copy = object(&mut e, 0x10, 0);
        e.register_double(SLOTS + 0x48, move |_, _| copy.into_ret());
        e.set(this, NiMeshPSysData::m_bFillPoolsOnLoad, 1);
        let block = e.mem.alloc(0x98);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_0c50, &args![this, 0x99u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_MESH_P_SYS_DATA_VTABLE);
        assert_eq!(e.mem.u32(block + POOLS), POOLS_VTABLE);
        assert_eq!(e.mem.u16(block + POOLS_GROW_BY), 1);
        assert_eq!(e.mem.u32(block + 0x7c), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert_eq!(e.mem.u8(block + 0x80), 1);
        assert_eq!(
            calls(&e, PSYS_DATA_COPY_MEMBERS_00C24F20),
            vec![vec![this.addr(), block, 0x99]]
        );
        // The factory constructs; a failed allocation answers 0.
        assert_eq!(e.call(0x00c3_0ca0, &args![]).u32(), block);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c3_0ca0, &args![]).u32(), 0);
    }

    #[test]
    fn destructors_delete_pools_and_release_the_mesh_node() {
        let mut e = Engine::new();
        interlocked(&mut e);
        noop(
            &mut e,
            &[
                SLOTS,
                SLOTS + 4,
                FREE_00AA10F0,
                PSYS_DATA_DESTRUCT_00C24EF0,
                POOL_FREE_00AA4060,
            ],
        );
        let pool = object(&mut e, 0x10, 1);
        let (this, meshes) = data(&mut e, &[pool, 0]);
        let buffer = e.mem.u32(this.addr() + POOLS_BASE);
        e.call_log = Some(vec![]);
        e.call(0x00c3_09d0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_MESH_P_SYS_DATA_VTABLE);
        assert_eq!(e.mem.u16(this.addr() + POOLS_SIZE), 0);
        assert_eq!(e.mem.u16(this.addr() + POOLS_USED), 1);
        assert_eq!(calls(&e, SLOTS), vec![vec![pool, 1]]);
        assert_eq!(e.mem.u32(this.addr() + POOLS), POOLS_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![buffer]]);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![meshes]]);
        assert_eq!(
            calls(&e, PSYS_DATA_DESTRUCT_00C24EF0),
            vec![vec![this.addr()]]
        );
        // The pool array's deleting destructor.
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_09a0, &args![array, 1u32]).u32(), array);
        assert_eq!(e.mem.u32(array), POOLS_ARRAY_VTABLE);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x4444]]);
        assert_eq!(
            calls(&e, POOL_FREE_00AA4060),
            vec![vec![POOL_ALLOCATOR_011F6238, array]]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c3_09a0, &args![array, 0u32]);
        assert!(calls(&e, POOL_FREE_00AA4060).is_empty());
    }

    #[test]
    fn resolve_added_particles_shows_the_new_meshes() {
        let mut e = Engine::new();
        noop(&mut e, &[PSYS_DATA_RESOLVE_ADDED_PARTICLES_00C24D20]);
        let (this, meshes) = data(&mut e, &[]);
        let children = e.mem.alloc(0x10);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child + 0x30, 0x13);
        e.mem.set_u32(children + 8, child);
        e.mem.set_u32(meshes + 0xa0, children);
        e.mem.set_u16(this.addr() + 0x7a, 1);
        e.mem.set_u16(this.addr() + 0x78, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0a90, &args![this]);
        assert_eq!(e.mem.u32(child + 0x30), 0x12);
        assert_eq!(
            calls(&e, PSYS_DATA_RESOLVE_ADDED_PARTICLES_00C24D20),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn pools_take_back_meshes_and_grow_or_shrink() {
        let mut e = Engine::new();
        interlocked(&mut e);
        noop(
            &mut e,
            &[
                SLOTS,
                SLOTS + 4,
                POOL_ADD_004AFC50,
                POOLS_SET_SIZE_00A90180,
                POOLS_SET_AT_00BA3AA0,
                POOL_RESIZE_0060BEF0,
            ],
        );
        let pool = object(&mut e, 0x10, 1);
        e.mem.set_u16(pool + 8, 2);
        e.mem.set_u16(pool + 0xa, 1);
        let (this, _) = data(&mut e, &[pool]);
        let mesh = object(&mut e, 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0df0, &args![this, 0u32, mesh]);
        let added = calls(&e, POOL_ADD_004AFC50);
        assert_eq!(added.len(), 1);
        assert_eq!(e.mem.u32(added[0][1]), mesh);
        assert_eq!(e.mem.u32(mesh + 4), 1);
        // A full pool or an index past the pools only releases.
        e.mem.set_u16(pool + 0xa, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0df0, &args![this, 0u32, mesh]);
        e.call(0x00c3_0df0, &args![this, 5u32, mesh]);
        assert!(calls(&e, POOL_ADD_004AFC50).is_empty());
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![mesh]]);
        // Growing to three pools adds two new ones of the default size.
        let blocks = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        let mut next = 0usize;
        e.register_double(POOL_ALLOCATE_00AA3E40, move |_, _| {
            next += 1;
            blocks[next - 1].into_ret()
        });
        e.set(this, NiMeshPSysData::m_uiDefaultPoolSize, 7);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0f90, &args![this, 3u32]);
        assert_eq!(
            calls(&e, POOLS_SET_SIZE_00A90180),
            vec![vec![this.addr() + POOLS, 3]]
        );
        assert_eq!(e.mem.u32(blocks[1]), POOL_VTABLE);
        assert_eq!(e.mem.u16(blocks[1] + 0xe), 1);
        assert_eq!(
            calls(&e, POOL_RESIZE_0060BEF0),
            vec![vec![blocks[0], 7], vec![blocks[1], 7]]
        );
        let set: Vec<u32> = calls(&e, POOLS_SET_AT_00BA3AA0)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(set, vec![1, 2]);
        // Shrinking to none deletes the pool.
        e.call_log = Some(vec![]);
        e.call(0x00c3_0f90, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this.addr() + POOLS_SIZE), 0);
        assert_eq!(calls(&e, SLOTS), vec![vec![pool, 1]]);
        assert_eq!(e.mem.u16(this.addr() + POOLS_USED), 0);
    }

    #[test]
    fn remove_particle_returns_the_mesh_to_its_generations_pool() {
        let mut e = Engine::new();
        interlocked(&mut e);
        noop(
            &mut e,
            &[
                PSYS_DATA_REMOVE_PARTICLE_00C24CC0,
                POOL_ADD_004AFC50,
                SLOTS + 0xf8,
                SLOTS + 4,
            ],
        );
        let pool = e.mem.alloc(0x10);
        e.mem.set_u16(pool + 8, 4);
        let (this, meshes) = data(&mut e, &[0, pool]);
        let info = e.mem.alloc(0x1c * 3);
        e.mem.set_u16(info + 0x1c + 0x18, 1);
        e.mem.set_u32(this.addr() + 0x70, info);
        e.mem.set_u16(this.addr() + 0x50, 3);
        let removed = object(&mut e, 0x10, 1);
        let last = object(&mut e, 0x10, 1);
        e.register_double(SLOTS + 0xec, move |e, a| {
            let detached = if a[1] == 2 { last } else { removed };
            e.mem.set_u32(a[2], detached);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c3_0e80, &args![this, 1u32]);
        assert_eq!(
            calls(&e, PSYS_DATA_REMOVE_PARTICLE_00C24CC0),
            vec![vec![this.addr(), 1]]
        );
        let detached: Vec<u32> = calls(&e, SLOTS + 0xec).iter().map(|c| c[1]).collect();
        assert_eq!(detached, vec![2, 1]);
        assert_eq!(calls(&e, SLOTS + 0xf8), vec![vec![meshes, 1, last]]);
        let added = calls(&e, POOL_ADD_004AFC50);
        assert_eq!(added[0][0], pool);
        assert_eq!(e.mem.u32(added[0][1]), removed);
        // The local, the copy and the argument each released once.
        assert_eq!(e.mem.u32(last + 4), 0);
        assert_eq!(e.mem.u32(removed + 4), 0);
        // The last particle: only its own mesh is detached.
        e.mem.set_u32(removed + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0e80, &args![this, 2u32]);
        let detached: Vec<u32> = calls(&e, SLOTS + 0xec).iter().map(|c| c[1]).collect();
        assert_eq!(detached, vec![2]);
        assert!(calls(&e, SLOTS + 0xf8).is_empty());
    }

    #[test]
    fn binary_streaming_writes_and_reads_the_pools() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[
                PSYS_DATA_SAVE_BINARY_00C25560,
                PSYS_DATA_LOAD_BINARY_00C25340,
                SLOTS + 0x30,
                STREAM_LINK_00A645B0,
                POOLS_SET_SIZE_00A90180,
                POOLS_SET_AT_00BA3AA0,
                POOL_RESIZE_0060BEF0,
            ],
        );
        e.register_double(POOL_ALLOCATE_00AA3E40, |e, _| e.mem.alloc(0x10).into_ret());
        // The write function records (buffer word, count); the read function hands out
        // queued words.
        let log = e.mem.alloc(0x100);
        e.register_double(0x0e60_0000, move |e, a| {
            let at = e.mem.u32(log);
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            e.mem.set_u32(log + 4 + 8 * at, value);
            e.mem.set_u32(log + 8 + 8 * at, a[2]);
            e.mem.set_u32(log, at + 1);
            a[2].into_ret()
        });
        let queue = e.mem.alloc(0x40);
        for (index, value) in [9u32, 1, 2, 5, 0].iter().enumerate() {
            e.mem.set_u32(queue + 4 + 4 * index as u32, *value);
        }
        e.register_double(0x0e60_0010, move |e, a| {
            let at = e.mem.u32(queue);
            let value = e.mem.u32(queue + 4 + 4 * at);
            e.mem.set_u32(queue, at + 1);
            if a[2] == 1 {
                e.mem.set_u8(a[1], value as u8);
            } else {
                e.mem.set_u32(a[1], value);
            }
            a[2].into_ret()
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 0xc, 0x0e60_0000);
        e.mem.set_u32(binary + 8, 0x0e60_0010);
        let stream = object(&mut e, 0x260, 1);
        e.mem.set_u32(stream + STREAM_OUTPUT_BINARY_STREAM, binary);
        e.mem.set_u32(stream + STREAM_INPUT_BINARY_STREAM, binary);
        let pool = e.mem.alloc(0x10);
        e.mem.set_u16(pool + 8, 6);
        let (this, meshes) = data(&mut e, &[pool, 0]);
        e.set(this, NiMeshPSysData::m_uiDefaultPoolSize, 8);
        e.set(this, NiMeshPSysData::m_bFillPoolsOnLoad, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_0cc0, &args![this, stream]);
        let written: Vec<u32> = (0..5).map(|i| e.mem.u32(log + 4 + 8 * i)).collect();
        assert_eq!(written, vec![8, 1, 2, 6, 0]);
        assert_eq!(e.mem.u32(log + 0xc), 1);
        assert_eq!(calls(&e, SLOTS + 0x30), vec![vec![stream, meshes]]);
        // Loading a new file: the size, the flag, two pools, the first of 5 entries.
        e.mem
            .set_u32(stream + STREAM_VERSION, POOL_DATA_VERSION + 1);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1090, &args![this, stream]);
        assert_eq!(e.get(this, NiMeshPSysData::m_uiDefaultPoolSize), 9);
        assert_eq!(e.get(this, NiMeshPSysData::m_bFillPoolsOnLoad), 1);
        assert_eq!(
            calls(&e, POOLS_SET_SIZE_00A90180),
            vec![vec![this.addr() + POOLS, 2]]
        );
        assert_eq!(calls(&e, POOL_RESIZE_0060BEF0)[0][1], 5);
        assert_eq!(calls(&e, POOLS_SET_AT_00BA3AA0).len(), 1);
        assert_eq!(calls(&e, STREAM_LINK_00A645B0), vec![vec![stream]]);
        // An old file: the vertex count as the pool size, one pool.
        e.mem.set_u32(stream + STREAM_VERSION, POOL_DATA_VERSION);
        e.mem.set_u16(this.addr() + 8, 12);
        e.mem.set_u16(this.addr() + POOLS_SIZE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1090, &args![this, stream]);
        assert_eq!(e.get(this, NiMeshPSysData::m_uiDefaultPoolSize), 12);
        assert_eq!(
            calls(&e, POOLS_SET_SIZE_00A90180),
            vec![vec![this.addr() + POOLS, 1]]
        );
        assert_eq!(calls(&e, POOL_RESIZE_0060BEF0)[0][1], 12);
    }

    #[test]
    fn viewer_strings_add_the_class_name() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        e.set_global(NI_MESH_P_SYS_DATA_RTTI, 0x0abc_0000u32);
        noop(
            &mut e,
            &[
                PSYS_DATA_GET_VIEWER_STRINGS_00C255B0,
                STRINGS_GROW,
                STRINGS_SET_AT,
            ],
        );
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        let (this, _) = data(&mut e, &[]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 0xe, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1210, &args![this, strings]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        let set = calls(&e, STRINGS_SET_AT);
        assert_eq!(set[0][1], 3);
        assert_eq!(e.mem.u32(set[0][2]), 0x0abc_0001);
    }
}
