//! `BSMain/bsmasterparticlesystem.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

// Translated from 00c5eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00c5db90` and frees 0xe0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_master_particle_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x00c5_db90, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xe0u32]);
    }
    this
}

// Translated from 00c5ec90 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 0120436c without reading the receiver. The map names this body `BSMasterParticleSystem::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c5ec90(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_436c
}

layout! {
    /// `BSMasterParticleSystem` (Xbox PDB; every field 0x24 lower on PC, the
    /// `NiNode` base being smaller): the fields its functions use. 0xE0
    /// bytes, the size its `CreateObject` allocates.
    pub struct BSMasterParticleSystem: 0xE0 {
        /// `lEmitterObjList` (Xbox PDB): `NiTPointerList<NiPointer<NiAVObject>>`;
        /// its head node (nodes: next at +0, item at +8), tail +4, count +8.
        0xAC lEmitterObjList: u32,
        /// `usActiveEmitterObjCount` (Xbox PDB).
        0xB8 usActiveEmitterObjCount: u16,
        /// `usMaxEmitterObj` (Xbox PDB).
        0xBA usMaxEmitterObj: u16,
        /// `FirstEmitterObjIter` (Xbox PDB): a list node.
        0xC0 FirstEmitterObjIter: u32,
        /// `fMasterPSysTime` (Xbox PDB).
        0xDC fMasterPSysTime: f32,
    }
}

/// `ChildParticles` (Xbox PDB): the `NiTPrimitiveArray<NiParticles*>` at
/// +0xCC (`NiTArray` layout).
const CHILD_PARTICLES: u32 = 0xcc;
/// The count word of `lEmitterObjList` (+0xB4).
const EMITTER_LIST_COUNT: u32 = 0xb4;

/// `NiTPointerListBase<..>::RemoveAll` (`00e74d40`, engine map; `thiscall`).
const LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// A block free (`00aa10f0`, `cdecl`, one word; no Xbox PDB name).
const NI_FREE: u32 = 0x00aa_10f0;
/// A block allocation (`00aa1070`, `cdecl`, the size; no Xbox PDB name).
const NI_ALLOC: u32 = 0x00aa_1070;
/// `NiNew` allocation (`00aa13e0`, `cdecl`, the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base destructor `00a5e930` (`thiscall`; no Xbox PDB name).
const NODE_DESTRUCTOR_00A5E930: u32 = 0x00a5_e930;
/// The vtable of `BSMasterParticleSystem` (RTTI
/// `.?AVBSMasterParticleSystem@@`).
const VTABLE: u32 = 0x010c_2fb4;
/// The vtable of `NiTArray<NiParticles*, NiTMallocInterface<..>>` (RTTI
/// `.?AV?$NiTArray@PAVNiParticles@@...`).
const PARTICLES_ARRAY_VTABLE: u32 = 0x010c_30b8;
/// `00c5bfb0` (`thiscall` on the list, two words: out pointer, the address
/// of the cursor): takes the cursor's node out of the list.
const LIST_REMOVE_AT_00C5BFB0: u32 = 0x00c5_bfb0;
/// `InterlockedDecrement` (import slot `00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiNode::UpdateDownwardPass` (`00a5dd70`, engine map; `thiscall`, two
/// words).
const NODE_UPDATE_DOWNWARD_PASS: u32 = 0x00a5_dd70;
/// The constructor `00c5da70` (unit `BSMain/run_00c5da70`).
const CONSTRUCTOR_00C5DA70: u32 = 0x00c5_da70;
/// `NiNode::LoadBinary` (`00a5dd40`, engine map; `thiscall`, one word).
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
/// `00a30370` (`cdecl`: stream, address): loads a 16-bit value.
const STREAM_LOAD_SHORT_00A30370: u32 = 0x00a3_0370;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`, engine map; `thiscall`).
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// `NiNode::LinkObject` (`00a5e9d0`, engine map; `thiscall`, one word).
const NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`, engine map; `thiscall`).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// `00c4c920` (`thiscall` on the array, one word: the size; no Xbox PDB
/// name).
const ARRAY_SET_SIZE_00C4C920: u32 = 0x00c4_c920;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, engine map; `thiscall`).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `0096ae90` (`thiscall` on the array, two words: index, the address of
/// the element; no Xbox PDB name).
const ARRAY_SET_AT_0096AE90: u32 = 0x0096_ae90;
/// `NiObjectNET::PostLinkObject` (`00a5c6a0`, engine map; `thiscall`, one
/// word).
const OBJECT_NET_POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// `BSMasterParticleSystem::UpdateParticleSystemList` (`00c5de70`, engine
/// map; `thiscall`).
const UPDATE_PARTICLE_SYSTEM_LIST: u32 = 0x00c5_de70;

/// The stream's file version word (+4).
fn stream_version(e: &Engine, stream: Ptr) -> u32 {
    e.mem.u32(stream.addr() + 4)
}

// Translated from 00c5db90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5db90 - the C++ exception unwinding frame (FS:[0], handler 00f2fdcb) is not translated
/// The `BSMasterParticleSystem` destructor (`thiscall`; no Xbox PDB name;
/// the scalar deleting destructor `00c5eb20` runs it): stores the class
/// vtable, clears the cursor, empties the emitter list (`RemoveAll`),
/// clears and frees the child array at +0xCC (its elements nulled, its
/// size and used count cleared, the array's vtable stored, the elements
/// freed), empties the list again as its destructor does, and runs the base
/// destructor `00a5e930`. The exception-unwinding frame is not translated.
pub fn fn_00c5db90(e: &mut Engine, this: Ptr<BSMasterParticleSystem>) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0);
    e.call(LIST_REMOVE_ALL, &args![base + 0xac]);
    let array = base + CHILD_PARTICLES;
    let mut index: u16 = 0;
    while index < e.mem.u16(array + 0xa) {
        let elements = e.mem.u32(array + 4);
        e.mem
            .set_u32(elements.wrapping_add(u32::from(index) * 4), 0);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    e.mem.set_u32(array, PARTICLES_ARRAY_VTABLE);
    let elements = e.mem.u32(array + 4);
    e.call(NI_FREE, &args![elements]);
    e.call(LIST_REMOVE_ALL, &args![base + 0xac]);
    e.call(NODE_DESTRUCTOR_00A5E930, &args![this]);
}

