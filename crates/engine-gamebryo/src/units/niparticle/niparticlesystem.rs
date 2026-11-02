//! `NiParticle/niparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

/// `NiParticleSystem::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_24e0;

// Translated from 00c1b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_system_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_SYSTEM_RTTI)
}

/// `00a80680`: the jump target of `00c1ad80`.
const JUMP_TARGET_00C1AD80: u32 = 0x00a8_0680;

// Translated from 00c1ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::SetModelData` (Xbox PDB): a 5-byte jump to `00a80680` (engine map: `NiGeometry::SetModelData`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetModelData@NiParticleSystem@@UAAXPAVNiGeometryData@@@Z`.
pub fn ni_particle_system_set_model_data(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C1AD80, &args![this, arg]);
}

/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The free-node list the list items go back to: its head (`011c5f58`), the two words after
/// the call `0040fbf0` is made on (`011c5f80`, cleared when the counter at `011c5f84`
/// reaches 0) and that counter, decremented once per node.
const FREE_NODES_011C5F58: u32 = 0x011c_5f58;
const FREE_LIST_STATE_011C5F80: u32 = 0x011c_5f80;
const COUNTER_011C5F84: u32 = 0x011c_5f84;
/// `0040fbf0` (unnamed): `thiscall` on `011c5f80`, `RET 4`.
const CALL_0040FBF0: u32 = 0x0040_fbf0;