// Translated from 00c5e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::ClearUnreferencedEmitters` (Xbox PDB, matched by
/// the engine map; `thiscall`): walks the emitter list from the cursor (the
/// head when unset, wrapping to the head at the end) for at most the list's
/// count of steps. An emitter with a reference count (+4) of 1 or bit 0 of
/// its +0x30 flags set is taken out (`00c5bfb0`) and released (reference
/// count decremented, deleted through its vtable +4 at zero); the others are
/// counted, the cursor saved where the count reaches the maximum. The saved
/// cursor is restored; the count becomes the smaller of the count and the
/// maximum, and a zero count clears the cursor.
pub fn bs_master_particle_system_clear_unreferenced_emitters(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) {
    let base = this.addr();
    e.set(this, BSMasterParticleSystem::usActiveEmitterObjCount, 0);
    if e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) == 0 {
        let head = e.get(this, BSMasterParticleSystem::lEmitterObjList);
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, head);
    }
    let mut saved = 0u32;
    let mut steps = e.mem.u32(base + EMITTER_LIST_COUNT);
    e.with_stack(4, |e, removed| {
        while e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) != 0 {
            let left = steps;
            steps = steps.wrapping_sub(1);
            if left == 0 {
                break;
            }
            let node = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
            let emitter = e.mem.u32(node + 8);
            let references = e.mem.u32(emitter + 4);
            if references == 1 || e.mem.u32(emitter + 0x30) & 1 != 0 {
                e.call(
                    LIST_REMOVE_AT_00C5BFB0,
                    &args![base + 0xac, removed, base + 0xc0],
                );
                let object = e.mem.u32(removed.addr());
                if object != 0 && e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
                    e.vcall(object, 4, &args![]);
                }
            } else {
                if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount)
                    == e.get(this, BSMasterParticleSystem::usMaxEmitterObj)
                {
                    saved = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
                }
                let count = e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount);
                e.set(
                    this,
                    BSMasterParticleSystem::usActiveEmitterObjCount,
                    count.wrapping_add(1),
                );
                let node = e.get(this, BSMasterParticleSystem::FirstEmitterObjIter);
                let next = if node != 0 { e.mem.u32(node) } else { 0 };
                e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, next);
            }
            if e.get(this, BSMasterParticleSystem::FirstEmitterObjIter) == 0 {
                let head = e.get(this, BSMasterParticleSystem::lEmitterObjList);
                e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, head);
            }
        }
    });
    if saved != 0 {
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, saved);
    }
    let maximum = e.get(this, BSMasterParticleSystem::usMaxEmitterObj);
    let count = e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount);
    let kept = if count < maximum { count } else { maximum };
    e.set(this, BSMasterParticleSystem::usActiveEmitterObjCount, kept);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount) == 0 {
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0);
    }
}

// Translated from 00c5e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::UpdateDownwardPass` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, flags)`, `time` the address of a
/// `float`): clears unreferenced emitters, then, with emitters left or a
/// child with active particles, adds the time to the system's own time
/// (+0xDC) and runs `NiNode::UpdateDownwardPass` with that accumulated time
/// in `*time`, restoring `*time` afterwards.
pub fn bs_master_particle_system_update_downward_pass(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    time: Ptr,
    flags: u32,
) {
    bs_master_particle_system_clear_unreferenced_emitters(e, this);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount) == 0
        && !bs_master_particle_system_child_has_active_particles(e, this)
    {
        return;
    }
    let own = this.addr() + 0xdc;
    let sum = x87_add(x87_load32(e.mem.u32(own)), x87_m32(e.mem.u32(time.addr())));
    e.mem.set_u32(own, x87_store32(sum));
    let saved = x87_quiet_bits(e.mem.u32(time.addr()));
    let accumulated = x87_quiet_bits(e.mem.u32(own));
    e.mem.set_u32(time.addr(), accumulated);
    e.call(NODE_UPDATE_DOWNWARD_PASS, &args![this, time, flags]);
    e.mem.set_u32(time.addr(), saved);
}

// Translated from 00c5e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5e5a0 - the C++ exception unwinding frame (FS:[0], handler 00f2fe48) is not translated
/// The class's `CreateObject` (`cdecl`, no words; no Xbox PDB name): a
/// 0xE0-byte block (`00aa13e0`) constructed by `00c5da70`, or null when the
/// allocation fails. The exception-unwinding frame is not translated.
pub fn fn_00c5e5a0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_NEW, &args![0xe0u32]).u32();
    if block != 0 {
        e.call(CONSTRUCTOR_00C5DA70, &args![block]).ptr()
    } else {
        Ptr::new(0)
    }
}

// Translated from 00c5e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::LoadBinary` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): `NiNode::LoadBinary`, the maximum
/// emitter count (+0xBA) and, for file versions above 0xF, the link ids
/// (`NiStream::ReadMultipleLinkIDs`).
pub fn bs_master_particle_system_load_binary(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    e.call(
        STREAM_LOAD_SHORT_00A30370,
        &args![stream, this.addr() + 0xba],
    );
    if stream_version(e, stream) > 0xf {
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    }
}

// Translated from 00c5e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::LinkObject` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): `NiNode::LinkObject`, then for
/// file versions above 0xF sizes the child array (+0xCC) to the stream's
/// link id count and stores each linked object at its index.
pub fn bs_master_particle_system_link_object(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(NODE_LINK_OBJECT, &args![this, stream]);
    if stream_version(e, stream) <= 0xf {
        return;
    }
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    if count == 0 {
        return;
    }
    let array = this.addr() + CHILD_PARTICLES;
    e.call(ARRAY_SET_SIZE_00C4C920, &args![array, count]);
    e.with_stack(4, |e, element| {
        let mut index = 0u32;
        while index < count {
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(element.addr(), object);
            e.call(ARRAY_SET_AT_0096AE90, &args![array, index, element]);
            index = index.wrapping_add(1);
        }
    });
}

// Translated from 00c5e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::PostLinkObject` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream):
/// `NiObjectNET::PostLinkObject`, then for file versions below 0x10
/// `BSMasterParticleSystem::UpdateParticleSystemList`.
pub fn bs_master_particle_system_post_link_object(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(OBJECT_NET_POST_LINK_OBJECT, &args![this, stream]);
    if stream_version(e, stream) < 0x10 {
        e.call(UPDATE_PARTICLE_SYSTEM_LIST, &args![this]);
    }
}

// Translated from 00c5eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::ChildHasActiveParticles` (Xbox PDB, matched by
/// the engine map; `thiscall`): whether any child of the array at +0xCC has
/// a +0xB8 object whose vtable +0x90 answers a positive 16-bit count.
pub fn bs_master_particle_system_child_has_active_particles(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) -> bool {
    let children = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
    let size = u32::from(e.get(children, NiTArray::m_usSize));
    let mut index = 0u32;
    while index < size {
        let elements = e.get(children, NiTArray::m_pBase);
        let child = e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)));
        if child != 0 {
            let data = e.mem.u32(child + 0xb8);
            if e.vcall(data, 0x90, &args![]).u16() > 0 {
                return true;
            }
        }
        index = index.wrapping_add(1);
    }
    false
}

// Translated from 00c5ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<NiParticles*, NiTMallocInterface<..>>` constructor
/// (`thiscall (this; max_size, grow_by)`, 16-bit words; no Xbox PDB name):
/// stores the vtable, the sizes, clears the size and used count and
/// allocates `max_size` elements (`00aa1070`), or none. Returns `this`.
pub fn fn_00c5ec10(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    max_size: u16,
    grow_by: u16,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), PARTICLES_ARRAY_VTABLE);
    e.set(this, NiTArray::m_usMaxSize, max_size);
    e.set(this, NiTArray::m_usGrowBy, grow_by);
    e.set(this, NiTArray::m_usSize, 0);
    e.set(this, NiTArray::m_usESize, 0);
    if e.get(this, NiTArray::m_usMaxSize) > 0 {
        let count = u32::from(e.get(this, NiTArray::m_usMaxSize));
        let block = e.call(NI_ALLOC, &args![count << 2]).u32();
        e.set(this, NiTArray::m_pBase, block);
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    this
}

// ---- Batch n067 ---------------------------------------------------------------------------