// Translated from 00e74d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<NiPSysModifier>>::RemoveAll`
/// (Xbox PDB). `thiscall`, no stack words. Walks the list from its head; for each item: the
/// `NiPointer` element is released (reference count at +4 decremented, deleted through vtable
/// slot +4 when it reaches 0) and cleared when not null; `0040fbf0` runs on `011c5f80` with 0;
/// the item's previous link is cleared and the item is pushed onto the free list at `011c5f58`
/// (its next link takes the old head); the counter at `011c5f84` is decremented and, when it
/// reaches 0, the word at `011c5f80` is cleared. Then the count, head and tail are cleared.
pub fn ni_t_pointer_list_base_remove_all(e: &mut Engine, this: Ptr<NiTPointerList>) {
    let mut item = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkHead));
    while !item.is_null() {
        let element = e.get(item, NiTListItem::m_element);
        let next = e.get(item, NiTListItem::m_pkNext);
        if element != 0 {
            let left = e
                .call(
                    IMPORT_INTERLOCKED_DECREMENT,
                    &args![element.wrapping_add(4)],
                )
                .u32();
            if left == 0 {
                e.vcall(element, 4, &args![]);
            }
            e.set(item, NiTListItem::m_element, 0);
        }
        e.call(CALL_0040FBF0, &args![FREE_LIST_STATE_011C5F80, 0u32]);
        e.set(item, NiTListItem::m_pkPrev, 0);
        let free = e.global::<u32>(FREE_NODES_011C5F58);
        e.set(item, NiTListItem::m_pkNext, free);
        let counter = e.global::<u32>(COUNTER_011C5F84).wrapping_sub(1);
        e.set_global(COUNTER_011C5F84, counter);
        e.set_global(FREE_NODES_011C5F58, item.addr());
        if counter == 0 {
            e.set_global(FREE_LIST_STATE_011C5F80, 0u32);
        }
        item = Ptr::new(next);
    }
    e.set(this, NiTPointerList::m_uiCount, 0);
    e.set(this, NiTPointerList::m_pkHead, 0);
    e.set(this, NiTPointerList::m_pkTail, 0);
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The list-item allocation `0043a010` (`thiscall` on the list's +8, no stack words; it
/// answers a cleared item from the free list at `011c5f58`).
const LIST_NEW_ITEM_0043A010: u32 = 0x0043_a010;

// Translated from 00c1b4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiParticle/niparticlesystem.cpp`): adds an `NiPointer`
/// element at the head of an `NiTPointerList`. `thiscall`, `RET 4`, one word (the address of
/// the `NiPointer` to add). Takes an item from `0043a010` (on the list's +8); when the item's
/// element differs from the pointer's object, the old one is released (reference count at +4
/// through `InterlockedDecrement`, deleted through vtable slot +4 at 0) when not null, and the
/// new one stored and, when not null, held (`InterlockedIncrement` on its +4). Then the item's
/// previous link is cleared and its next link takes the old head; with an old head, that head's
/// previous link takes the item, otherwise the tail does; the count goes up by one and the item
/// becomes the head.
pub fn fn_00c1b4a0(e: &mut Engine, this: Ptr<NiTPointerList>, element: Ptr) {
    let count = this.addr().wrapping_add(8);
    let item = Ptr::<NiTListItem>::new(e.call(LIST_NEW_ITEM_0043A010, &args![count]).u32());
    let held = e.get(item, NiTListItem::m_element);
    if held != e.mem.u32(element.addr()) {
        if held != 0 {
            let left = e
                .call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(held, 4, &args![]);
            }
        }
        let object = e.mem.u32(element.addr());
        e.set(item, NiTListItem::m_element, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    e.set(item, NiTListItem::m_pkPrev, 0);
    let head = e.get(this, NiTPointerList::m_pkHead);
    e.set(item, NiTListItem::m_pkNext, head);
    let head = e.get(this, NiTPointerList::m_pkHead);
    if head != 0 {
        e.set(
            Ptr::<NiTListItem>::new(head),
            NiTListItem::m_pkPrev,
            item.addr(),
        );
        let n = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
        e.set(this, NiTPointerList::m_uiCount, n);
        e.set(this, NiTPointerList::m_pkHead, item.addr());
        return;
    }
    let n = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(this, NiTPointerList::m_uiCount, n);
    e.set(this, NiTPointerList::m_pkTail, item.addr());
    e.set(this, NiTPointerList::m_pkHead, item.addr());
}

layout! {
    /// `NiParticleSystem` (Xbox PDB), 0x110 bytes on PC (the Xbox build aligns its transforms
    /// to 16 bytes, so its offsets are higher). The fields its own functions use, at the PC
    /// offsets the code reads.
    pub struct NiParticleSystem: 0x110 {
        /// `NiAVObject::m_uFlags` (Xbox PDB).
        0x30 m_uFlags: u32,
        /// `NiGeometry::m_spModelData` (Xbox PDB): the particle data.
        0xb8 m_spModelData: u32,
        /// `m_bWorldSpace` (Xbox PDB).
        0xc4 m_bWorldSpace: bool,
        /// `m_kModifierList` (Xbox PDB): an `NiTPointerList` of `NiPointer<NiPSysModifier>`.
        0xc8 m_kModifierList: Inline<NiTPointerList>,
        /// `m_fLastTime` (Xbox PDB).
        0xd4 m_fLastTime: u32,
        /// `m_bResetSystem` (Xbox PDB).
        0xd8 m_bResetSystem: bool,
        /// `m_bDynamicBounds` (Xbox PDB).
        0xd9 m_bDynamicBounds: bool,
    }
}

/// `NiParticleSystem`'s world transform (`NiAVObject::m_kWorld`, 0xd words: rotation, then
/// translation at +0x8c, then scale at +0x98) and the copy of it the system keeps
/// (`m_kUnmodifiedWorld`, Xbox PDB).
const WORLD_TRANSFORM: u32 = 0x68;
const WORLD_TRANSLATE: u32 = 0x8c;
const WORLD_SCALE: u32 = 0x98;
const UNMODIFIED_WORLD: u32 = 0xdc;
/// The three words stored as the world translation in world space (`011f426c`) and the nine
/// stored as its rotation (`011a9448`).
const TRANSLATE_011F426C: u32 = 0x011f_426c;
const ROTATE_011A9448: u32 = 0x011a_9448;
/// The update data's flags read by the downward passes (+5, a byte) and set (+6).
const UPDATE_DATA_SKIP: u32 = 5;
const UPDATE_DATA_SKIPPED: u32 = 6;
/// `NiAVObject::UpdateDownwardPass` (engine map, `00a59f90`, `thiscall`, `RET 8`), the
/// selective pass `00a59fd0` (`thiscall`, `RET 8`), `NiAVObject::UpdateWorldData` (engine
/// map, `00a68c60`, `RET 4`) and `NiGeometry::UpdateWorldBound` (engine map, `00a80710`, no
/// stack words).
const UPDATE_DOWNWARD_PASS: u32 = 0x00a5_9f90;
const UPDATE_SELECTED_DOWNWARD_PASS_00A59FD0: u32 = 0x00a5_9fd0;
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const GEOMETRY_UPDATE_WORLD_BOUND: u32 = 0x00a8_0710;
/// The particle-system update's switches: a byte (`012024d8`), the object at `011f5b04`
/// (its byte +0x1b0, its vtable slot +0x8c), the thread id it compares (`012024dc`) and the
/// word passed to slot +0x8c (`011ae6e0`).
const UPDATE_SWITCH_012024D8: u32 = 0x0120_24d8;
const UPDATE_OBJECT_011F5B04: u32 = 0x011f_5b04;
const UPDATE_THREAD_012024DC: u32 = 0x0120_24dc;
const UPDATE_VALUE_011AE6E0: u32 = 0x011a_e6e0;
/// Imported `GetCurrentThreadId` (import slot).
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// `00c25c40` (no arguments; answers an object or null) and `00c258e0` (`thiscall` on that
/// object, `RET 8`: the system and the time), both unnamed.
const UPDATE_OBJECT_00C25C40: u32 = 0x00c2_5c40;
const UPDATE_WITH_00C258E0: u32 = 0x00c2_58e0;
/// `NiParticleSystem::PerformSystemReset` (decompiler name, `00c1b120`, no stack words),
/// `NiGeometryData::MarkAsChanged` (decompiler name, `00a67090`, `RET 4`).
const PERFORM_SYSTEM_RESET_00C1B120: u32 = 0x00c1_b120;
const MARK_AS_CHANGED_00A67090: u32 = 0x00a6_7090;
/// The `double` the last time is compared with (`010241b0`).
const UNSET_TIME_010241B0: u32 = 0x0102_41b0;
/// `NiGeometry::ProcessClone` (engine map, `00a80a10`, `RET 4`) and `00a53c30` (`thiscall`
/// on the cloning process's first word, `RET 8`: the object and the address of a word it
/// writes the clone to).
const GEOMETRY_PROCESS_CLONE: u32 = 0x00a8_0a10;
const CLONE_LOOKUP_00A53C30: u32 = 0x00a5_3c30;
/// `NiLines::LoadBinary` (decompiler name, `00a9b180`, `RET 4`), `NiStream::ReadMultipleLinkIDs`
/// (`00a64670`, `thiscall` on the stream, no stack words).
const LINES_LOAD_BINARY_00A9B180: u32 = 0x00a9_b180;
const READ_MULTIPLE_LINK_IDS_00A64670: u32 = 0x00a6_4670;
/// The stream's binary stream (+0x24c) and that object's read function (its word +8, called
/// `cdecl` with the object, the buffer, the count, the address of the component size and the
/// component count); the bytes read are added to its word +4.
const STREAM_BINARY_STREAM: u32 = 0x24c;
/// `NiLines::LinkObject` (decompiler name, `00a74780`, `RET 4`), `NiStream::GetNumberOfLinkIDs`
/// (`00a63460`) and `NiStream::GetObjectFromLinkID` (`00a63430`), both `thiscall` on the
/// stream with no stack words.
const LINES_LINK_OBJECT_00A74780: u32 = 0x00a7_4780;
const GET_NUMBER_OF_LINK_IDS_00A63460: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID_00A63430: u32 = 0x00a6_3430;
/// The list's insert-before (`00c1b520`, `thiscall`, `RET 8`: the item and the `NiPointer`'s
/// address) and add-tail (`00a5a310`, `thiscall`, `RET 4`: the `NiPointer`'s address).
const LIST_INSERT_BEFORE_00C1B520: u32 = 0x00c1_b520;
const LIST_ADD_TAIL_00A5A310: u32 = 0x00a5_a310;
/// The base constructor (`00a9b120`, engine map unit `NiMain`, no stack words), the base
/// `CopyMembers` (`00a9b140`, decompiler name `NiParticleMeshes::CopyMembers`, `RET 8`), the
/// Ni allocation (`00aa13e0`, `cdecl`: size), the vtable (`010bd44c`) and the `float` the last
/// time starts at (`01015f5c`).
const BASE_CONSTRUCT_00A9B120: u32 = 0x00a9_b120;
const BASE_COPY_MEMBERS_00A9B140: u32 = 0x00a9_b140;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_PARTICLE_SYSTEM_VTABLE_010BD44C: u32 = 0x010b_d44c;
const INITIAL_TIME_01015F5C: u32 = 0x0101_5f5c;

/// The reference count of an `NiRefObject` released (`InterlockedDecrement` on +4), deleted
/// through vtable slot +4 when that answers 0.
fn release(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// A `float` memory operand of an x87 instruction at `addr`.
fn m32_at(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// `FLD dword` of the `float` at `addr`.
fn load_at(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The elements of the modifier list, walked as the code walks it (the next link read before
/// the element is used).
fn for_each_modifier(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    mut f: impl FnMut(&mut Engine, u32),
) {
    let mut item = e.mem.u32(this.addr() + 0xc8);
    while item != 0 {
        let element = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        f(e, element);
    }
}

// Translated from 00a7fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBound::Update` (Xbox PDB), `thiscall`, `RET 8`: `this` becomes `bound` moved by the
/// transform `transform` (rotation rows at +0, +0xc, +0x18, translation +0x24, scale +0x30):
/// each centre coordinate is the translation plus the scale times the dot product of a row
/// with the centre (rounded to a `float` before the scaling), and the radius is the radius
/// times the scale. x87 operations in the exe's order.
pub fn ni_bound_update(e: &mut Engine, this: Ptr, bound: Ptr, transform: Ptr) {
    let b = bound.addr();
    let t = transform.addr();
    let mut rows = [0u32; 3];
    for (row, slot) in rows.iter_mut().enumerate() {
        let r = t + 0xc * row as u32;
        let (first, second) = if row == 0 {
            // FLD T+4, FMUL B+4; FLD B, FMUL T; FADDP.
            (
                x87_mul(load_at(e, r + 4), m32_at(e, b + 4)),
                x87_mul(load_at(e, b), m32_at(e, r)),
            )
        } else {
            // FLD T, FMUL B; FLD T+4, FMUL B+4; FADDP.
            (
                x87_mul(load_at(e, r), m32_at(e, b)),
                x87_mul(load_at(e, r + 4), m32_at(e, b + 4)),
            )
        };
        let sum = x87_add(first, second);
        let third = x87_mul(load_at(e, r + 8), m32_at(e, b + 8));
        *slot = x87_store32(x87_add(sum, third));
    }
    let scale_bits = x87_quiet_bits(e.mem.u32(t + 0x30));
    let scale = x87_load32(scale_bits);
    let scaled = [
        x87_store32(x87_mul(x87_load32(rows[0]), scale)),
        x87_store32(x87_mul(x87_load32(rows[1]), scale)),
        x87_store32(x87_mul(scale, x87_m32(rows[2]))),
    ];
    let mut out = [0u32; 3];
    for (index, word) in out.iter_mut().enumerate() {
        let offset = 4 * index as u32;
        *word = x87_store32(x87_add(
            load_at(e, t + 0x24 + offset),
            x87_m32(scaled[index]),
        ));
    }
    let this = this.addr();
    e.mem.set_u32(this, out[0]);
    e.mem.set_u32(this + 4, out[1]);
    e.mem.set_u32(this + 8, out[2]);
    let radius = x87_store32(x87_mul(load_at(e, b + 0xc), m32_at(e, t + 0x30)));
    e.mem.set_u32(this + 0xc, radius);
}

// Translated from 00c1ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::UpdateDownwardPass` (Xbox PDB), `thiscall`, `RET 8` (the update data and
/// a second word). When the update data's byte +5 is set, only its byte +6 is set; otherwise
/// the system's vtable slot +0xb8 runs with the update data, then `NiAVObject::UpdateDownwardPass`
/// (`00a59f90`) with both words.
pub fn ni_particle_system_update_downward_pass(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    update_data: Ptr,
    second: u32,
) {
    let data = update_data.addr();
    if e.mem.u8(data + UPDATE_DATA_SKIP) != 0 {
        e.mem.set_u8(data + UPDATE_DATA_SKIPPED, 1);
        return;
    }
    e.vcall(this.addr(), 0xb8, &args![data]);
    e.call(UPDATE_DOWNWARD_PASS, &args![this, data, second]);
}

// Translated from 00c1add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::UpdateWorldData` (Xbox PDB), `thiscall`, `RET 4`. Runs
/// `NiAVObject::UpdateWorldData` (`00a68c60`) with the word, copies the world transform (0xd
/// words at +0x68) to `m_kUnmodifiedWorld` (+0xdc), and in world space (`m_bWorldSpace`, read
/// before the copy) sets the world translation to the three words at `011f426c` and the
/// rotation to the nine at `011a9448`.
pub fn ni_particle_system_update_world_data(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    update_data: Ptr,
) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update_data]);
    let world_space = e.get(this, NiParticleSystem::m_bWorldSpace);
    let base = this.addr();
    for word in 0..0xdu32 {
        let value = e.mem.u32(base + WORLD_TRANSFORM + 4 * word);
        e.mem.set_u32(base + UNMODIFIED_WORLD + 4 * word, value);
    }
    if world_space {
        for word in 0..3u32 {
            let value = e.global::<u32>(TRANSLATE_011F426C + 4 * word);
            e.mem.set_u32(base + WORLD_TRANSLATE + 4 * word, value);
        }
        for word in 0..9u32 {
            let value = e.global::<u32>(ROTATE_011A9448 + 4 * word);
            e.mem.set_u32(base + WORLD_TRANSFORM + 4 * word, value);
        }
    }
}

// Translated from 00c1ae40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::UpdateSystem` (Xbox PDB), `thiscall`, `RET 4` (the time, a `float`).
/// When the byte `012024d8` is set, the object at `011f5b04` exists with its byte +0x1b0 set,
/// 0.0 is below the particle data's `float` +0x1c, the current thread is the one at
/// `012024dc` and `00c25c40` answers an object: `00c258e0` runs on that object with the
/// system and the time, then the `011f5b04` object's slot +0x8c with the object and the word
/// at `011ae6e0`; when that answers true the function returns, otherwise the object's slot
/// +0x94 runs. Then (and in every other case) the system's slot +0xf4 runs with the time.
pub fn ni_particle_system_update_system(e: &mut Engine, this: Ptr<NiParticleSystem>, time: f32) {
    let time_bits = x87_quiet_bits(time.to_bits());
    if e.global::<u8>(UPDATE_SWITCH_012024D8) != 0 {
        let manager = e.global::<u32>(UPDATE_OBJECT_011F5B04);
        if manager != 0 && e.mem.u8(manager + 0x1b0) != 0 {
            let data = e.get(this, NiParticleSystem::m_spModelData);
            // FLDZ; FCOMP [data+0x1c]; TEST AH,5; JP: on only when 0.0 < the value.
            if 0.0 < e.mem.f32(data + 0x1c) {
                let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &args![]).u32();
                if thread == e.global::<u32>(UPDATE_THREAD_012024DC) {
                    let object = e.call(UPDATE_OBJECT_00C25C40, &args![]).u32();
                    if object != 0 {
                        e.call(UPDATE_WITH_00C258E0, &args![object, this, time_bits]);
                        let manager = e.global::<u32>(UPDATE_OBJECT_011F5B04);
                        let word = e.global::<u32>(UPDATE_VALUE_011AE6E0);
                        if e.vcall(manager, 0x8c, &args![object, word]).bool() {
                            return;
                        }
                        e.vcall(object, 0x94, &args![]);
                    }
                }
            }
        }
    }
    e.vcall(this.addr(), 0xf4, &args![time_bits]);
}

// Translated from 00c1aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::InitializeNewParticle` (Xbox PDB), `thiscall`, `RET 4` (the particle's
/// index): each modifier's slot +0x90 runs with the particle data and the index.
pub fn ni_particle_system_initialize_new_particle(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    index: u32,
) {
    let data = e.get(this, NiParticleSystem::m_spModelData);
    for_each_modifier(e, this, |e, modifier| {
        e.vcall(modifier, 0x90, &args![data, index]);
    });
}

// Translated from 00c1af20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niparticlesystem.cpp`; a selective downward pass),
/// `thiscall`, `RET 8` (the update data and a second word). When the update data's byte +5 is
/// set, only its byte +6 is set; otherwise, with bit 2 of the flags (+0x30) set, the system's
/// slot +0xb8 runs with the update data, and then `00a59fd0` with both words.
pub fn fn_00c1af20(e: &mut Engine, this: Ptr<NiParticleSystem>, update_data: Ptr, second: u32) {
    let data = update_data.addr();
    if e.mem.u8(data + UPDATE_DATA_SKIP) != 0 {
        e.mem.set_u8(data + UPDATE_DATA_SKIPPED, 1);
        return;
    }
    if (e.get(this, NiParticleSystem::m_uFlags) >> 2) & 1 != 0 {
        e.vcall(this.addr(), 0xb8, &args![data]);
    }
    e.call(
        UPDATE_SELECTED_DOWNWARD_PASS_00A59FD0,
        &args![this, data, second],
    );
}

// Translated from 00c1af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::UpdateWorldBound` (Xbox PDB), no stack words. Runs
/// `NiGeometry::UpdateWorldBound` (`00a80710`); in world space the world bound (the object at
/// +0x20) becomes the particle data's bound (+0x10): with dynamic bounds (`m_bDynamicBounds`)
/// copied and then scaled (centre and radius) by the world scale (+0x98), otherwise moved by
/// `m_kUnmodifiedWorld` through `NiBound::Update`.
pub fn ni_particle_system_update_world_bound(e: &mut Engine, this: Ptr<NiParticleSystem>) {
    e.call(GEOMETRY_UPDATE_WORLD_BOUND, &args![this]);
    if !e.get(this, NiParticleSystem::m_bWorldSpace) {
        return;
    }
    let base = this.addr();
    let source = e
        .get(this, NiParticleSystem::m_spModelData)
        .wrapping_add(0x10);
    let bound = e.mem.u32(base + 0x20);
    if !e.get(this, NiParticleSystem::m_bDynamicBounds) {
        ni_bound_update(
            e,
            Ptr::new(bound),
            Ptr::new(source),
            Ptr::new(base + UNMODIFIED_WORLD),
        );
        return;
    }
    let word = e.mem.u32(source);
    let radius = x87_quiet_bits(e.mem.u32(source + 0xc));
    e.mem.set_u32(bound, word);
    let word = e.mem.u32(source + 4);
    e.mem.set_u32(bound + 4, word);
    let word = e.mem.u32(source + 8);
    e.mem.set_u32(bound + 0xc, radius);
    e.mem.set_u32(bound + 8, word);
    let scale = x87_load32(x87_quiet_bits(e.mem.u32(base + WORLD_SCALE)));
    let bound = e.mem.u32(base + 0x20);
    let x = x87_store32(x87_mul(x87_load32(e.mem.u32(bound)), scale));
    let y = x87_store32(x87_mul(x87_load32(e.mem.u32(bound + 4)), scale));
    let z = x87_store32(x87_mul(scale, x87_m32(e.mem.u32(bound + 8))));
    let r = x87_store32(x87_mul(
        x87_load32(e.mem.u32(bound + 0xc)),
        x87_m32(e.mem.u32(base + WORLD_SCALE)),
    ));
    e.mem.set_u32(bound, x);
    e.mem.set_u32(bound + 4, y);
    e.mem.set_u32(bound + 8, z);
    e.mem.set_u32(bound + 0xc, r);
}

// Translated from 00c1b020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::SetSelectiveUpdateFlags` (Xbox PDB), `thiscall`, `RET 0xc` (three bool
/// addresses; the second is not used). Stores 1 in the first and 0 in the third, then sets
/// flag bit 1 (+0x30) from the first byte as read back, bits 2 and 3, and bit 4 from the
/// third byte as read back.
pub fn ni_particle_system_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    selective: Ptr,
    _unused_2: Ptr,
    rigid: Ptr,
) {
    e.mem.set_u8(selective.addr(), 1);
    e.mem.set_u8(rigid.addr(), 0);
    let mut flags = e.get(this, NiParticleSystem::m_uFlags);
    if e.mem.u8(selective.addr()) != 0 {
        flags |= 2;
    } else {
        flags &= 0xffff_fffd;
    }
    flags |= 0xc;
    e.set(this, NiParticleSystem::m_uFlags, flags);
    if e.mem.u8(rigid.addr()) != 0 {
        e.set(this, NiParticleSystem::m_uFlags, flags | 0x10);
    } else {
        e.set(this, NiParticleSystem::m_uFlags, flags & 0xffff_ffef);
    }
}

// Translated from 00c1b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::ProcessClone` (Xbox PDB), `thiscall`, `RET 4` (the cloning process).
/// Runs `NiGeometry::ProcessClone` (`00a80a10`), then `00a53c30` on the cloning process's
/// first word with the system and the address of the argument's stack word (which it may
/// overwrite; the code goes on with the value it read first), then each modifier's slot
/// +0x68 and the particle data's slot +0x68 with the cloning process.
pub fn ni_particle_system_process_clone(e: &mut Engine, this: Ptr<NiParticleSystem>, cloning: Ptr) {
    let cloning = cloning.addr();
    e.call(GEOMETRY_PROCESS_CLONE, &args![this, cloning]);
    let map = e.mem.u32(cloning);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cloning);
        e.call(CLONE_LOOKUP_00A53C30, &args![map, this, slot]);
    });
    for_each_modifier(e, this, |e, modifier| {
        e.vcall(modifier, 0x68, &args![cloning]);
    });
    let data = e.get(this, NiParticleSystem::m_spModelData);
    e.vcall(data, 0x68, &args![cloning]);
}