/// `005e03d0` (`thiscall` on the child array, no stack words; unnamed, the engine map
/// places it in `tesscriptfunctions.cpp`): what `UpdateParticleSystemList` runs on the
/// array before refilling it.
const ARRAY_00_5E03D0: u32 = 0x005e_03d0;
/// `00c4c8d0` (`thiscall` on the child array, the address of the element; unnamed): adds
/// an element.
const ARRAY_ADD_00C4C8D0: u32 = 0x00c4_c8d0;
/// The `NiRTTI` record of `BSPSysMultiTargetEmitterCtlr` (its `GetRTTI`, `00c5cea0`,
/// answers it).
const MULTI_TARGET_EMITTER_RTTI: u32 = 0x0120_3340;
/// `NiNode::UpdateSelectedDownwardPass` and `NiNode::UpdateRigidDownwardPass` (engine map;
/// `thiscall`, two words).
const NODE_UPDATE_SELECTED_DOWNWARD_PASS: u32 = 0x00a5_ded0;
const NODE_UPDATE_RIGID_DOWNWARD_PASS: u32 = 0x00a5_e1f0;
/// `00a5dc80` (unnamed, `ninode.cpp` range; `thiscall`, two words): the `NiNode` member
/// copy.
const NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
/// `NiNode::RegisterStreamables`, `NiNode::SaveBinary`, `NiNode::IsEqual` and
/// `NiNode::GetViewerStrings` (engine map; `thiscall`, one word).
const NODE_REGISTER_STREAMABLES: u32 = 0x00a5_e5b0;
const NODE_SAVE_BINARY: u32 = 0x00a5_e880;
const NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NODE_GET_VIEWER_STRINGS: u32 = 0x00a5_ea70;
/// `NiStreamSaveBinary<unsigned short>` and `NiStreamSaveBinary<unsigned int>` (engine
/// map; cdecl: the stream, the value's address).
const STREAM_SAVE_SHORT: u32 = 0x00a3_03a0;
const STREAM_SAVE_INT: u32 = 0x0044_adf0;
/// `NiGetViewerString` (cdecl, a name), `_ov8` (a label and a word), `_ov6` (a label and a
/// 16-bit value, pushed as a word) and `_ov10` (a label and an object) (engine map).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_INT: u32 = 0x00a7_d510;
const GET_VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const GET_VIEWER_STRING_OBJECT: u32 = 0x00a7_d5a0;
/// Adds a string to the viewer strings (`this` the strings, the address of a word holding
/// the string).
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// The class's `NiRTTI` record (`0120436c`): its first word is the class name the viewer
/// strings start with.
const RTTI: u32 = 0x0120_436c;
/// The viewer-string labels `"Index"` (`010c30e8`), `"Max Emitter Objs"` (`010c30d4`),
/// `"Emitter Objs"` (`010c30c4`) and `"No Name"` (`010c30bc`).
const INDEX_LABEL: u32 = 0x010c_30e8;
const MAX_EMITTERS_LABEL: u32 = 0x010c_30d4;
const EMITTERS_LABEL: u32 = 0x010c_30c4;
const NO_NAME_LABEL: u32 = 0x010c_30bc;
/// The memory manager singleton and its free (`thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `uiIndex` (Xbox PDB, +0xC4 on PC) and `usMaxParticlesPerFrame` (+0xBC on PC).
const INDEX: u32 = 0xc4;
const MAX_PARTICLES_PER_FRAME: u32 = 0xbc;

/// The child array at +0xCC.
fn child_particles(this: Ptr<BSMasterParticleSystem>) -> Ptr<NiTArray> {
    Ptr::new(this.addr() + CHILD_PARTICLES)
}

/// Element `index` of the child array (its base read each time).
fn child_at(e: &Engine, this: Ptr<BSMasterParticleSystem>, index: u32) -> u32 {
    let elements = e.get(child_particles(this), NiTArray::m_pBase);
    e.mem.u32(elements.wrapping_add(index.wrapping_mul(4)))
}

/// Adds `string` to `strings` through a local word holding it, as the exe does.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

// Translated from 00c5de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::UpdateParticleSystemList` (Xbox PDB; `thiscall`): `005e03d0`
/// on the child array (+0xCC), then the array refilled from the system's own subtree
/// ([`fn_00c5e280`] with the system as the node).
pub fn bs_master_particle_system_update_particle_system_list(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
) {
    let array = this.addr() + CHILD_PARTICLES;
    e.call(ARRAY_00_5E03D0, &args![array]);
    fn_00c5e280(e, this, this.cast(), Ptr::new(array));
}

// Translated from 00c5dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): sets the byte `+0xd8` of each non-null child
/// below the child array's size (read once).
pub fn fn_00c5dea0(e: &mut Engine, this: Ptr<BSMasterParticleSystem>) {
    let size = u32::from(e.get(child_particles(this), NiTArray::m_usSize));
    for index in 0..size {
        let child = child_at(e, this, index);
        if child != 0 {
            e.mem.set_u8(child + 0xd8, 1);
        }
    }
}

// Translated from 00c5df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): fills six words at `out`: the child
/// array's size (word 0), the sums over the non-null children of the 16-bit values at
/// +0x50 (word 1) and +8 (word 2) of their data (+0xB8, read for each), the active emitter
/// count (`+0xb8`, word 3), the emitter list's count (`+0xb4`, word 4) and the sum of the
/// children's words at +0xD0 (word 5). The sums wrap.
pub fn fn_00c5df00(e: &mut Engine, this: Ptr<BSMasterParticleSystem>, out: Ptr) {
    let size = u32::from(e.get(child_particles(this), NiTArray::m_usSize));
    let mut first = 0u32;
    let mut second = 0u32;
    let mut third = 0u32;
    for index in 0..size {
        let child = child_at(e, this, index);
        if child != 0 {
            let data = e.mem.u32(child + 0xb8);
            first = u32::from(e.mem.u16(data + 0x50)).wrapping_add(first);
            let data = e.mem.u32(child + 0xb8);
            second = u32::from(e.mem.u16(data + 8)).wrapping_add(second);
            third = third.wrapping_add(e.mem.u32(child + 0xd0));
        }
    }
    let out = out.addr();
    e.mem.set_u32(out + 4, first);
    e.mem.set_u32(out + 8, second);
    e.mem.set_u32(out + 0x14, third);
    let active = u32::from(e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount));
    e.mem.set_u32(out + 0xc, active);
    let listed = e.mem.u32(this.addr() + EMITTER_LIST_COUNT);
    e.mem.set_u32(out + 0x10, listed);
    e.mem.set_u32(out, size);
}

// Translated from 00c5e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): whether the controller list of `object`
/// (`+0xc`, next at `+0x30`) holds a `BSPSysMultiTargetEmitterCtlr` (`GetRTTI`, slot 8,
/// answers `01203340`).
pub fn fn_00c5e010(e: &mut Engine, object: Ptr) -> bool {
    let mut controller = e.mem.u32(object.addr() + 0xc);
    while controller != 0 {
        let is_emitter = if controller == 0 {
            false
        } else {
            e.vcall(controller, 8, &[]).u32() == MULTI_TARGET_EMITTER_RTTI
        };
        if is_emitter {
            return true;
        }
        controller = e.mem.u32(controller + 0x30);
    }
    false
}

// Translated from 00c5e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: a node and the child array): nothing
/// for a null node; a node whose slot 0x30 (`IsParticlesGeom`) answers non-zero is added
/// to the array (`00c4c8d0` with the address of a local holding it); any other node's
/// children (`IsNode`, slot 0xc; the count `+0xa6` read once) are visited recursively.
pub fn fn_00c5e280(e: &mut Engine, this: Ptr<BSMasterParticleSystem>, node: Ptr, array: Ptr) {
    let node = node.addr();
    if node == 0 {
        return;
    }
    if e.vcall(node, 0x30, &[]).u32() != 0 {
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), node);
            e.call(ARRAY_ADD_00C4C8D0, &args![array, local]);
        });
        return;
    }
    let as_node = if node != 0 {
        e.vcall(node, 0xc, &[]).u32()
    } else {
        0
    };
    if as_node == 0 {
        return;
    }
    let count = u32::from(e.mem.u16(as_node + 0xa6));
    for index in 0..count {
        let child = e.mem.u32(e.mem.u32(as_node + 0xa0).wrapping_add(4 * index));
        if child != 0 {
            fn_00c5e280(e, this, Ptr::new(child), array);
        }
    }
}

/// The body shared by the selected and rigid downward passes: as
/// [`bs_master_particle_system_update_downward_pass`] with `pass` as the `NiNode` pass.
fn downward_pass(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    time: Ptr,
    flags: u32,
    pass: u32,
) {
    bs_master_particle_system_clear_unreferenced_emitters(e, this);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount) == 0
        && !bs_master_particle_system_child_has_active_particles(e, this)
    {
        return;
    }
    let own = this.addr() + 0xdc;
    let sum = x87_add(x87_load32(e.mem.u32(own)), x87_m32(e.mem.u32(time.addr())));
    e.mem.set_u32(own, x87_store32(sum));
    let saved = x87_quiet_bits(e.mem.u32(time.addr()));
    let accumulated = x87_quiet_bits(e.mem.u32(own));
    e.mem.set_u32(time.addr(), accumulated);
    e.call(pass, &args![this, time, flags]);
    e.mem.set_u32(time.addr(), x87_quiet_bits(saved));
}

// Translated from 00c5e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::UpdateSelectedDownwardPass` (Xbox PDB; `thiscall (this; time,
/// flags)`, `time` the address of a `float`): clears unreferenced emitters, then, with
/// emitters left or a child with active particles, adds the time to the system's own time
/// (+0xDC) and runs `NiNode::UpdateSelectedDownwardPass` with that accumulated time in
/// `*time`, restoring `*time` afterwards.
pub fn bs_master_particle_system_update_selected_downward_pass(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    time: Ptr,
    flags: u32,
) {
    downward_pass(e, this, time, flags, NODE_UPDATE_SELECTED_DOWNWARD_PASS);
}

// Translated from 00c5e450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::UpdateRigidDownwardPass` (Xbox PDB): as
/// [`bs_master_particle_system_update_selected_downward_pass`] with
/// `NiNode::UpdateRigidDownwardPass`.
pub fn bs_master_particle_system_update_rigid_downward_pass(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    time: Ptr,
    flags: u32,
) {
    downward_pass(e, this, time, flags, NODE_UPDATE_RIGID_DOWNWARD_PASS);
}

// Translated from 00c5e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c5e4d0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSMasterParticleSystem::CreateClone` (Xbox PDB; `RET 4`): a new system
/// (`operator new(0xe0)`, `00c5da70`; null when the allocation fails), then this system's
/// members copied into it ([`fn_00c5e560`], which runs on a null clone too). Returns the
/// clone.
pub fn bs_master_particle_system_create_clone(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    cloning: Ptr,
) -> Ptr {
    let block = e.call(NI_NEW, &args![0xe0u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        e.call(CONSTRUCTOR_00C5DA70, &args![block]).u32()
    };
    fn_00c5e560(e, this, Ptr::new(clone), cloning);
    Ptr::new(clone)
}

// Translated from 00c5e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`): the system's `CopyMembers` as `CreateClone` calls
/// it: the `NiNode` part (`00a5dc80`), then `usMaxEmitterObj` (+0xBA).
pub fn fn_00c5e560(e: &mut Engine, this: Ptr<BSMasterParticleSystem>, clone: Ptr, cloning: Ptr) {
    e.call(NODE_COPY_MEMBERS, &args![this, clone, cloning]);
    let maximum = e.get(this, BSMasterParticleSystem::usMaxEmitterObj);
    e.set(
        clone.cast::<BSMasterParticleSystem>(),
        BSMasterParticleSystem::usMaxEmitterObj,
        maximum,
    );
}

// Translated from 00c5e720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::RegisterStreamables` (Xbox PDB): `NiNode::RegisterStreamables`
/// (false when it fails), then each non-null child of the array (its size read for each
/// step) gets its slot 0x54 with the stream. True.
pub fn bs_master_particle_system_register_streamables(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) -> bool {
    if !e
        .call(NODE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let mut index = 0u32;
    while index < u32::from(e.get(child_particles(this), NiTArray::m_usSize)) {
        let child = child_at(e, this, index);
        if child != 0 {
            e.vcall(child, 0x54, &args![stream]);
        }
        index += 1;
    }
    true
}

// Translated from 00c5e7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::SaveBinary` (Xbox PDB): `NiNode::SaveBinary`, `usMaxEmitterObj`
/// (`NiStreamSaveBinary<unsigned short>`), the child array's size as a word
/// (`NiStreamSaveBinary<unsigned int>` from a local), then each child (null ones too) through
/// the stream's slot 0x30.
pub fn bs_master_particle_system_save_binary(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    stream: Ptr,
) {
    e.call(NODE_SAVE_BINARY, &args![this, stream]);
    e.call(STREAM_SAVE_SHORT, &args![stream, this.addr() + 0xba]);
    let size = u32::from(e.get(child_particles(this), NiTArray::m_usSize));
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), size);
        e.call(STREAM_SAVE_INT, &args![stream, local]);
    });
    for index in 0..size {
        let child = child_at(e, this, index);
        e.vcall(stream.addr(), 0x30, &args![child]);
    }
}

// Translated from 00c5e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::IsEqual` (Xbox PDB): `NiNode::IsEqual`; then the active and
/// maximum emitter counts (+0xB8, +0xBA), the index (+0xC4) and the particles per frame
/// (+0xBC) must match; the two emitter lists (+0xAC; next at +0, item at +8) must hold the
/// same items while both last; the child arrays must have the same size, null in the
/// same places, and each non-null child's slot 0x5c (`IsEqual`) must hold with the other's;
/// and both lists must have ended together.
pub fn bs_master_particle_system_is_equal(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    other: Ptr<BSMasterParticleSystem>,
) -> bool {
    if !e.call(NODE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mut own = e.get(this, BSMasterParticleSystem::lEmitterObjList);
    let mut theirs = e.get(other, BSMasterParticleSystem::lEmitterObjList);
    if e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount)
        != e.get(other, BSMasterParticleSystem::usActiveEmitterObjCount)
        || e.get(this, BSMasterParticleSystem::usMaxEmitterObj)
            != e.get(other, BSMasterParticleSystem::usMaxEmitterObj)
        || e.mem.u32(this.addr() + INDEX) != e.mem.u32(other.addr() + INDEX)
        || e.mem.u16(this.addr() + MAX_PARTICLES_PER_FRAME)
            != e.mem.u16(other.addr() + MAX_PARTICLES_PER_FRAME)
    {
        return false;
    }
    while own != 0 && theirs != 0 {
        let own_item = e.mem.u32(own + 8);
        own = e.mem.u32(own);
        let their_item = e.mem.u32(theirs + 8);
        theirs = e.mem.u32(theirs);
        if own_item != their_item {
            return false;
        }
    }
    let size = u32::from(e.get(child_particles(this), NiTArray::m_usSize));
    if size != u32::from(e.get(child_particles(other), NiTArray::m_usSize)) {
        return false;
    }
    for index in 0..size {
        let mine = child_at(e, this, index);
        let their_child = child_at(e, other, index);
        if (mine != 0 && their_child == 0) || (mine == 0 && their_child != 0) {
            return false;
        }
        if mine != 0 && !e.vcall(mine, 0x5c, &args![their_child]).bool() {
            return false;
        }
    }
    own == 0 && theirs == 0
}