// Translated from 00c1b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::Do_UpdateSystem` (Xbox PDB), `thiscall`, `RET 4` (the time, a `float`).
/// Notes whether the particle data's word +0x1c is 0, runs the data's slot +0xac; with
/// `m_bResetSystem` set or the time below `m_fLastTime` (ordered), runs
/// `PerformSystemReset` (`00c1b120`) and clears `m_bResetSystem`; when `m_fLastTime` equals
/// the `double` at `010241b0`, it becomes the time. Then each modifier: when its slot +0xa0
/// answers true it is kept as the last such one; otherwise, when its byte +0x14 is set, its
/// slot +0x8c runs with the time and the data. The last kept one, if any, then runs slot
/// +0x8c the same way when its byte +0x14 is set (only when the list is not empty).
/// `NiGeometryData::MarkAsChanged(7)` (`00a67090`) on the data; `m_fLastTime` becomes the
/// time. When the data's word +0x1c was 0 and no longer is, the first of the system and its
/// parents (+0x18) whose slot +0x10 answers nonzero gets flag 0x400000 (+0x30).
pub fn ni_particle_system_do_update_system(e: &mut Engine, this: Ptr<NiParticleSystem>, time: f32) {
    let time_bits = x87_quiet_bits(time.to_bits());
    let data = e.get(this, NiParticleSystem::m_spModelData);
    let was_empty = e.mem.u32(data + 0x1c) == 0;
    e.vcall(data, 0xac, &args![]);
    // FLD m_fLastTime; FLD time; FCOMPP; TEST AH,5; JP: reset when time < last (ordered).
    if e.get(this, NiParticleSystem::m_bResetSystem)
        || time < f32::from_bits(e.get(this, NiParticleSystem::m_fLastTime))
    {
        e.call(PERFORM_SYSTEM_RESET_00C1B120, &args![this]);
        e.set(this, NiParticleSystem::m_bResetSystem, false);
    }
    // FCOMP double; TEST AH,0x44; JP: equal (ordered) only.
    let unset: f64 = e.global(UNSET_TIME_010241B0);
    if f64::from(f32::from_bits(e.get(this, NiParticleSystem::m_fLastTime))) == unset {
        e.set(this, NiParticleSystem::m_fLastTime, time_bits);
    }
    let head = e.mem.u32(this.addr() + 0xc8);
    if head != 0 {
        let mut last = 0u32;
        for_each_modifier(e, this, |e, modifier| {
            if e.vcall(modifier, 0xa0, &args![]).bool() {
                last = modifier;
            } else if e.mem.u8(modifier + 0x14) != 0 {
                e.vcall(modifier, 0x8c, &args![time_bits, data]);
            }
        });
        if last != 0 && e.mem.u8(last + 0x14) != 0 {
            e.vcall(last, 0x8c, &args![time_bits, data]);
        }
    }
    e.call(MARK_AS_CHANGED_00A67090, &args![data, 7u32]);
    e.set(this, NiParticleSystem::m_fLastTime, time_bits);
    if was_empty && e.mem.u32(data + 0x1c) != 0 {
        let mut object = e.mem.u32(this.addr() + 0x18);
        while object != 0 {
            if e.vcall(object, 0x10, &args![]).u32() != 0 {
                let flags = e.mem.u32(object + 0x30);
                e.mem.set_u32(object + 0x30, flags | 0x40_0000);
                return;
            }
            object = e.mem.u32(object + 0x18);
        }
    }
}

// Translated from 00c1b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::LoadBinary` (Xbox PDB), `thiscall`, `RET 4` (the stream). Runs
/// `NiLines::LoadBinary` (`00a9b180`), reads one byte into the argument's own stack word
/// through the stream's binary stream (+0x24c: its read function at +8 with the component
/// size 1 and one component; the answer is added to its word +4), sets `m_bWorldSpace` to
/// whether that byte is nonzero, and runs `NiStream::ReadMultipleLinkIDs` (`00a64670`).
pub fn ni_particle_system_load_binary(e: &mut Engine, this: Ptr<NiParticleSystem>, stream: Ptr) {
    let stream = stream.addr();
    e.call(LINES_LOAD_BINARY_00A9B180, &args![this, stream]);
    let binary = e.mem.u32(stream + STREAM_BINARY_STREAM);
    let read = e.mem.u32(binary + 8);
    let byte = e.with_stack(8, |e, frame| {
        let slot = frame.addr();
        let size = slot + 4;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(size, 1);
        let count = e.call(read, &args![binary, slot, 1u32, size, 1u32]).u32();
        let total = e.mem.u32(binary + 4).wrapping_add(count);
        e.mem.set_u32(binary + 4, total);
        e.mem.u8(slot)
    });
    e.set(this, NiParticleSystem::m_bWorldSpace, byte != 0);
    e.call(READ_MULTIPLE_LINK_IDS_00A64670, &args![stream]);
}

// Translated from 00c1b5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niparticlesystem.cpp`): `NiParticleSystem`'s
/// constructor, no stack words. Runs the base constructor `00a9b120`, stores the vtable
/// `010bd44c`, sets `m_bWorldSpace`, clears the modifier list, sets `m_fLastTime` to the
/// `float` at `01015f5c` (through `FLD`/`FSTP`) and clears `m_bResetSystem` and
/// `m_bDynamicBounds`. Answers `this`.
pub fn fn_00c1b5c0(e: &mut Engine, this: Ptr<NiParticleSystem>) -> Ptr<NiParticleSystem> {
    e.call(BASE_CONSTRUCT_00A9B120, &args![this]);
    let initial = x87_quiet_bits(e.global::<u32>(INITIAL_TIME_01015F5C));
    let base = this.addr();
    e.mem.set_u32(base, NI_PARTICLE_SYSTEM_VTABLE_010BD44C);
    e.set(this, NiParticleSystem::m_bWorldSpace, true);
    e.mem.set_u32(base + 0xd0, 0);
    e.mem.set_u32(base + 0xc8, 0);
    e.mem.set_u32(base + 0xcc, 0);
    e.set(this, NiParticleSystem::m_fLastTime, initial);
    e.set(this, NiParticleSystem::m_bResetSystem, false);
    e.set(this, NiParticleSystem::m_bDynamicBounds, false);
    this
}

// Translated from 00c1b610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::AddModifier` (Xbox PDB), `thiscall`, `RET 4` (the modifier). The
/// modifier, held in the argument's stack word as an `NiPointer` for the call (its count
/// incremented before and released after): into an empty list at the head (`00c1b4a0`, only
/// then with null checks), before the first item whose modifier's order (+0xc) is above its
/// own (unsigned, `00c1b520`), or at the tail (`00a5a310`). Then the modifier's slot +0x9c
/// runs with the system.
pub fn ni_particle_system_add_modifier(e: &mut Engine, this: Ptr<NiParticleSystem>, modifier: Ptr) {
    let modifier = modifier.addr();
    let list = this.addr() + 0xc8;
    let mut item = e.mem.u32(list);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), modifier);
        if item == 0 {
            if modifier != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![modifier + 4]);
            }
            fn_00c1b4a0(e, Ptr::new(list), slot);
            if modifier != 0 {
                release(e, modifier);
            }
            return;
        }
        let order = e.mem.u32(modifier + 0xc);
        while item != 0 {
            let element = e.mem.u32(item + 8);
            if e.mem.u32(element + 0xc) > order {
                e.call(
                    IMPORT_INTERLOCKED_INCREMENT,
                    &args![modifier.wrapping_add(4)],
                );
                e.call(LIST_INSERT_BEFORE_00C1B520, &args![list, item, slot]);
                release(e, modifier);
                return;
            }
            item = e.mem.u32(item);
        }
        e.call(
            IMPORT_INTERLOCKED_INCREMENT,
            &args![modifier.wrapping_add(4)],
        );
        e.call(LIST_ADD_TAIL_00A5A310, &args![list, slot]);
        release(e, modifier);
    });
    e.vcall(modifier, 0x9c, &args![this]);
}

// Translated from 00c1b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niparticlesystem.cpp`): `NiParticleSystem::CopyMembers`,
/// `thiscall`, `RET 8` (the clone and the cloning process). Runs the base `CopyMembers`
/// (`00a9b140`), copies `m_bWorldSpace`, adds a clone of each modifier (its slot +0x48 with
/// the cloning process, held in the first argument's stack word as an `NiPointer`) at the
/// tail of the clone's list (`00a5a310`), copies `m_fLastTime` (`FLD`/`FSTP`) and
/// `m_bDynamicBounds`, and assigns the particle data's clone (its slot +0x48) to the
/// clone's `m_spModelData` (`NiPointer` assignment).
pub fn fn_00c1b6f0(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    clone: Ptr<NiParticleSystem>,
    cloning: Ptr,
) {
    let cloning = cloning.addr();
    e.call(BASE_COPY_MEMBERS_00A9B140, &args![this, clone, cloning]);
    let world_space = e.mem.u8(this.addr() + 0xc4);
    e.mem.set_u8(clone.addr() + 0xc4, world_space);
    let clone_list = clone.addr() + 0xc8;
    for_each_modifier(e, this, |e, modifier| {
        let copy = e.vcall(modifier, 0x48, &args![cloning]).u32();
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), copy);
            if copy != 0 {
                e.call(IMPORT_INTERLOCKED_INCREMENT, &args![copy + 4]);
            }
            e.call(LIST_ADD_TAIL_00A5A310, &args![clone_list, slot]);
            if copy != 0 {
                release(e, copy);
            }
        });
    });
    let last_time = x87_quiet_bits(e.get(this, NiParticleSystem::m_fLastTime));
    e.set(clone, NiParticleSystem::m_fLastTime, last_time);
    let dynamic = e.mem.u8(this.addr() + 0xd9);
    e.mem.set_u8(clone.addr() + 0xd9, dynamic);
    let data = e.get(this, NiParticleSystem::m_spModelData);
    let copy = e.vcall(data, 0x48, &args![cloning]).u32();
    let old = e.get(clone, NiParticleSystem::m_spModelData);
    if old != copy {
        if old != 0 {
            release(e, old);
        }
        e.set(clone, NiParticleSystem::m_spModelData, copy);
        if copy != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![copy + 4]);
        }
    }
}