// Translated from 00c5e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMasterParticleSystem::GetViewerStrings` (Xbox PDB): `NiNode::GetViewerStrings`, then
/// strings for the class name (the first word of the `NiRTTI` record `0120436c`),
/// `"Index"` with `+0xc4`, `"Max Emitter Objs"` with `usMaxEmitterObj`, the label
/// `"Emitter Objs"`, and one per emitter of the list (+0xAC) with its name (its word at +8,
/// `"No Name"` when null) and the emitter itself, each added through `0096a610`.
pub fn bs_master_particle_system_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSMasterParticleSystem>,
    strings: Ptr,
) {
    e.call(NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(RTTI);
    let string = e.call(GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let index = e.mem.u32(this.addr() + INDEX);
    let string = e
        .call(GET_VIEWER_STRING_INT, &args![INDEX_LABEL, index])
        .u32();
    add_viewer_string(e, strings, string);
    let maximum = u32::from(e.get(this, BSMasterParticleSystem::usMaxEmitterObj));
    let string = e
        .call(GET_VIEWER_STRING_SHORT, &args![MAX_EMITTERS_LABEL, maximum])
        .u32();
    add_viewer_string(e, strings, string);
    let string = e.call(GET_VIEWER_STRING, &args![EMITTERS_LABEL]).u32();
    add_viewer_string(e, strings, string);
    let mut node = e.get(this, BSMasterParticleSystem::lEmitterObjList);
    while node != 0 {
        let item_at = node + 8;
        node = e.mem.u32(node);
        let emitter = e.mem.u32(item_at);
        let label = if e.mem.u32(emitter + 8) != 0 {
            e.mem.u32(emitter + 8)
        } else {
            NO_NAME_LABEL
        };
        let string = e
            .call(GET_VIEWER_STRING_OBJECT, &args![label, emitter])
            .u32();
        add_viewer_string(e, strings, string);
    }
}

// Translated from 00c5eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the child array type's destructor: stores the
/// `NiTArray<NiParticles*, ..>` vtable (`010c30b8`) and frees the elements (`00aa10f0`).
pub fn fn_00c5eb60(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), PARTICLES_ARRAY_VTABLE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(NI_FREE, &args![elements]);
}

// Translated from 00c5eca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPrimitiveArray<NiParticles*>::_scalar_deleting_destructor_` (engine map; `RET 4`):
/// the destructor's body (vtable `010c30b8`, elements freed with `00aa10f0`), then `this`
/// freed (memory manager) when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_primitive_array_ni_particles_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTArray>,
    flags: u32,
) -> Ptr<NiTArray> {
    e.mem.set_u32(this.addr(), PARTICLES_ARRAY_VTABLE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(NI_FREE, &args![elements]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c5_de70,
            bs_master_particle_system_update_particle_system_list(Ptr<BSMasterParticleSystem>)
        ),
        entry!(0x00c5_dea0, fn_00c5dea0(Ptr<BSMasterParticleSystem>)),
        entry!(0x00c5_df00, fn_00c5df00(Ptr<BSMasterParticleSystem>, Ptr)),
        entry!(0x00c5_e010, fn_00c5e010(Ptr) -> bool),
        entry!(
            0x00c5_e280,
            fn_00c5e280(Ptr<BSMasterParticleSystem>, Ptr, Ptr)
        ),
        entry!(
            0x00c5_e3d0,
            bs_master_particle_system_update_selected_downward_pass(
                Ptr<BSMasterParticleSystem>,
                Ptr,
                u32,
            )
        ),
        entry!(
            0x00c5_e450,
            bs_master_particle_system_update_rigid_downward_pass(
                Ptr<BSMasterParticleSystem>,
                Ptr,
                u32,
            )
        ),
        entry!(
            0x00c5_e4d0,
            bs_master_particle_system_create_clone(Ptr<BSMasterParticleSystem>, Ptr) -> Ptr
        ),
        entry!(
            0x00c5_e560,
            fn_00c5e560(Ptr<BSMasterParticleSystem>, Ptr, Ptr)
        ),
        entry!(
            0x00c5_e720,
            bs_master_particle_system_register_streamables(
                Ptr<BSMasterParticleSystem>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x00c5_e7a0,
            bs_master_particle_system_save_binary(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_e830,
            bs_master_particle_system_is_equal(
                Ptr<BSMasterParticleSystem>,
                Ptr<BSMasterParticleSystem>,
            ) -> bool
        ),
        entry!(
            0x00c5_e9f0,
            bs_master_particle_system_get_viewer_strings(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(0x00c5_eb60, fn_00c5eb60(Ptr<NiTArray>)),
        entry!(
            0x00c5_eca0,
            ni_t_primitive_array_ni_particles_scalar_deleting_destructor(
                Ptr<NiTArray>,
                u32,
            )
                -> Ptr<NiTArray>
        ),
        entry!(0x00c5_ec90, fn_00c5ec90(Ptr) -> u32),
        entry!(0x00c5_eb20, bs_master_particle_system_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c5_db90, fn_00c5db90(Ptr<BSMasterParticleSystem>)),
        entry!(
            0x00c5_e080,
            bs_master_particle_system_clear_unreferenced_emitters(Ptr<BSMasterParticleSystem>)
        ),
        entry!(
            0x00c5_e350,
            bs_master_particle_system_update_downward_pass(Ptr<BSMasterParticleSystem>, Ptr, u32)
        ),
        entry!(0x00c5_e5a0, fn_00c5e5a0() -> Ptr),
        entry!(
            0x00c5_e620,
            bs_master_particle_system_load_binary(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_e660,
            bs_master_particle_system_link_object(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_e6f0,
            bs_master_particle_system_post_link_object(Ptr<BSMasterParticleSystem>, Ptr)
        ),
        entry!(
            0x00c5_eb90,
            bs_master_particle_system_child_has_active_particles(
                Ptr<BSMasterParticleSystem>,
            ) -> bool
        ),
        entry!(
            0x00c5_ec10,
            fn_00c5ec10(Ptr<NiTArray>, u16, u16) -> Ptr<NiTArray>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_master_particle_system_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x00c5_eb20, 0x00c5_db90, 0xe0);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c5ec90_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_ec90, &args![0x1000u32]).u32(), 0x0120_436c);
        assert_eq!(e.call(0x00c5_ec90, &args![0u32]).u32(), 0x0120_436c);
    }
}

#[cfg(test)]
mod master_tests {
    use super::*;

    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;

    type CallLog = Vec<(u32, Vec<u32>)>;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine) -> CallLog {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    fn system(e: &mut Engine) -> Ptr<BSMasterParticleSystem> {
        e.new_object::<BSMasterParticleSystem>()
    }

    /// Nodes (next, -, item) for `items`, returned head first.
    fn nodes(e: &mut Engine, items: &[u32]) -> Vec<u32> {
        let mut list = vec![];
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
            list.push(node);
        }
        list.reverse();
        list
    }

    /// An emitter object (reference count +4, flags +0x30) whose vtable +4
    /// is `SLOT_A`.
    fn emitter(e: &mut Engine, references: u32, flags: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.put_vtable(0x0002_0000, &[0, SLOT_A]);
        e.mem.set_u32(object, 0x0002_0000);
        e.mem.set_u32(object + 4, references);
        e.mem.set_u32(object + 0x30, flags);
        object
    }

    fn list_doubles(e: &mut Engine) {
        // Takes the iterator's node out: hands its item over and moves the
        // iterator on.
        e.register(LIST_REMOVE_AT_00C5BFB0, |e, a| {
            let node = e.mem.u32(a[2]);
            e.mem.set_u32(a[1], e.mem.u32(node + 8));
            e.mem.set_u32(a[2], e.mem.u32(node));
            Ret::default()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(SLOT_A, |_, _| Ret::default());
    }

    #[test]
    fn destructor_empties_the_members_in_order() {
        let mut e = Engine::new();
        let this = system(&mut e);
        let elements = e.mem.alloc(0xc);
        for i in 0..3 {
            e.mem.set_u32(elements + 4 * i, 0x100 + i);
        }
        let array = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
        e.set(array, NiTArray::m_pBase, elements);
        e.set(array, NiTArray::m_usSize, 3);
        e.set(array, NiTArray::m_usESize, 2);
        e.set(this, BSMasterParticleSystem::FirstEmitterObjIter, 0x55);
        for callee in [LIST_REMOVE_ALL, NI_FREE, NODE_DESTRUCTOR_00A5E930] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c5_db90, &args![this]);
        assert_eq!(
            calls(&mut e),
            vec![
                (LIST_REMOVE_ALL, vec![this.addr() + 0xac]),
                (NI_FREE, vec![elements]),
                (LIST_REMOVE_ALL, vec![this.addr() + 0xac]),
                (NODE_DESTRUCTOR_00A5E930, vec![this.addr()]),
            ]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, BSMasterParticleSystem::FirstEmitterObjIter), 0);
        assert_eq!(e.mem.bytes(elements, 0xc), vec![0; 0xc]);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        assert_eq!(e.mem.u32(array.addr()), PARTICLES_ARRAY_VTABLE);
    }

    #[test]
    fn clear_unreferenced_emitters_drops_single_and_flagged_references() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = system(&mut e);
        let kept = emitter(&mut e, 2, 0);
        let last = emitter(&mut e, 1, 0);
        let flagged = emitter(&mut e, 3, 1);
        let list = nodes(&mut e, &[kept, last, flagged]);
        e.set(this, BSMasterParticleSystem::lEmitterObjList, list[0]);
        e.mem.set_u32(this.addr() + EMITTER_LIST_COUNT, 3);
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 0x14);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e080, &args![this]);
        let log = calls(&mut e);
        assert_eq!(
            e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount),
            1
        );
        assert_eq!(
            e.get(this, BSMasterParticleSystem::FirstEmitterObjIter),
            list[0]
        );
        assert_eq!(e.mem.u32(last + 4), 0);
        assert_eq!(e.mem.u32(flagged + 4), 2);
        let deleted: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SLOT_A)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(deleted, vec![last]);
    }

    #[test]
    fn clear_unreferenced_emitters_keeps_the_iterator_at_the_maximum() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let this = system(&mut e);
        let first = emitter(&mut e, 2, 0);
        let second = emitter(&mut e, 2, 0);
        let list = nodes(&mut e, &[first, second]);
        e.set(this, BSMasterParticleSystem::lEmitterObjList, list[0]);
        e.mem.set_u32(this.addr() + EMITTER_LIST_COUNT, 2);
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 1);
        e.call(0x00c5_e080, &args![this]);
        assert_eq!(
            e.get(this, BSMasterParticleSystem::usActiveEmitterObjCount),
            1
        );
        assert_eq!(
            e.get(this, BSMasterParticleSystem::FirstEmitterObjIter),
            list[1]
        );

        // No emitters: the iterator is cleared.
        let empty = system(&mut e);
        e.set(empty, BSMasterParticleSystem::usMaxEmitterObj, 0x14);
        e.call(0x00c5_e080, &args![empty]);
        assert_eq!(
            e.get(empty, BSMasterParticleSystem::usActiveEmitterObjCount),
            0
        );
        assert_eq!(e.get(empty, BSMasterParticleSystem::FirstEmitterObjIter), 0);
    }

    /// One child whose +0xB8 object answers `count` from vtable +0x90.
    fn child(e: &mut Engine, this: Ptr<BSMasterParticleSystem>, count: u32) {
        let data = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x94 / 4];
        slots[0x90 / 4] = SLOT_B;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(data, 0x0002_1000);
        e.register_double(SLOT_B, move |_, _| word(count));
        let particles = e.mem.alloc(0xc0);
        e.mem.set_u32(particles + 0xb8, data);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements + 4, particles);
        let array = Ptr::<NiTArray>::new(this.addr() + CHILD_PARTICLES);
        e.set(array, NiTArray::m_pBase, elements);
        e.set(array, NiTArray::m_usSize, 2);
    }

    #[test]
    fn child_has_active_particles_checks_each_child() {
        let mut e = Engine::new();
        let this = system(&mut e);
        child(&mut e, this, 0xffff_0003);
        assert!(e.call(0x00c5_eb90, &args![this]).bool());
        child(&mut e, this, 0xffff_0000);
        assert!(!e.call(0x00c5_eb90, &args![this]).bool());
    }

    #[test]
    fn update_downward_pass_runs_with_the_accumulated_time() {
        let mut e = Engine::new();
        let this = system(&mut e);
        child(&mut e, this, 1);
        e.set(this, BSMasterParticleSystem::fMasterPSysTime, 1.0);
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 0.5);
        e.register(NODE_UPDATE_DOWNWARD_PASS, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 1.5);
            assert_eq!(a[2], 7);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c5_e350, &args![this, time, 7u32]);
        assert_eq!(e.get(this, BSMasterParticleSystem::fMasterPSysTime), 1.5);
        assert_eq!(e.mem.f32(time), 0.5);
        assert_eq!(calls(&mut e).len(), 2);

        // Nothing active: no update.
        child(&mut e, this, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e350, &args![this, time, 7u32]);
        assert!(calls(&mut e)
            .iter()
            .all(|c| c.0 != NODE_UPDATE_DOWNWARD_PASS));
        assert_eq!(e.get(this, BSMasterParticleSystem::fMasterPSysTime), 1.5);
    }

    #[test]
    fn create_object_constructs_a_block() {
        let mut e = Engine::new();
        e.register(NI_NEW, |_, a| {
            assert_eq!(a[0], 0xe0);
            word(0x7000)
        });
        e.register(CONSTRUCTOR_00C5DA70, |_, a| word(a[0]));
        assert_eq!(e.call(0x00c5_e5a0, &args![]).u32(), 0x7000);
        e.register(NI_NEW, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_e5a0, &args![]).u32(), 0);
        assert!(calls(&mut e).iter().all(|c| c.0 != CONSTRUCTOR_00C5DA70));
    }

    fn stream(e: &mut Engine, version: u32) -> u32 {
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream + 4, version);
        stream
    }

    #[test]
    fn load_binary_reads_link_ids_from_version_0x10() {
        let mut e = Engine::new();
        let this = system(&mut e);
        for callee in [
            NODE_LOAD_BINARY,
            STREAM_LOAD_SHORT_00A30370,
            STREAM_READ_MULTIPLE_LINK_IDS,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e620, &args![this, new]);
        assert_eq!(
            calls(&mut e),
            vec![
                (NODE_LOAD_BINARY, vec![this.addr(), new]),
                (STREAM_LOAD_SHORT_00A30370, vec![new, this.addr() + 0xba]),
                (STREAM_READ_MULTIPLE_LINK_IDS, vec![new]),
            ]
        );
        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e620, &args![this, old]);
        assert_eq!(calls(&mut e).len(), 2);
    }

    #[test]
    fn link_object_stores_each_linked_child() {
        let mut e = Engine::new();
        let this = system(&mut e);
        e.register(NODE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(2));
        e.register(ARRAY_SET_SIZE_00C4C920, |_, _| Ret::default());
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut next = 0x110;
            move |_, _| {
                next += 0x110;
                word(next)
            }
        });
        e.register(ARRAY_SET_AT_0096AE90, |e, a| word(e.mem.u32(a[2])));
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, new]);
        let log = calls(&mut e);
        let array = this.addr() + CHILD_PARTICLES;
        assert_eq!(log[2], (ARRAY_SET_SIZE_00C4C920, vec![array, 2]));
        let set: Vec<_> = log
            .iter()
            .filter(|c| c.0 == ARRAY_SET_AT_0096AE90)
            .map(|c| c.1[1])
            .collect();
        assert_eq!(set, vec![0, 1]);

        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, old]);
        assert_eq!(calls(&mut e).len(), 1);
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x00c5_e660, &args![this, new]);
        assert_eq!(calls(&mut e).len(), 2);
    }

    #[test]
    fn post_link_object_updates_old_files_only() {
        let mut e = Engine::new();
        let this = system(&mut e);
        e.register(OBJECT_NET_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(UPDATE_PARTICLE_SYSTEM_LIST, |_, _| Ret::default());
        let old = stream(&mut e, 0xf);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e6f0, &args![this, old]);
        assert_eq!(
            calls(&mut e),
            vec![
                (OBJECT_NET_POST_LINK_OBJECT, vec![this.addr(), old]),
                (UPDATE_PARTICLE_SYSTEM_LIST, vec![this.addr()]),
            ]
        );
        let new = stream(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00c5_e6f0, &args![this, new]);
        assert_eq!(calls(&mut e).len(), 1);
    }

    #[test]
    fn particles_array_constructor_allocates_the_maximum() {
        let mut e = Engine::new();
        let array = e.new_object::<NiTArray>();
        e.register(NI_ALLOC, |_, a| word(0x8000 + a[0]));
        assert_eq!(
            e.call(0x00c5_ec10, &args![array, 3u32, 2u32]).u32(),
            array.addr()
        );
        assert_eq!(e.mem.u32(array.addr()), PARTICLES_ARRAY_VTABLE);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0x800c);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usGrowBy), 2);
        e.call(0x00c5_ec10, &args![array, 0u32, 1u32]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
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
    /// `IsParticlesGeom` (0x30) answer the words at +0x100, +0x104 and +0x108; slots 0x54
    /// and 0x5c are logged (0x5c answers the word at +0x10c).
    const MOCK_RTTI: u32 = 0x0090_3000;
    const MOCK_IS_NODE: u32 = 0x0090_3004;
    const MOCK_IS_PSYS: u32 = 0x0090_3008;
    const MOCK_REGISTER: u32 = 0x0090_300c;
    const MOCK_IS_EQUAL: u32 = 0x0090_3010;

    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(MOCK_RTTI, |e, a| e.mem.u32(a[0] + 0x100).into_ret());
        e.register(MOCK_IS_NODE, |e, a| e.mem.u32(a[0] + 0x104).into_ret());
        e.register(MOCK_IS_PSYS, |e, a| e.mem.u32(a[0] + 0x108).into_ret());
        e.register(MOCK_REGISTER, nothing);
        e.register(MOCK_IS_EQUAL, |e, a| e.mem.u32(a[0] + 0x10c).into_ret());
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in [
            (8, MOCK_RTTI),
            (0xc, MOCK_IS_NODE),
            (0x30, MOCK_IS_PSYS),
            (0x54, MOCK_REGISTER),
            (0x5c, MOCK_IS_EQUAL),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32) -> u32 {
        let object = e.mem.alloc(0x110);
        e.mem.set_u32(object, vtable);
        object
    }

    fn set_children(e: &mut Engine, node: u32, children: &[u32]) {
        e.mem.set_u32(node + 0x104, node);
        let array = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in (0u32..).zip(children) {
            e.mem.set_u32(array + 4 * i, *child);
        }
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
    }

    /// Sets the child-particles array (+0xCC) of `this`.
    fn set_particles(e: &mut Engine, this: u32, children: &[u32]) {
        let array = Ptr::<NiTArray>::new(this + CHILD_PARTICLES);
        let elements = e.mem.alloc(4 * children.len() as u32 + 4);
        for (i, child) in (0u32..).zip(children) {
            e.mem.set_u32(elements + 4 * i, *child);
        }
        e.set(array, NiTArray::m_pBase, elements);
        e.set(array, NiTArray::m_usSize, children.len() as u16);
    }

    #[test]
    fn particle_system_list_collects_the_particle_geometry() {
        let (mut e, vtable) = engine();
        e.register(ARRAY_00_5E03D0, nothing);
        e.register(ARRAY_ADD_00C4C8D0, |e, a| e.mem.u32(a[1]).into_ret());
        let first = object(&mut e, vtable);
        e.mem.set_u32(first + 0x108, 1);
        let deep = object(&mut e, vtable);
        e.mem.set_u32(deep + 0x108, 1);
        let inner = object(&mut e, vtable);
        set_children(&mut e, inner, &[deep]);
        let leaf = object(&mut e, vtable);
        let this = object(&mut e, vtable);
        set_children(&mut e, this, &[first, 0, inner, leaf]);
        e.call(0x00c5_de70, &args![this]);
        assert_eq!(calls(&e, ARRAY_00_5E03D0), vec![vec![this + 0xcc]]);
        let added: Vec<u32> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == ARRAY_ADD_00C4C8D0)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(added, vec![this + 0xcc, this + 0xcc]);
        e.call(0x00c5_e280, &args![this, 0u32, 0x500u32]);
        assert_eq!(calls(&e, ARRAY_ADD_00C4C8D0).len(), 2);
    }

    #[test]
    fn children_flags_and_statistics() {
        let (mut e, vtable) = engine();
        let this = object(&mut e, vtable);
        let (a, b) = (object(&mut e, vtable), object(&mut e, vtable));
        for (child, values) in [(a, (3u16, 4u16, 5u32)), (b, (10, 20, 30))] {
            let data = e.mem.alloc(0x54);
            e.mem.set_u16(data + 0x50, values.0);
            e.mem.set_u16(data + 8, values.1);
            e.mem.set_u32(child + 0xb8, data);
            e.mem.set_u32(child + 0xd0, values.2);
        }
        set_particles(&mut e, this, &[a, 0, b]);
        e.call(0x00c5_dea0, &args![this]);
        assert_eq!((e.mem.u8(a + 0xd8), e.mem.u8(b + 0xd8)), (1, 1));
        e.mem.set_u16(this + 0xb8, 7);
        e.mem.set_u32(this + 0xb4, 9);
        let out = e.mem.alloc(0x18);
        e.call(0x00c5_df00, &args![this, out]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![3, 13, 24, 7, 9, 35]);
    }

    #[test]
    fn finds_a_multi_target_emitter_controller() {
        let (mut e, vtable) = engine();
        let owner = object(&mut e, vtable);
        let first = object(&mut e, vtable);
        let second = object(&mut e, vtable);
        e.mem.set_u32(owner + 0xc, first);
        e.mem.set_u32(first + 0x30, second);
        e.mem.set_u32(second + 0x100, MULTI_TARGET_EMITTER_RTTI);
        assert!(e.call(0x00c5_e010, &args![owner]).bool());
        e.mem.set_u32(second + 0x100, 0x5000);
        assert!(!e.call(0x00c5_e010, &args![owner]).bool());
    }

    #[test]
    fn selected_and_rigid_passes_run_with_the_accumulated_time() {
        for (address, pass) in [
            (0x00c5_e3d0u32, NODE_UPDATE_SELECTED_DOWNWARD_PASS),
            (0x00c5_e450, NODE_UPDATE_RIGID_DOWNWARD_PASS),
        ] {
            let (mut e, vtable) = engine();
            let this = e.new_object::<BSMasterParticleSystem>();
            // A child whose data answers 2 active particles (slot 0x90).
            let data = object(&mut e, vtable);
            let data_vtable = e.mem.alloc(0x94);
            e.mem.set_u32(data_vtable + 0x90, 0x0090_3020);
            e.register(0x0090_3020, |_, _| 2u32.into_ret());
            e.mem.set_u32(data, data_vtable);
            let child = object(&mut e, vtable);
            e.mem.set_u32(child + 0xb8, data);
            set_particles(&mut e, this.addr(), &[child]);
            e.set(this, BSMasterParticleSystem::fMasterPSysTime, 1.0);
            let time = e.mem.alloc(4);
            e.mem.set_f32(time, 0.25);
            e.register(pass, |e, a| {
                assert_eq!(e.mem.f32(a[1]), 1.25);
                Ret::default()
            });
            e.call(address, &args![this, time, 3u32]);
            assert_eq!(calls(&e, pass), vec![vec![this.addr(), time, 3]]);
            assert_eq!(e.get(this, BSMasterParticleSystem::fMasterPSysTime), 1.25);
            assert_eq!(e.mem.f32(time), 0.25);
            set_particles(&mut e, this.addr(), &[]);
            e.call(address, &args![this, time, 3u32]);
            assert_eq!(calls(&e, pass).len(), 1);
        }
    }

    #[test]
    fn clone_copy_members_register_and_save() {
        let (mut e, vtable) = engine();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCTOR_00C5DA70, |_, a| a[0].into_ret());
        e.register(NODE_COPY_MEMBERS, nothing);
        let this = e.new_object::<BSMasterParticleSystem>();
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 12);
        let clone = e.call(0x00c5_e4d0, &args![this, 0x77u32]).u32();
        assert_eq!(calls(&e, NI_NEW), vec![vec![0xe0]]);
        assert_eq!(
            calls(&e, NODE_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
        assert_eq!(e.mem.u16(clone + 0xba), 12);
        let child = object(&mut e, vtable);
        set_particles(&mut e, this.addr(), &[0, child]);
        e.register_double(NODE_REGISTER_STREAMABLES, |_, a| (a[1] & 1).into_ret());
        assert!(e.call(0x00c5_e720, &args![this, 0x201u32]).bool());
        assert_eq!(calls(&e, MOCK_REGISTER), vec![vec![child, 0x201]]);
        assert!(!e.call(0x00c5_e720, &args![this, 0x200u32]).bool());
        assert_eq!(calls(&e, MOCK_REGISTER).len(), 1);
        e.register(NODE_SAVE_BINARY, nothing);
        e.register(STREAM_SAVE_SHORT, nothing);
        e.register(STREAM_SAVE_INT, |e, a| e.mem.u32(a[1]).into_ret());
        let stream = e.mem.alloc(4);
        let stream_vtable = e.mem.alloc(0x34);
        e.mem.set_u32(stream_vtable + 0x30, 0x0090_3030);
        e.register(0x0090_3030, nothing);
        e.mem.set_u32(stream, stream_vtable);
        e.call(0x00c5_e7a0, &args![this, stream]);
        assert_eq!(
            calls(&e, STREAM_SAVE_SHORT),
            vec![vec![stream, this.addr() + 0xba]]
        );
        assert_eq!(calls(&e, STREAM_SAVE_INT).len(), 1);
        assert_eq!(
            calls(&e, 0x0090_3030),
            vec![vec![stream, 0], vec![stream, child]]
        );
    }

    #[test]
    fn is_equal_compares_fields_lists_and_children() {
        let (mut e, vtable) = engine();
        e.register(NODE_IS_EQUAL, |_, _| 1u32.into_ret());
        let make = |e: &mut Engine, items: &[u32], children: &[u32]| {
            let this = e.mem.alloc(0x110);
            let mut next = 0u32;
            for item in items.iter().rev() {
                let node = e.mem.alloc(0xc);
                e.mem.set_u32(node, next);
                e.mem.set_u32(node + 8, *item);
                next = node;
            }
            e.mem.set_u32(this + 0xac, next);
            e.mem.set_u16(this + 0xb8, 1);
            e.mem.set_u16(this + 0xba, 2);
            e.mem.set_u32(this + 0xc4, 3);
            e.mem.set_u16(this + 0xbc, 4);
            set_particles(e, this, children);
            this
        };
        let child = object(&mut e, vtable);
        e.mem.set_u32(child + 0x10c, 1);
        let a = make(&mut e, &[5, 6], &[child, 0]);
        let b = make(&mut e, &[5, 6], &[child, 0]);
        assert!(e.call(0x00c5_e830, &args![a, b]).bool());
        assert_eq!(calls(&e, MOCK_IS_EQUAL), vec![vec![child, child]]);
        e.mem.set_u32(b + 0xc4, 4);
        assert!(!e.call(0x00c5_e830, &args![a, b]).bool());
        let c = make(&mut e, &[5, 7], &[child, 0]);
        assert!(!e.call(0x00c5_e830, &args![a, c]).bool());
        let d = make(&mut e, &[5], &[child, 0]);
        assert!(!e.call(0x00c5_e830, &args![a, d]).bool());
        let f = make(&mut e, &[5, 6], &[0, child]);
        assert!(!e.call(0x00c5_e830, &args![a, f]).bool());
        let g = make(&mut e, &[5, 6], &[child]);
        assert!(!e.call(0x00c5_e830, &args![a, g]).bool());
        e.mem.set_u32(child + 0x10c, 0x100);
        assert!(!e.call(0x00c5_e830, &args![a, b]).bool());
        e.register(NODE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c5_e830, &args![a, a]).bool());
    }

    #[test]
    fn viewer_strings_and_array_destructors() {
        let (mut e, _) = engine();
        e.register(NODE_GET_VIEWER_STRINGS, nothing);
        for address in [
            GET_VIEWER_STRING,
            GET_VIEWER_STRING_INT,
            GET_VIEWER_STRING_SHORT,
            GET_VIEWER_STRING_OBJECT,
        ] {
            e.register(address, |_, a| a[0].into_ret());
        }
        e.register(ADD_VIEWER_STRING, nothing);
        e.map(RTTI, 4);
        e.set_global(RTTI, 0x1234u32);
        let this = e.new_object::<BSMasterParticleSystem>();
        e.mem.set_u32(this.addr() + 0xc4, 9);
        e.set(this, BSMasterParticleSystem::usMaxEmitterObj, 0x8001);
        let named = e.mem.alloc(0xc);
        e.mem.set_u32(named + 8, 0x4444);
        let unnamed = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(second + 8, unnamed);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, named);
        e.mem.set_u32(this.addr() + 0xac, first);
        e.call(0x00c5_e9f0, &args![this, 0x900u32]);
        assert_eq!(
            calls(&e, GET_VIEWER_STRING),
            vec![vec![0x1234], vec![EMITTERS_LABEL]]
        );
        assert_eq!(calls(&e, GET_VIEWER_STRING_INT), vec![vec![INDEX_LABEL, 9]]);
        assert_eq!(
            calls(&e, GET_VIEWER_STRING_SHORT),
            vec![vec![MAX_EMITTERS_LABEL, 0x8001]]
        );
        assert_eq!(
            calls(&e, GET_VIEWER_STRING_OBJECT),
            vec![vec![0x4444, named], vec![NO_NAME_LABEL, unnamed]]
        );
        assert_eq!(calls(&e, ADD_VIEWER_STRING).len(), 6);
        e.register(NI_FREE, nothing);
        e.register(MEMORY_FREE, nothing);
        let array = e.new_object::<NiTArray>();
        e.set(array, NiTArray::m_pBase, 0x6000);
        e.call(0x00c5_eb60, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), PARTICLES_ARRAY_VTABLE);
        assert_eq!(calls(&e, NI_FREE), vec![vec![0x6000]]);
        assert_eq!(e.call(0x00c5_eca0, &args![array, 0u32]).u32(), array.addr());
        assert!(calls(&e, MEMORY_FREE).is_empty());
        e.call(0x00c5_eca0, &args![array, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, array.addr()]]
        );
        assert_eq!(calls(&e, NI_FREE).len(), 3);
    }
}