// Translated from 00c1b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not a Ghidra function (reached through a pushed address, `00c1aa32`): the class's create
/// function. No arguments. Allocates 0x110 bytes (`00aa13e0`) and runs the constructor
/// (`00c1b5c0`, a tail jump) on them; answers the system, or 0 when the allocation failed.
pub fn fn_00c1b7f0(e: &mut Engine) -> Ptr<NiParticleSystem> {
    let block = e.call(NI_ALLOCATE, &args![0x110u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00c1b5c0(e, Ptr::new(block))
}

// Translated from 00c1b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::LinkObject` (Xbox PDB), `thiscall`, `RET 4` (the stream). Runs
/// `NiLines::LinkObject` (`00a74780`), then adds (`AddModifier`) one object from
/// `NiStream::GetObjectFromLinkID` (`00a63430`) per link id the stream counts
/// (`00a63460`).
pub fn ni_particle_system_link_object(e: &mut Engine, this: Ptr<NiParticleSystem>, stream: Ptr) {
    e.call(LINES_LINK_OBJECT_00A74780, &args![this, stream]);
    let mut count = e
        .call(GET_NUMBER_OF_LINK_IDS_00A63460, &args![stream])
        .u32();
    while count != 0 {
        let object = e
            .call(GET_OBJECT_FROM_LINK_ID_00A63430, &args![stream])
            .u32();
        ni_particle_system_add_modifier(e, this, Ptr::new(object));
        count -= 1;
    }
}

// Translated from 00c1b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::CreateClone` (Xbox PDB), `thiscall`, `RET 4` (the cloning process).
/// Allocates 0x110 bytes (`00aa13e0`), constructs a system in them (`00c1b5c0`; skipped when
/// the allocation failed, the clone then being 0), copies the members into it (`00c1b6f0`,
/// run in both cases) and answers it.
pub fn ni_particle_system_create_clone(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    cloning: Ptr,
) -> Ptr<NiParticleSystem> {
    let block = e.call(NI_ALLOCATE, &args![0x110u32]).u32();
    let clone = if block != 0 {
        fn_00c1b5c0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    fn_00c1b6f0(e, this, clone, cloning);
    clone
}

/// `NiLines::RegisterStreamables` (decompiler name, `00a9b190`, `thiscall`, `RET 4`; result
/// in `AL`), `NiLines::IsEqual` (decompiler name, `00a89770`, the same shape) and
/// `NiLines::SaveBinary` (decompiler name, `00a9b1a0`, `RET 4`).
const LINES_REGISTER_STREAMABLES_00A9B190: u32 = 0x00a9_b190;
const LINES_IS_EQUAL_00A89770: u32 = 0x00a8_9770;
const LINES_SAVE_BINARY_00A9B1A0: u32 = 0x00a9_b1a0;
/// The stream's output binary stream (+0x250) and that object's write function (its word
/// +0xc, called `cdecl` with the object, the buffer, the count, the address of the component
/// size and the component count); the bytes written are added to its word +4.
const STREAM_OUTPUT_BINARY_STREAM: u32 = 0x250;
/// The RTTI record `PerformSystemReset` looks for in each controller's RTTI chain
/// (`012027c8`).
const RESET_CONTROLLER_RTTI_012027C8: u32 = 0x0120_27c8;
/// `NiParticles::GetViewerStrings` (decompiler name, `00a9b2e0`, `RET 4`), the viewer string
/// makers (`cdecl`): `NiGetViewerString` (`00a7d330`, one word), of a byte (`00a7d370`), of
/// an unsigned word (`00a7d510`) and of a 16-bit word (`00a7d490`), each with a name; the
/// string array's grow (`0096ad30`, `thiscall`, one word) and set-at (`0096ae90`, `thiscall`,
/// two words: the index and the address of the string).
const PARTICLES_GET_VIEWER_STRINGS_00A9B2E0: u32 = 0x00a9_b2e0;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL_00A7D370: u32 = 0x00a7_d370;
const VIEWER_STRING_UNSIGNED_00A7D510: u32 = 0x00a7_d510;
const VIEWER_STRING_SHORT_00A7D490: u32 = 0x00a7_d490;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `NiParticleSystem`'s class name pointer (`012024e0`) and the viewer string names
/// (`World Space`, `NumMods`, `NumParticles`, `MaxParticles`).
const CLASS_NAME_012024E0: u32 = 0x0120_24e0;
const WORLD_SPACE_NAME: u32 = 0x0101_fe88;
const NUM_MODS_NAME: u32 = 0x010b_d564;
const NUM_PARTICLES_NAME: u32 = 0x010b_d554;
const MAX_PARTICLES_NAME: u32 = 0x010b_d544;

/// The modifier list's count (`m_kModifierList.m_uiCount`, +0xd0).
const MODIFIER_COUNT: u32 = 0xd0;

// Translated from 00c1b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::RegisterStreamables` (Xbox PDB, `thiscall`, one word: the stream):
/// `NiLines::RegisterStreamables` first (answers 0 when that fails), then each modifier's
/// virtual slot +0x54 with the stream. Answers 1.
pub fn ni_particle_system_register_streamables(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    stream: Ptr,
) -> bool {
    if !e
        .call(LINES_REGISTER_STREAMABLES_00A9B190, &args![this, stream])
        .bool()
    {
        return false;
    }
    for_each_modifier(e, this, |e, modifier| {
        e.vcall(modifier, 0x54, &args![stream]);
    });
    true
}

// Translated from 00c1b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiParticle/niparticlesystem.cpp`, `thiscall`, one word:
/// the address of a name word): the first modifier whose word +8 (its name) equals the word
/// at `name`, or 0.
pub fn fn_00c1b0f0(e: &mut Engine, this: Ptr<NiParticleSystem>, name: Ptr) -> u32 {
    let mut item = e.mem.u32(this.addr() + 0xc8);
    while item != 0 {
        let modifier = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        if modifier != 0 && e.mem.u32(modifier + 8) == e.mem.u32(name.addr()) {
            return modifier;
        }
    }
    0
}

// Translated from 00c1b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::PerformSystemReset` (decompiler name, `thiscall`): the particle data's
/// virtual slot +0x8c with 0, its 16-bit words +0x78 and +0x7a cleared, `m_fLastTime` set to
/// the `float` at `01015f5c`; each modifier's virtual slot +0x94; then each controller (from
/// `m_spControllers` at +0xc, the next at +0x30) whose RTTI chain (virtual slot +8, the base
/// at +4) holds the record at `012027c8` gets its virtual slot +0xec.
pub fn ni_particle_system_perform_system_reset(e: &mut Engine, this: Ptr<NiParticleSystem>) {
    let data = e.get(this, NiParticleSystem::m_spModelData);
    e.vcall(data, 0x8c, &args![0u32]);
    let start = x87_quiet_bits(e.global::<u32>(INITIAL_TIME_01015F5C));
    e.mem.set_u16(data + 0x78, 0);
    e.mem.set_u16(data + 0x7a, 0);
    e.set(this, NiParticleSystem::m_fLastTime, start);
    for_each_modifier(e, this, |e, modifier| {
        e.vcall(modifier, 0x94, &args![]);
    });
    let mut controller = e.mem.u32(this.addr() + 0xc);
    while controller != 0 {
        let mut rtti = e.vcall(controller, 8, &args![]).u32();
        let mut matches = false;
        while rtti != 0 {
            if rtti == RESET_CONTROLLER_RTTI_012027C8 {
                matches = true;
                break;
            }
            rtti = e.mem.u32(rtti + 4);
        }
        if matches {
            e.vcall(controller, 0xec, &args![]);
        }
        controller = e.mem.u32(controller + 0x30);
    }
}

// Translated from 00c1b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::IsEqual` (Xbox PDB, `thiscall`, one word: the other system):
/// `NiLines::IsEqual`, the same `m_bWorldSpace` and `m_bDynamicBounds`, then for each
/// modifier its virtual slot +0x5c with the other system's modifier of the same name (word
/// +8; 0 when there is none) must answer non-zero.
pub fn ni_particle_system_is_equal(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    other: Ptr<NiParticleSystem>,
) -> bool {
    if !e.call(LINES_IS_EQUAL_00A89770, &args![this, other]).bool() {
        return false;
    }
    if e.mem.u8(this.addr() + 0xc4) != e.mem.u8(other.addr() + 0xc4)
        || e.mem.u8(this.addr() + 0xd9) != e.mem.u8(other.addr() + 0xd9)
    {
        return false;
    }
    let mut item = e.mem.u32(this.addr() + 0xc8);
    while item != 0 {
        let modifier = e.mem.u32(item + 8);
        item = e.mem.u32(item);
        let mut theirs = e.mem.u32(other.addr() + 0xc8);
        let mut found = 0;
        while theirs != 0 {
            let candidate = e.mem.u32(theirs + 8);
            theirs = e.mem.u32(theirs);
            if candidate != 0 && e.mem.u32(candidate + 8) == e.mem.u32(modifier + 8) {
                found = candidate;
                break;
            }
        }
        if !e.vcall(modifier, 0x5c, &args![found]).bool() {
            return false;
        }
    }
    true
}

/// Writes `count` bytes from `buffer` through the stream's output binary stream (+0x250):
/// its write function (word +0xc) with the object, the buffer, the count, the address of the
/// component size (`size_slot`, set to `count`) and one component; the result is added to the
/// object's word +4.
fn write_binary(e: &mut Engine, stream: u32, buffer: u32, count: u32, size_slot: u32) {
    let binary = e.mem.u32(stream + STREAM_OUTPUT_BINARY_STREAM);
    e.mem.set_u32(size_slot, count);
    let write = e.mem.u32(binary + 0xc);
    let written = e
        .call(write, &args![binary, buffer, count, size_slot, 1u32])
        .u32();
    let total = e.mem.u32(binary + 4).wrapping_add(written);
    e.mem.set_u32(binary + 4, total);
}

// Translated from 00c1b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::SaveBinary` (Xbox PDB, `thiscall`, one word: the stream):
/// `NiLines::SaveBinary`, then `m_bWorldSpace` (one byte, written from the low byte of the
/// argument's stack word) and the modifier count (`+0xd0`, four bytes) through the stream's
/// output binary stream, then each modifier's link ID (the stream's virtual slot +0x30).
pub fn ni_particle_system_save_binary(e: &mut Engine, this: Ptr<NiParticleSystem>, stream: Ptr) {
    e.call(LINES_SAVE_BINARY_00A9B1A0, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        // The component size word, the count word and the argument's stack word.
        let (size_slot, word_slot, argument_slot) =
            (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        e.mem.set_u32(argument_slot, stream.addr());
        let world_space = e.mem.u8(this.addr() + 0xc4);
        e.mem.set_u8(argument_slot, world_space);
        write_binary(e, stream.addr(), argument_slot, 1, size_slot);
        let count = e.mem.u32(this.addr() + MODIFIER_COUNT);
        e.mem.set_u32(word_slot, count);
        write_binary(e, stream.addr(), word_slot, 4, size_slot);
    });
    for_each_modifier(e, this, |e, modifier| {
        e.vcall(stream.addr(), 0x30, &args![modifier]);
    });
}

// Translated from 00c1b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `NiParticle/niparticlesystem.cpp`): inserts an `NiPointer`
/// element before an item of an `NiTPointerList` (`thiscall`, `RET 8`: the item and the
/// address of the `NiPointer`). Takes an item from `0043a010` (on the list's +8); when its
/// element differs from the pointer's object, the old one is released (reference count at +4
/// through `InterlockedDecrement`, deleted through vtable slot +4 at 0) when not null, and the
/// new one stored and, when not null, held (`InterlockedIncrement` on its +4). The new item's
/// next link is `before` and its previous link `before`'s; that previous item's next link (or,
/// without one, the head) takes the new item, as does `before`'s previous link; the count goes
/// up by one. Answers the new item.
pub fn fn_00c1b520(
    e: &mut Engine,
    this: Ptr<NiTPointerList>,
    before: Ptr<NiTListItem>,
    element: Ptr,
) -> Ptr<NiTListItem> {
    let count = this.addr().wrapping_add(8);
    let item = Ptr::<NiTListItem>::new(e.call(LIST_NEW_ITEM_0043A010, &args![count]).u32());
    let held = e.get(item, NiTListItem::m_element);
    if held != e.mem.u32(element.addr()) {
        if held != 0 {
            let left = e
                .call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
                .u32();
            if left == 0 {
                e.vcall(held, 4, &args![]);
            }
        }
        let object = e.mem.u32(element.addr());
        e.set(item, NiTListItem::m_element, object);
        if object != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    e.set(item, NiTListItem::m_pkNext, before.addr());
    let previous = e.get(before, NiTListItem::m_pkPrev);
    e.set(item, NiTListItem::m_pkPrev, previous);
    let previous = e.get(before, NiTListItem::m_pkPrev);
    if previous != 0 {
        e.set(
            Ptr::<NiTListItem>::new(previous),
            NiTListItem::m_pkNext,
            item.addr(),
        );
    } else {
        e.set(this, NiTPointerList::m_pkHead, item.addr());
    }
    e.set(before, NiTListItem::m_pkPrev, item.addr());
    let n = e.get(this, NiTPointerList::m_uiCount).wrapping_add(1);
    e.set(this, NiTPointerList::m_uiCount, n);
    item
}

/// Adds the string at `slot` to the viewer string array (`NiTPrimitiveArray`: capacity +8,
/// size +0xa, growth +0xe, 16-bit words): grows it (`0096ad30`) to growth + size when full,
/// then sets it at the old size (`0096ae90`).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step.wrapping_add(size)]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00c1b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::GetViewerStrings` (Xbox PDB, `thiscall`, one word: the string array):
/// `NiParticles::GetViewerStrings`, then the class name (`012024e0`), `World Space`
/// (`m_bWorldSpace`), `NumMods` (the modifier count, +0xd0), `NumParticles` and
/// `MaxParticles` (the particle data's 16-bit words +0x50 and +8), each added to the array
/// (the string kept in the argument's stack word).
pub fn ni_particle_system_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiParticleSystem>,
    strings: Ptr,
) {
    e.call(PARTICLES_GET_VIEWER_STRINGS_00A9B2E0, &args![this, strings]);
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let name = e.global::<u32>(CLASS_NAME_012024E0);
        let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let world_space = u32::from(e.mem.u8(this.addr() + 0xc4));
        let text = e
            .call(
                VIEWER_STRING_BOOL_00A7D370,
                &args![WORLD_SPACE_NAME, world_space],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let count = e.mem.u32(this.addr() + MODIFIER_COUNT);
        let text = e
            .call(
                VIEWER_STRING_UNSIGNED_00A7D510,
                &args![NUM_MODS_NAME, count],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let data = e.get(this, NiParticleSystem::m_spModelData);
        let particles = u32::from(e.mem.u16(data + 0x50));
        let text = e
            .call(
                VIEWER_STRING_SHORT_00A7D490,
                &args![NUM_PARTICLES_NAME, particles],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let data = e.get(this, NiParticleSystem::m_spModelData);
        let maximum = u32::from(e.mem.u16(data + 8));
        let text = e
            .call(
                VIEWER_STRING_SHORT_00A7D490,
                &args![MAX_PARTICLES_NAME, maximum],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_fdd0, ni_bound_update(Ptr, Ptr, Ptr)),
        entry!(
            0x00c1_ad90,
            ni_particle_system_update_downward_pass(Ptr<NiParticleSystem>, Ptr, u32)
        ),
        entry!(
            0x00c1_add0,
            ni_particle_system_update_world_data(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_ae40,
            ni_particle_system_update_system(Ptr<NiParticleSystem>, f32)
        ),
        entry!(
            0x00c1_aee0,
            ni_particle_system_initialize_new_particle(Ptr<NiParticleSystem>, u32)
        ),
        entry!(0x00c1_af20, fn_00c1af20(Ptr<NiParticleSystem>, Ptr, u32)),
        entry!(
            0x00c1_af60,
            ni_particle_system_update_world_bound(Ptr<NiParticleSystem>)
        ),
        entry!(
            0x00c1_b020,
            ni_particle_system_set_selective_update_flags(Ptr<NiParticleSystem>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00c1_b060,
            ni_particle_system_process_clone(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_b260,
            ni_particle_system_do_update_system(Ptr<NiParticleSystem>, f32)
        ),
        entry!(
            0x00c1_b3a0,
            ni_particle_system_load_binary(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_b5c0,
            fn_00c1b5c0(Ptr<NiParticleSystem>) -> Ptr<NiParticleSystem>
        ),
        entry!(
            0x00c1_b610,
            ni_particle_system_add_modifier(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_b6f0,
            fn_00c1b6f0(Ptr<NiParticleSystem>, Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(0x00c1_b7f0, fn_00c1b7f0() -> Ptr<NiParticleSystem>),
        entry!(
            0x00c1_b810,
            ni_particle_system_link_object(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_b9e0,
            ni_particle_system_create_clone(Ptr<NiParticleSystem>, Ptr) -> Ptr<NiParticleSystem>
        ),
        entry!(0x00c1_b4a0, fn_00c1b4a0(Ptr<NiTPointerList>, Ptr)),
        entry!(0x00c1b5b0, ni_particle_system_get_rtti() -> Ptr),
        entry!(0x00c1_ad80, ni_particle_system_set_model_data(Ptr, Ptr)),
        entry!(
            0x00e7_4d40,
            ni_t_pointer_list_base_remove_all(Ptr<NiTPointerList>)
        ),
        entry!(
            0x00c1_b0b0,
            ni_particle_system_register_streamables(Ptr<NiParticleSystem>, Ptr) -> bool
        ),
        entry!(0x00c1_b0f0, fn_00c1b0f0(Ptr<NiParticleSystem>, Ptr) -> u32),
        entry!(
            0x00c1_b120,
            ni_particle_system_perform_system_reset(Ptr<NiParticleSystem>)
        ),
        entry!(
            0x00c1_b1d0,
            ni_particle_system_is_equal(Ptr<NiParticleSystem>, Ptr<NiParticleSystem>) -> bool
        ),
        entry!(
            0x00c1_b400,
            ni_particle_system_save_binary(Ptr<NiParticleSystem>, Ptr)
        ),
        entry!(
            0x00c1_b520,
            fn_00c1b520(Ptr<NiTPointerList>, Ptr<NiTListItem>, Ptr) -> Ptr<NiTListItem>
        ),
        entry!(
            0x00c1_b850,
            ni_particle_system_get_viewer_strings(Ptr<NiParticleSystem>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_system_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1b5b0, &args![]).u32(), NI_PARTICLE_SYSTEM_RTTI);
    }

    #[test]
    fn thunk_00c1ad80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c1_ad80;
        const TARGET: u32 = 0x00a8_0680;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    /// A list of three items: an element held once (deleted), a null element, and an
    /// element held twice (kept). The free list starts with one node and the counter at 2.
    #[test]
    fn remove_all_releases_the_elements_and_frees_every_item() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        let free_before = 0x0bad_0000u32;
        e.set_global(FREE_NODES_011C5F58, free_before);
        e.set_global(FREE_LIST_STATE_011C5F80, 0x77u32);
        e.set_global(COUNTER_011C5F84, 2u32);
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        let elements: Vec<u32> = (0..2).map(|_| e.mem.alloc(8)).collect();
        for (element, count) in elements.iter().zip([1u32, 2]) {
            e.mem.set_u32(*element, 0x0f00_1000);
            e.mem.set_u32(element + 4, count);
        }
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(12)).collect();
        for (i, item) in items.iter().enumerate() {
            let next = items.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(*item, next);
            e.mem.set_u32(item + 4, 0x5555);
        }
        e.mem.set_u32(items[0] + 8, elements[0]);
        e.mem.set_u32(items[1] + 8, 0);
        e.mem.set_u32(items[2] + 8, elements[1]);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, items[0]);
        e.mem.set_u32(list + 4, items[2]);
        e.mem.set_u32(list + 8, 3);

        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(0x0f00_2000, |_, _| Ret::default());
        e.register_double(CALL_0040FBF0, |e, a| {
            assert_eq!(a, &[FREE_LIST_STATE_011C5F80, 0][..]);
            // The word is cleared once the counter has reached 0 (after the second item).
            let counter = e.global::<u32>(COUNTER_011C5F84);
            let word = e.global::<u32>(FREE_LIST_STATE_011C5F80);
            assert_eq!(word == 0, counter == 0, "counter {counter:#x}");
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_4d40, &args![list]);

        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == 0x0f00_2000).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![elements[0]]);
        assert_eq!(log.iter().filter(|c| c.0 == CALL_0040FBF0).count(), 3);
        assert_eq!(e.mem.u32(elements[1] + 4), 1);
        // Items pushed onto the free list in order: the last one is the new head.
        assert_eq!(e.global::<u32>(FREE_NODES_011C5F58), items[2]);
        assert_eq!(e.mem.u32(items[2]), items[1]);
        assert_eq!(e.mem.u32(items[1]), items[0]);
        assert_eq!(e.mem.u32(items[0]), free_before);
        for item in &items {
            assert_eq!(e.mem.u32(item + 4), 0);
            assert_eq!(e.mem.u32(item + 8), 0);
        }
        // 2 - 3 wraps; the word was cleared when the counter reached 0 (second item).
        assert_eq!(e.global::<u32>(COUNTER_011C5F84), 0xffff_ffff);
        assert_eq!(e.global::<u32>(FREE_LIST_STATE_011C5F80), 0);
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
    }

    #[test]
    fn remove_all_on_an_empty_list_only_clears_it() {
        let mut e = Engine::new();
        let list = e.mem.alloc(12);
        e.mem.set_u32(list + 4, 0x1234);
        e.mem.set_u32(list + 8, 9);
        e.call_log = Some(vec![]);
        e.call(0x00e7_4d40, &args![list]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
    }

    /// An engine whose item allocation answers `item` (holding `stale` as its element) and
    /// whose interlocked doubles change the counts in memory.
    fn add_head_rig(item: &mut u32, stale: u32) -> Engine {
        let mut e = Engine::new();
        *item = e.mem.alloc(12);
        e.mem.set_u32(*item + 8, stale);
        let answer = *item;
        e.register_double(LIST_NEW_ITEM_0043A010, move |_, _| answer.into_ret());
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.put_vtable(0x0f00_1000, &[0, 0x0f00_2000]);
        e.register(0x0f00_2000, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00c1b4a0_adds_before_the_old_head_and_holds_the_element() {
        let mut item = 0;
        let mut e = add_head_rig(&mut item, 0);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object + 4, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, object);
        let old = e.mem.alloc(12);
        let list = e.mem.alloc(12);
        e.mem.set_u32(list, old);
        e.mem.set_u32(list + 4, old);
        e.mem.set_u32(list + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b4a0, &args![list, slot]);
        let log = e.call_log.take().unwrap();
        let new_item: Vec<_> = log
            .iter()
            .filter(|c| c.0 == LIST_NEW_ITEM_0043A010)
            .collect();
        assert_eq!(new_item[0].1, [list + 8]);
        assert_eq!(e.mem.u32(item + 8), object);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(e.mem.u32(item), old);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(old + 4), item);
        assert_eq!(e.mem.u32(list), item);
        assert_eq!(e.mem.u32(list + 4), old);
        assert_eq!(e.mem.u32(list + 8), 2);
    }

    #[test]
    fn fn_00c1b4a0_releases_a_stale_element_and_starts_an_empty_list() {
        let mut item = 0;
        let stale_object = 0x0f00_5000u32;
        let mut e = add_head_rig(&mut item, stale_object);
        e.map(stale_object, 8);
        e.mem.set_u32(stale_object, 0x0f00_1000);
        e.mem.set_u32(stale_object + 4, 1);
        // A null pointer to add: the stale element is released, nothing is held.
        let slot = e.mem.alloc(4);
        let list = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b4a0, &args![list, slot]);
        let log = e.call_log.take().unwrap();
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == 0x0f00_2000).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, [stale_object]);
        assert!(!log.iter().any(|c| c.0 == IMPORT_INTERLOCKED_INCREMENT));
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.mem.u32(list), item);
        assert_eq!(e.mem.u32(list + 4), item);
        assert_eq!(e.mem.u32(list + 8), 1);
    }

    #[test]
    fn fn_00c1b4a0_leaves_the_same_element_alone() {
        let mut item = 0;
        let object = 0x0f00_6000u32;
        let mut e = add_head_rig(&mut item, object);
        e.map(object, 8);
        e.mem.set_u32(object + 4, 5);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, object);
        let list = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b4a0, &args![list, slot]);
        let log = e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|c| c.0 == IMPORT_INTERLOCKED_INCREMENT || c.0 == IMPORT_INTERLOCKED_DECREMENT));
        assert_eq!(e.mem.u32(object + 4), 5);
    }
}

#[cfg(test)]
mod system_tests {
    use super::*;

    /// The targets of the test vtables: slot byte offset `o` of the table made with base `b`
    /// calls `b + o`.
    const SYSTEM_SLOTS: u32 = 0x0e10_0000;
    const MODIFIER_SLOTS: u32 = 0x0e20_0000;
    const DATA_SLOTS: u32 = 0x0e30_0000;
    const OTHER_SLOTS: u32 = 0x0e40_0000;
    const SYSTEM_VTABLE: u32 = 0x0e50_0000;
    const MODIFIER_VTABLE: u32 = 0x0e51_0000;
    const DATA_VTABLE: u32 = 0x0e52_0000;
    const OTHER_VTABLE: u32 = 0x0e53_0000;

    fn vtable(e: &mut Engine, at: u32, slots: u32) {
        let targets: Vec<u32> = (0..0x48u32).map(|i| slots + 4 * i).collect();
        e.put_vtable(at, &targets);
    }

    /// An engine with counting interlocked doubles and the four test vtables.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        vtable(&mut e, SYSTEM_VTABLE, SYSTEM_SLOTS);
        vtable(&mut e, MODIFIER_VTABLE, MODIFIER_SLOTS);
        vtable(&mut e, DATA_VTABLE, DATA_SLOTS);
        vtable(&mut e, OTHER_VTABLE, OTHER_SLOTS);
        e
    }

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn system(e: &mut Engine) -> Ptr<NiParticleSystem> {
        let this: Ptr<NiParticleSystem> = e.new_object();
        e.mem.set_u32(this.addr(), SYSTEM_VTABLE);
        this
    }

    /// An object of 0x40 bytes with the given vtable and reference count.
    fn object(e: &mut Engine, vtable: u32, count: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// Puts `modifiers` into the system's list (next, previous, element items).
    fn set_modifiers(e: &mut Engine, this: Ptr<NiParticleSystem>, modifiers: &[u32]) {
        let items: Vec<u32> = modifiers.iter().map(|_| e.mem.alloc(12)).collect();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(*item, items.get(i + 1).copied().unwrap_or(0));
            e.mem.set_u32(item + 8, modifiers[i]);
        }
        let base = this.addr();
        e.mem
            .set_u32(base + 0xc8, items.first().copied().unwrap_or(0));
        e.mem
            .set_u32(base + 0xcc, items.last().copied().unwrap_or(0));
        e.mem.set_u32(base + 0xd0, items.len() as u32);
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &Engine, addr: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    fn set_floats(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *v);
        }
    }

    #[test]
    fn bound_update_rotates_scales_and_moves_the_bound() {
        let mut e = Engine::new();
        let bound = e.mem.alloc(0x10);
        set_floats(&mut e, bound, &[1.0, 2.0, 3.0, 4.0]);
        let transform = e.mem.alloc(0x34);
        set_floats(
            &mut e,
            transform,
            &[
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
            ],
        );
        let out = e.mem.alloc(0x10);
        e.call(0x00a7_fdd0, &args![out, bound, transform]);
        assert_eq!(floats(&e, out, 4), vec![14.0, 22.0, 36.0, 8.0]);
        // In place: the bound is read before it is written.
        e.call(0x00a7_fdd0, &args![bound, bound, transform]);
        assert_eq!(floats(&e, bound, 4), vec![14.0, 22.0, 36.0, 8.0]);
    }

    #[test]
    fn update_downward_pass_marks_skipped_data_or_updates() {
        let mut e = rig();
        noop(&mut e, &[SYSTEM_SLOTS + 0xb8, UPDATE_DOWNWARD_PASS]);
        let this = system(&mut e);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data + 5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_ad90, &args![this, data, 9u32]);
        assert_eq!(e.mem.u8(data + 6), 1);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u8(data + 5, 0);
        e.mem.set_u8(data + 6, 0);
        e.call(0x00c1_ad90, &args![this, data, 9u32]);
        assert_eq!(e.mem.u8(data + 6), 0);
        assert_eq!(
            calls(&e, SYSTEM_SLOTS + 0xb8),
            vec![vec![this.addr(), data]]
        );
        assert_eq!(
            calls(&e, UPDATE_DOWNWARD_PASS),
            vec![vec![this.addr(), data, 9]]
        );
    }

    #[test]
    fn update_world_data_keeps_the_world_and_resets_it_in_world_space() {
        let mut e = rig();
        noop(&mut e, &[AV_OBJECT_UPDATE_WORLD_DATA]);
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        for i in 0..3u32 {
            e.set_global(TRANSLATE_011F426C + 4 * i, 0x100 + i);
        }
        for i in 0..9u32 {
            e.set_global(ROTATE_011A9448 + 4 * i, 0x200 + i);
        }
        for world_space in [false, true] {
            let this = system(&mut e);
            for i in 0..0xdu32 {
                e.mem.set_u32(this.addr() + 0x68 + 4 * i, 0x300 + i);
            }
            e.set(this, NiParticleSystem::m_bWorldSpace, world_space);
            e.call_log = Some(vec![]);
            e.call(0x00c1_add0, &args![this, 0x77u32]);
            assert_eq!(
                calls(&e, AV_OBJECT_UPDATE_WORLD_DATA),
                vec![vec![this.addr(), 0x77]]
            );
            for i in 0..0xdu32 {
                assert_eq!(e.mem.u32(this.addr() + 0xdc + 4 * i), 0x300 + i);
            }
            for i in 0..9u32 {
                let expected = if world_space { 0x200 + i } else { 0x300 + i };
                assert_eq!(e.mem.u32(this.addr() + 0x68 + 4 * i), expected);
            }
            for i in 0..3u32 {
                let expected = if world_space { 0x100 + i } else { 0x309 + i };
                assert_eq!(e.mem.u32(this.addr() + 0x8c + 4 * i), expected);
            }
        }
    }

    /// A system whose data's `float` +0x1c is 1.0, with the update switches on and the
    /// manager answering `manager_answer` from its slot +0x8c. Answers the engine, the
    /// system, the manager and the update object.
    fn update_system_rig(manager_answer: u32) -> (Engine, Ptr<NiParticleSystem>, u32, u32) {
        let mut e = rig();
        e.map(0x0120_2000, 0x1000);
        e.map(0x011f_5000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.mem.set_f32(data + 0x1c, 1.0);
        e.set(this, NiParticleSystem::m_spModelData, data);
        let manager = e.mem.alloc(0x200);
        e.mem.set_u32(manager, OTHER_VTABLE);
        e.mem.set_u8(manager + 0x1b0, 1);
        let update_object = object(&mut e, MODIFIER_VTABLE, 1);
        e.set_global(UPDATE_SWITCH_012024D8, 1u8);
        e.set_global(UPDATE_OBJECT_011F5B04, manager);
        e.set_global(UPDATE_THREAD_012024DC, 0x42u32);
        e.set_global(UPDATE_VALUE_011AE6E0, 0x55u32);
        e.register(IMPORT_GET_CURRENT_THREAD_ID, |_, _| 0x42u32.into_ret());
        e.register_double(UPDATE_OBJECT_00C25C40, move |_, _| update_object.into_ret());
        e.register_double(OTHER_SLOTS + 0x8c, move |_, _| manager_answer.into_ret());
        noop(
            &mut e,
            &[
                UPDATE_WITH_00C258E0,
                MODIFIER_SLOTS + 0x94,
                SYSTEM_SLOTS + 0xf4,
            ],
        );
        (e, this, manager, update_object)
    }

    #[test]
    fn update_system_hands_the_system_to_the_update_object() {
        let (mut e, this, manager, update_object) = update_system_rig(0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_ae40, &args![this, 0.5f32]);
        let half = 0.5f32.to_bits();
        assert_eq!(
            calls(&e, UPDATE_WITH_00C258E0),
            vec![vec![update_object, this.addr(), half]]
        );
        assert_eq!(
            calls(&e, OTHER_SLOTS + 0x8c),
            vec![vec![manager, update_object, 0x55]]
        );
        assert_eq!(calls(&e, MODIFIER_SLOTS + 0x94), vec![vec![update_object]]);
        assert_eq!(
            calls(&e, SYSTEM_SLOTS + 0xf4),
            vec![vec![this.addr(), half]]
        );

        // The manager takes it: nothing else runs.
        let (mut e, this, _, _) = update_system_rig(1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_ae40, &args![this, 0.5f32]);
        assert!(calls(&e, MODIFIER_SLOTS + 0x94).is_empty());
        assert!(calls(&e, SYSTEM_SLOTS + 0xf4).is_empty());
    }

    #[test]
    fn update_system_updates_itself_when_a_switch_is_off() {
        // Each condition off in turn: only slot +0xf4 runs (with the time quieted).
        for case in 0..5 {
            let (mut e, this, manager, _) = update_system_rig(1);
            match case {
                0 => e.set_global(UPDATE_SWITCH_012024D8, 0u8),
                1 => e.mem.set_u8(manager + 0x1b0, 0),
                2 => {
                    let data = e.get(this, NiParticleSystem::m_spModelData);
                    e.mem.set_u32(data + 0x1c, 0x7fc0_0000);
                }
                3 => e.set_global(UPDATE_THREAD_012024DC, 0x43u32),
                _ => e.register(UPDATE_OBJECT_00C25C40, |_, _| Ret::default()),
            }
            e.call_log = Some(vec![]);
            e.call(0x00c1_ae40, &args![this, 0x7f80_0001u32]);
            assert!(calls(&e, UPDATE_WITH_00C258E0).is_empty(), "case {case}");
            assert_eq!(
                calls(&e, SYSTEM_SLOTS + 0xf4),
                vec![vec![this.addr(), 0x7fc0_0001]],
                "case {case}"
            );
        }
    }

    #[test]
    fn initialize_new_particle_runs_every_modifier() {
        let mut e = rig();
        noop(&mut e, &[MODIFIER_SLOTS + 0x90]);
        let this = system(&mut e);
        e.set(this, NiParticleSystem::m_spModelData, 0x1234);
        let modifiers = [
            object(&mut e, MODIFIER_VTABLE, 1),
            object(&mut e, MODIFIER_VTABLE, 1),
        ];
        set_modifiers(&mut e, this, &modifiers);
        e.call_log = Some(vec![]);
        e.call(0x00c1_aee0, &args![this, 7u32]);
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x90),
            vec![vec![modifiers[0], 0x1234, 7], vec![modifiers[1], 0x1234, 7]]
        );
    }

    #[test]
    fn selected_downward_pass_follows_flag_bit_2() {
        let mut e = rig();
        noop(
            &mut e,
            &[SYSTEM_SLOTS + 0xb8, UPDATE_SELECTED_DOWNWARD_PASS_00A59FD0],
        );
        let this = system(&mut e);
        let data = e.mem.alloc(8);
        e.mem.set_u8(data + 5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c1_af20, &args![this, data, 3u32]);
        assert_eq!(e.mem.u8(data + 6), 1);
        assert!(calls(&e, UPDATE_SELECTED_DOWNWARD_PASS_00A59FD0).is_empty());
        e.mem.set_u8(data + 5, 0);
        e.call(0x00c1_af20, &args![this, data, 3u32]);
        assert!(calls(&e, SYSTEM_SLOTS + 0xb8).is_empty());
        e.set(this, NiParticleSystem::m_uFlags, 4);
        e.call(0x00c1_af20, &args![this, data, 3u32]);
        assert_eq!(
            calls(&e, SYSTEM_SLOTS + 0xb8),
            vec![vec![this.addr(), data]]
        );
        assert_eq!(
            calls(&e, UPDATE_SELECTED_DOWNWARD_PASS_00A59FD0),
            vec![vec![this.addr(), data, 3]; 2]
        );
    }

    #[test]
    fn update_world_bound_takes_the_data_bound_in_world_space() {
        let mut e = rig();
        noop(&mut e, &[GEOMETRY_UPDATE_WORLD_BOUND]);
        let this = system(&mut e);
        let data = e.mem.alloc(0x20);
        set_floats(&mut e, data + 0x10, &[1.0, 2.0, 3.0, 4.0]);
        e.set(this, NiParticleSystem::m_spModelData, data);
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(this.addr() + 0x20, bound);
        e.mem.set_f32(this.addr() + 0x98, 3.0);
        // Not in world space: only the base runs.
        e.call(0x00c1_af60, &args![this]);
        assert_eq!(floats(&e, bound, 4), vec![0.0; 4]);
        // Dynamic bounds: copied and scaled.
        e.set(this, NiParticleSystem::m_bWorldSpace, true);
        e.set(this, NiParticleSystem::m_bDynamicBounds, true);
        e.call(0x00c1_af60, &args![this]);
        assert_eq!(floats(&e, bound, 4), vec![3.0, 6.0, 9.0, 12.0]);
        // Otherwise moved by the unmodified world transform.
        e.set(this, NiParticleSystem::m_bDynamicBounds, false);
        set_floats(
            &mut e,
            this.addr() + 0xdc,
            &[
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 2.0,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c1_af60, &args![this]);
        assert_eq!(floats(&e, bound, 4), vec![7.0, 4.0, 6.0, 8.0]);
        assert_eq!(
            calls(&e, GEOMETRY_UPDATE_WORLD_BOUND),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn set_selective_update_flags_reads_the_bytes_back() {
        let mut e = rig();
        let this = system(&mut e);
        let selective = e.mem.alloc(4);
        let rigid = e.mem.alloc(4);
        e.mem.set_u8(rigid, 1);
        e.set(this, NiParticleSystem::m_uFlags, 0x10);
        e.call(0x00c1_b020, &args![this, selective, 0u32, rigid]);
        assert_eq!(e.mem.u8(selective), 1);
        assert_eq!(e.mem.u8(rigid), 0);
        assert_eq!(e.get(this, NiParticleSystem::m_uFlags), 0xe);
        // The same byte for both: it reads back 0, so bit 1 is cleared.
        e.set(this, NiParticleSystem::m_uFlags, 0x2);
        e.call(0x00c1_b020, &args![this, selective, 0u32, selective]);
        assert_eq!(e.get(this, NiParticleSystem::m_uFlags), 0xc);
    }

    #[test]
    fn process_clone_runs_the_base_lookup_modifiers_and_data() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                GEOMETRY_PROCESS_CLONE,
                MODIFIER_SLOTS + 0x68,
                DATA_SLOTS + 0x68,
            ],
        );
        e.register(CLONE_LOOKUP_00A53C30, |e, a| {
            // The word behind the address holds the cloning process; it may be overwritten.
            assert_eq!(e.mem.u32(a[2]), 0x0e70_0000);
            e.mem.set_u32(a[2], 0);
            Ret::default()
        });
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.set(this, NiParticleSystem::m_spModelData, data);
        let modifier = object(&mut e, MODIFIER_VTABLE, 1);
        set_modifiers(&mut e, this, &[modifier]);
        e.map(0x0e70_0000, 0x10);
        e.mem.set_u32(0x0e70_0000, 0x0e71_0000);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b060, &args![this, 0x0e70_0000u32]);
        assert_eq!(
            calls(&e, GEOMETRY_PROCESS_CLONE),
            vec![vec![this.addr(), 0x0e70_0000]]
        );
        let lookup = calls(&e, CLONE_LOOKUP_00A53C30);
        assert_eq!(&lookup[0][..2], &[0x0e71_0000, this.addr()]);
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x68),
            vec![vec![modifier, 0x0e70_0000]]
        );
        assert_eq!(calls(&e, DATA_SLOTS + 0x68), vec![vec![data, 0x0e70_0000]]);
    }

    /// A system with data whose word +0x1c the data's slot +0xac sets to `count_after`.
    fn do_update_rig(count_after: u32) -> (Engine, Ptr<NiParticleSystem>, u32) {
        let mut e = rig();
        e.map(0x0102_4000, 0x1000);
        e.set_global(UNSET_TIME_010241B0, -f64::from(f32::MAX));
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.set(this, NiParticleSystem::m_spModelData, data);
        e.register_double(DATA_SLOTS + 0xac, move |e, a| {
            e.mem.set_u32(a[0] + 0x1c, count_after);
            Ret::default()
        });
        noop(
            &mut e,
            &[
                PERFORM_SYSTEM_RESET_00C1B120,
                MARK_AS_CHANGED_00A67090,
                MODIFIER_SLOTS + 0x8c,
            ],
        );
        (e, this, data)
    }

    #[test]
    fn do_update_system_runs_the_modifiers_and_the_kept_one_last() {
        let (mut e, this, data) = do_update_rig(0);
        e.set(this, NiParticleSystem::m_fLastTime, 1.0f32.to_bits());
        // Modifier 0 answers true from +0xa0 (kept), 1 is active, 2 is inactive.
        let modifiers: Vec<u32> = (0..3).map(|_| object(&mut e, MODIFIER_VTABLE, 1)).collect();
        e.mem.set_u8(modifiers[0] + 0x14, 1);
        e.mem.set_u8(modifiers[1] + 0x14, 1);
        let kept = modifiers[0];
        e.register_double(MODIFIER_SLOTS + 0xa0, move |_, a| {
            u32::from(a[0] == kept).into_ret()
        });
        set_modifiers(&mut e, this, &modifiers);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b260, &args![this, 2.0f32]);
        let two = 2.0f32.to_bits();
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x8c),
            vec![vec![modifiers[1], two, data], vec![modifiers[0], two, data]]
        );
        assert!(calls(&e, PERFORM_SYSTEM_RESET_00C1B120).is_empty());
        assert_eq!(calls(&e, MARK_AS_CHANGED_00A67090), vec![vec![data, 7]]);
        assert_eq!(e.get(this, NiParticleSystem::m_fLastTime), two);
    }

    #[test]
    fn do_update_system_resets_and_flags_the_parent() {
        let (mut e, this, _) = do_update_rig(5);
        // The time went back: reset.
        e.set(this, NiParticleSystem::m_fLastTime, 3.0f32.to_bits());
        let parent = object(&mut e, OTHER_VTABLE, 1);
        let grandparent = object(&mut e, OTHER_VTABLE, 1);
        e.mem.set_u32(this.addr() + 0x18, parent);
        e.mem.set_u32(parent + 0x18, grandparent);
        e.register_double(OTHER_SLOTS + 0x10, move |_, a| {
            u32::from(a[0] == grandparent).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c1_b260, &args![this, 2.0f32]);
        assert_eq!(
            calls(&e, PERFORM_SYSTEM_RESET_00C1B120),
            vec![vec![this.addr()]]
        );
        assert_eq!(e.mem.u32(grandparent + 0x30), 0x40_0000);
        assert_eq!(e.mem.u32(parent + 0x30), 0);
        // The reset flag also resets.
        e.set(this, NiParticleSystem::m_bResetSystem, true);
        e.call(0x00c1_b260, &args![this, 2.0f32]);
        assert_eq!(calls(&e, PERFORM_SYSTEM_RESET_00C1B120).len(), 2);
        assert!(!e.get(this, NiParticleSystem::m_bResetSystem));

        // The unset last time (the `double` at 010241b0) becomes the time: no reset, and the
        // data count was already nonzero, so no parent is asked.
        let (mut e, this, data) = do_update_rig(5);
        e.mem.set_u32(data + 0x1c, 1);
        e.set(this, NiParticleSystem::m_fLastTime, (-f32::MAX).to_bits());
        e.mem.set_u32(this.addr() + 0x18, 0x0bad_0000);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b260, &args![this, 2.0f32]);
        assert!(calls(&e, PERFORM_SYSTEM_RESET_00C1B120).is_empty());
        assert_eq!(e.get(this, NiParticleSystem::m_fLastTime), 2.0f32.to_bits());
    }

    #[test]
    fn load_binary_reads_the_world_space_byte() {
        const READ: u32 = 0x0e60_0000;
        let mut e = rig();
        noop(
            &mut e,
            &[LINES_LOAD_BINARY_00A9B180, READ_MULTIPLE_LINK_IDS_00A64670],
        );
        e.register(READ, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], 2);
            1u32.into_ret()
        });
        let this = system(&mut e);
        let stream = e.mem.alloc(0x250);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 10);
        e.mem.set_u32(binary + 8, READ);
        e.mem.set_u32(stream + 0x24c, binary);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b3a0, &args![this, stream]);
        assert!(e.get(this, NiParticleSystem::m_bWorldSpace));
        assert_eq!(e.mem.u32(binary + 4), 11);
        assert_eq!(
            calls(&e, LINES_LOAD_BINARY_00A9B180),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls(&e, READ_MULTIPLE_LINK_IDS_00A64670),
            vec![vec![stream]]
        );
        assert_eq!(calls(&e, READ)[0][0], binary);
    }

    #[test]
    fn constructor_sets_the_vtable_and_the_defaults() {
        let mut e = rig();
        noop(&mut e, &[BASE_CONSTRUCT_00A9B120]);
        e.map(0x0101_5000, 0x1000);
        e.set_global(INITIAL_TIME_01015F5C, 0x7f80_0002u32);
        let this: Ptr<NiParticleSystem> = e.new_object();
        e.mem.write(this.addr(), &[0xcc; 0x110]);
        let made = e.call(0x00c1_b5c0, &args![this]).u32();
        assert_eq!(made, this.addr());
        assert_eq!(e.mem.u32(this.addr()), NI_PARTICLE_SYSTEM_VTABLE_010BD44C);
        assert!(e.get(this, NiParticleSystem::m_bWorldSpace));
        assert_eq!(e.mem.bytes(this.addr() + 0xc8, 0xc), vec![0; 0xc]);
        assert_eq!(e.get(this, NiParticleSystem::m_fLastTime), 0x7fc0_0002);
        assert_eq!(e.mem.u16(this.addr() + 0xd8), 0);
    }

    #[test]
    fn add_modifier_into_an_empty_list_uses_the_head_insert() {
        let mut e = rig();
        noop(&mut e, &[MODIFIER_SLOTS + 0x9c]);
        let item = e.mem.alloc(12);
        e.register_double(LIST_NEW_ITEM_0043A010, move |_, _| item.into_ret());
        let this = system(&mut e);
        let modifier = object(&mut e, MODIFIER_VTABLE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b610, &args![this, modifier]);
        assert_eq!(e.mem.u32(this.addr() + 0xc8), item);
        assert_eq!(e.mem.u32(item + 8), modifier);
        // Held by the list once; the temporary hold was released.
        assert_eq!(e.mem.u32(modifier + 4), 1);
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x9c),
            vec![vec![modifier, this.addr()]]
        );
    }

    #[test]
    fn add_modifier_keeps_the_list_ordered() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                MODIFIER_SLOTS + 0x9c,
                LIST_INSERT_BEFORE_00C1B520,
                LIST_ADD_TAIL_00A5A310,
            ],
        );
        let this = system(&mut e);
        let present: Vec<u32> = (0..2).map(|_| object(&mut e, MODIFIER_VTABLE, 1)).collect();
        e.mem.set_u32(present[0] + 0xc, 2);
        e.mem.set_u32(present[1] + 0xc, 5);
        set_modifiers(&mut e, this, &present);
        let list = this.addr() + 0xc8;
        let second_item = e.mem.u32(e.mem.u32(list));
        // Order 3: before the second item.
        let modifier = object(&mut e, MODIFIER_VTABLE, 1);
        e.mem.set_u32(modifier + 0xc, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b610, &args![this, modifier]);
        let insert = calls(&e, LIST_INSERT_BEFORE_00C1B520);
        assert_eq!(&insert[0][..2], &[list, second_item]);
        assert_eq!(e.mem.u32(modifier + 4), 1);
        // Order 5: not above any (unsigned compare), so at the tail.
        let modifier = object(&mut e, MODIFIER_VTABLE, 1);
        e.mem.set_u32(modifier + 0xc, 5);
        e.call(0x00c1_b610, &args![this, modifier]);
        assert_eq!(calls(&e, LIST_ADD_TAIL_00A5A310)[0][0], list);
        assert_eq!(calls(&e, MODIFIER_SLOTS + 0x9c).len(), 2);
    }

    #[test]
    fn add_modifier_deletes_a_modifier_nobody_kept() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                MODIFIER_SLOTS + 4,
                MODIFIER_SLOTS + 0x9c,
                LIST_ADD_TAIL_00A5A310,
            ],
        );
        let this = system(&mut e);
        let present = object(&mut e, MODIFIER_VTABLE, 1);
        set_modifiers(&mut e, this, &[present]);
        let modifier = object(&mut e, MODIFIER_VTABLE, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b610, &args![this, modifier]);
        assert_eq!(calls(&e, MODIFIER_SLOTS + 4), vec![vec![modifier]]);
    }

    #[test]
    fn copy_members_clones_the_modifiers_and_the_data() {
        let mut e = rig();
        noop(&mut e, &[BASE_COPY_MEMBERS_00A9B140, DATA_SLOTS + 4]);
        let modifier_clone = object(&mut e, MODIFIER_VTABLE, 0);
        let data_clone = object(&mut e, DATA_VTABLE, 0);
        e.register_double(MODIFIER_SLOTS + 0x48, move |_, _| modifier_clone.into_ret());
        e.register_double(DATA_SLOTS + 0x48, move |_, _| data_clone.into_ret());
        e.register(LIST_ADD_TAIL_00A5A310, |e, a| {
            // The list holds the clone it is given.
            let object = e.mem.u32(a[1]);
            let count = e.mem.u32(object + 4) + 1;
            e.mem.set_u32(object + 4, count);
            Ret::default()
        });
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.set(this, NiParticleSystem::m_spModelData, data);
        e.set(this, NiParticleSystem::m_bWorldSpace, true);
        e.set(this, NiParticleSystem::m_bDynamicBounds, true);
        e.set(this, NiParticleSystem::m_fLastTime, 0x7f80_0003);
        let modifier = object(&mut e, MODIFIER_VTABLE, 1);
        set_modifiers(&mut e, this, &[modifier]);
        let clone = system(&mut e);
        let old_data = object(&mut e, DATA_VTABLE, 1);
        e.set(clone, NiParticleSystem::m_spModelData, old_data);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b6f0, &args![this, clone, 0x99u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS_00A9B140),
            vec![vec![this.addr(), clone.addr(), 0x99]]
        );
        assert!(e.get(clone, NiParticleSystem::m_bWorldSpace));
        assert!(e.get(clone, NiParticleSystem::m_bDynamicBounds));
        assert_eq!(e.get(clone, NiParticleSystem::m_fLastTime), 0x7fc0_0003);
        assert_eq!(e.mem.u32(modifier_clone + 4), 1);
        assert_eq!(calls(&e, LIST_ADD_TAIL_00A5A310)[0][0], clone.addr() + 0xc8);
        assert_eq!(e.get(clone, NiParticleSystem::m_spModelData), data_clone);
        assert_eq!(e.mem.u32(data_clone + 4), 1);
        assert_eq!(calls(&e, DATA_SLOTS + 4), vec![vec![old_data]]);
    }

    #[test]
    fn create_function_allocates_and_constructs() {
        let mut e = rig();
        noop(&mut e, &[BASE_CONSTRUCT_00A9B120]);
        e.map(0x0101_5000, 0x1000);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00c1_b7f0, &args![]).u32(), 0);
        let block = e.mem.alloc(0x110);
        e.register_double(NI_ALLOCATE, move |_, a| {
            assert_eq!(a, &[0x110][..]);
            block.into_ret()
        });
        assert_eq!(e.call(0x00c1_b7f0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_PARTICLE_SYSTEM_VTABLE_010BD44C);
    }

    #[test]
    fn link_object_adds_one_modifier_per_link() {
        let mut e = rig();
        noop(
            &mut e,
            &[
                LINES_LINK_OBJECT_00A74780,
                MODIFIER_SLOTS + 0x9c,
                LIST_ADD_TAIL_00A5A310,
            ],
        );
        let this = system(&mut e);
        let present = object(&mut e, MODIFIER_VTABLE, 1);
        set_modifiers(&mut e, this, &[present]);
        let linked = object(&mut e, MODIFIER_VTABLE, 1);
        e.register(GET_NUMBER_OF_LINK_IDS_00A63460, |_, _| 2u32.into_ret());
        e.register_double(GET_OBJECT_FROM_LINK_ID_00A63430, move |_, _| {
            linked.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c1_b810, &args![this, 0x55u32]);
        assert_eq!(
            calls(&e, LINES_LINK_OBJECT_00A74780),
            vec![vec![this.addr(), 0x55]]
        );
        assert_eq!(
            calls(&e, GET_OBJECT_FROM_LINK_ID_00A63430),
            vec![vec![0x55]; 2]
        );
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x9c),
            vec![vec![linked, this.addr()]; 2]
        );
        // No links: nothing added.
        e.register(GET_NUMBER_OF_LINK_IDS_00A63460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c1_b810, &args![this, 0x55u32]);
        assert!(calls(&e, GET_OBJECT_FROM_LINK_ID_00A63430).is_empty());
    }

    #[test]
    fn create_clone_copies_into_a_new_system_or_into_null() {
        let mut e = rig();
        noop(
            &mut e,
            &[BASE_CONSTRUCT_00A9B120, BASE_COPY_MEMBERS_00A9B140],
        );
        e.map(0x0101_5000, 0x1000);
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.set(this, NiParticleSystem::m_spModelData, data);
        e.register_double(DATA_SLOTS + 0x48, move |_, _| data.into_ret());
        let block = e.mem.alloc(0x110);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_b9e0, &args![this, 0x99u32]).u32(), block);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS_00A9B140),
            vec![vec![this.addr(), block, 0x99]]
        );
        assert_eq!(e.mem.u32(block + 0xb8), data);
        // A failed allocation: the copy still runs, on 0 (whose page the test maps).
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS_00A9B140, |_, a| {
            assert_eq!(a[1], 0);
            Ret::default()
        });
        e.map(0, 0x1000);
        assert_eq!(e.call(0x00c1_b9e0, &args![this, 0x99u32]).u32(), 0);
    }

    /// A modifier with the modifier vtable and name word `name` at +8.
    fn named_modifier(e: &mut Engine, name: u32) -> u32 {
        let modifier = object(e, MODIFIER_VTABLE, 1);
        e.mem.set_u32(modifier + 8, name);
        modifier
    }

    #[test]
    fn n063_register_streamables_runs_the_lines_then_each_modifier() {
        let mut e = rig();
        let this = system(&mut e);
        let modifiers = [named_modifier(&mut e, 1), named_modifier(&mut e, 2)];
        set_modifiers(&mut e, this, &modifiers);
        noop(&mut e, &[MODIFIER_SLOTS + 0x54]);
        e.register(LINES_REGISTER_STREAMABLES_00A9B190, |_, _| {
            0x101u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c1_b0b0, &args![this, 0x77u32]).bool());
        assert_eq!(
            calls(&e, MODIFIER_SLOTS + 0x54),
            vec![vec![modifiers[0], 0x77], vec![modifiers[1], 0x77]]
        );
        e.register(LINES_REGISTER_STREAMABLES_00A9B190, |_, _| {
            0x100u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c1_b0b0, &args![this, 0x77u32]).bool());
        assert!(calls(&e, MODIFIER_SLOTS + 0x54).is_empty());
    }

    #[test]
    fn n063_modifier_by_name_skips_empty_items() {
        let mut e = rig();
        let this = system(&mut e);
        let wanted = named_modifier(&mut e, 0x55);
        let other = named_modifier(&mut e, 0x44);
        set_modifiers(&mut e, this, &[0, other, wanted]);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x55);
        assert_eq!(e.call(0x00c1_b0f0, &args![this, name]).u32(), wanted);
        e.mem.set_u32(name, 0x66);
        assert_eq!(e.call(0x00c1_b0f0, &args![this, name]).u32(), 0);
    }

    #[test]
    fn n063_perform_system_reset_resets_data_modifiers_and_matching_controllers() {
        let mut e = rig();
        e.map(0x0101_5000, 0x1000);
        e.set_global(INITIAL_TIME_01015F5C, 0xff7f_ffffu32);
        let this = system(&mut e);
        let data = object(&mut e, DATA_VTABLE, 1);
        e.mem.set_u32(data + 0x78, 0x0005_0006);
        e.set(this, NiParticleSystem::m_spModelData, data);
        let modifier = named_modifier(&mut e, 1);
        set_modifiers(&mut e, this, &[modifier]);
        // Two controllers: the first's RTTI chain reaches the reset record, the second's not.
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, RESET_CONTROLLER_RTTI_012027C8);
        let first = object(&mut e, OTHER_VTABLE, 1);
        let second = object(&mut e, OTHER_VTABLE, 1);
        e.mem.set_u32(this.addr() + 0xc, first);
        e.mem.set_u32(first + 0x30, second);
        e.register_double(OTHER_SLOTS + 8, move |_, a| {
            if a[0] == first {
                base.into_ret()
            } else {
                0x0e70_0000u32.into_ret()
            }
        });
        e.map(0x0e70_0000, 8);
        noop(
            &mut e,
            &[DATA_SLOTS + 0x8c, MODIFIER_SLOTS + 0x94, OTHER_SLOTS + 0xec],
        );
        e.call_log = Some(vec![]);
        e.call(0x00c1_b120, &args![this]);
        assert_eq!(calls(&e, DATA_SLOTS + 0x8c), vec![vec![data, 0]]);
        assert_eq!(e.mem.u32(data + 0x78), 0);
        assert_eq!(e.get(this, NiParticleSystem::m_fLastTime), 0xff7f_ffff);
        assert_eq!(calls(&e, MODIFIER_SLOTS + 0x94), vec![vec![modifier]]);
        assert_eq!(calls(&e, OTHER_SLOTS + 0xec), vec![vec![first]]);
    }

    #[test]
    fn n063_is_equal_compares_flags_and_each_named_modifier() {
        let mut e = rig();
        let this = system(&mut e);
        let other = system(&mut e);
        let mine = named_modifier(&mut e, 7);
        let theirs = named_modifier(&mut e, 7);
        set_modifiers(&mut e, this, &[mine]);
        let unrelated = named_modifier(&mut e, 8);
        set_modifiers(&mut e, other, &[unrelated, theirs]);
        e.register(LINES_IS_EQUAL_00A89770, |_, _| 1u32.into_ret());
        e.register(MODIFIER_SLOTS + 0x5c, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c1_b1d0, &args![this, other]).bool());
        assert_eq!(calls(&e, MODIFIER_SLOTS + 0x5c), vec![vec![mine, theirs]]);
        // A modifier without a match is compared with 0; its answer decides.
        e.mem.set_u32(theirs + 8, 9);
        e.register(MODIFIER_SLOTS + 0x5c, |_, a| {
            u32::from(a[1] != 0).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c1_b1d0, &args![this, other]).bool());
        assert_eq!(calls(&e, MODIFIER_SLOTS + 0x5c), vec![vec![mine, 0]]);
        // Different flags stop before the modifiers.
        e.mem.set_u8(other.addr() + 0xd9, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c1_b1d0, &args![this, other]).bool());
        assert!(calls(&e, MODIFIER_SLOTS + 0x5c).is_empty());
        e.mem.set_u8(other.addr() + 0xd9, 0);
        e.register(LINES_IS_EQUAL_00A89770, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c1_b1d0, &args![this, other]).bool());
    }

    #[test]
    fn n063_save_binary_writes_world_space_count_and_link_ids() {
        let mut e = rig();
        let this = system(&mut e);
        let modifiers = [named_modifier(&mut e, 1), named_modifier(&mut e, 2)];
        set_modifiers(&mut e, this, &modifiers);
        e.mem.set_u8(this.addr() + 0xc4, 1);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream, OTHER_VTABLE);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 4, 100);
        e.mem.set_u32(binary + 0xc, 0x0e60_0000);
        e.mem.set_u32(stream + STREAM_OUTPUT_BINARY_STREAM, binary);
        // The write function records the bytes and answers the count.
        e.register(0x0e60_0000, |e, a| {
            let size = e.mem.u32(a[3]);
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            e.mem.set_u32(0x0e61_0000 + 4 * size, value);
            a[2].into_ret()
        });
        e.map(0x0e61_0000, 0x20);
        noop(&mut e, &[LINES_SAVE_BINARY_00A9B1A0, OTHER_SLOTS + 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x00c1_b400, &args![this, stream]);
        assert_eq!(e.mem.u32(0x0e61_0004), 1);
        assert_eq!(e.mem.u32(0x0e61_0010), 2);
        assert_eq!(e.mem.u32(binary + 4), 105);
        assert_eq!(calls(&e, 0x0e60_0000)[0][4], 1);
        assert_eq!(
            calls(&e, OTHER_SLOTS + 0x30),
            vec![vec![stream, modifiers[0]], vec![stream, modifiers[1]]]
        );
    }

    #[test]
    fn n063_insert_before_links_the_new_item_and_holds_the_element() {
        let mut e = rig();
        let list = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(list, first);
        e.mem.set_u32(list + 4, second);
        e.mem.set_u32(list + 8, 2);
        e.mem.set_u32(first, second);
        e.mem.set_u32(second + 4, first);
        let fresh = e.mem.alloc(0xc);
        let old = object(&mut e, OTHER_VTABLE, 1);
        e.mem.set_u32(fresh + 8, old);
        e.register_double(LIST_NEW_ITEM_0043A010, move |_, _| fresh.into_ret());
        noop(&mut e, &[OTHER_SLOTS + 4]);
        let element = object(&mut e, MODIFIER_VTABLE, 3);
        let pointer = e.mem.alloc(4);
        e.mem.set_u32(pointer, element);
        e.call_log = Some(vec![]);
        // Before the second item: between the two.
        assert_eq!(
            e.call(0x00c1_b520, &args![list, second, pointer]).u32(),
            fresh
        );
        assert_eq!(calls(&e, OTHER_SLOTS + 4), vec![vec![old]]);
        assert_eq!(e.mem.u32(element + 4), 4);
        assert_eq!(e.mem.u32(fresh + 8), element);
        assert_eq!((e.mem.u32(fresh), e.mem.u32(fresh + 4)), (second, first));
        assert_eq!(e.mem.u32(first), fresh);
        assert_eq!(e.mem.u32(second + 4), fresh);
        assert_eq!(e.mem.u32(list + 8), 3);
        // Before the head: the new item becomes the head.
        let head = e.mem.alloc(0xc);
        e.mem.set_u32(head + 8, element);
        e.register_double(LIST_NEW_ITEM_0043A010, move |_, _| head.into_ret());
        assert_eq!(
            e.call(0x00c1_b520, &args![list, first, pointer]).u32(),
            head
        );
        assert_eq!(e.mem.u32(list), head);
        assert_eq!(e.mem.u32(first + 4), head);
        assert_eq!(e.mem.u32(head + 4), 0);
        assert_eq!(e.mem.u32(element + 4), 4);
        assert_eq!(e.mem.u32(list + 8), 4);
    }

    #[test]
    fn n063_get_viewer_strings_adds_five_strings() {
        let mut e = rig();
        e.map(0x0120_2000, 0x1000);
        e.set_global(CLASS_NAME_012024E0, 0x0123_0000u32);
        let this = system(&mut e);
        let data = e.mem.alloc(0x60);
        e.mem.set_u32(data, DATA_VTABLE);
        e.mem.set_u16(data + 0x50, 12);
        e.mem.set_u16(data + 8, 40);
        e.set(this, NiParticleSystem::m_spModelData, data);
        e.mem.set_u8(this.addr() + 0xc4, 1);
        e.mem.set_u32(this.addr() + MODIFIER_COUNT, 3);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 4);
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            Ret::default()
        });
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        noop(&mut e, &[PARTICLES_GET_VIEWER_STRINGS_00A9B2E0]);
        for (address, base) in [
            (NI_GET_VIEWER_STRING, 0x1000u32),
            (VIEWER_STRING_BOOL_00A7D370, 0x2000),
            (VIEWER_STRING_UNSIGNED_00A7D510, 0x3000),
            (VIEWER_STRING_SHORT_00A7D490, 0x4000),
        ] {
            e.register_double(address, move |_, a| (base + a[a.len() - 1]).into_ret());
        }
        e.call_log = Some(vec![]);
        e.call(0x00c1_b850, &args![this, strings]);
        assert_eq!(
            calls(&e, VIEWER_STRING_BOOL_00A7D370),
            vec![vec![WORLD_SPACE_NAME, 1]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_UNSIGNED_00A7D510),
            vec![vec![NUM_MODS_NAME, 3]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_SHORT_00A7D490),
            vec![vec![NUM_PARTICLES_NAME, 12], vec![MAX_PARTICLES_NAME, 40]]
        );
        let set: Vec<u32> = calls(&e, STRINGS_SET_AT).iter().map(|c| c[1]).collect();
        assert_eq!(set, vec![2, 3, 4, 5, 6]);
        // Full at 3, grown to 4 + 3; full again at 7.
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 7]]);
        let slot = calls(&e, STRINGS_SET_AT)[4][2];
        assert_eq!(e.mem.u32(slot), 0x4000 + 40);
    }
}
