//! `BSHavok/bhkentity.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkEntity` is a `bhkWorldObject` (`+8` is the Havok entity, `hkpEntity`; its `+0x210` is
//! the local-frame reference, `+0xCC` the simulation island and `+0xE8` the motion type).
//! The helpers marked `pub(crate)` are shared by the other units translated with it
//! (the viewer-string append, the allocation-scope word and the stream I/O call).
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
pub(crate) const SCOPE_WORD: u32 = 0x2b4;
/// `bhkSerializable::Kill` (`00c876a0`), no stack arguments.
pub(crate) const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
/// `hkReferencedObject::removeReference` (`00c905b0`).
pub(crate) const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `bhkWorldObject` base `LoadBinary` (`00c85f20`), `SaveBinary` (`00c86060`) and
/// `GetViewerStrings` (`00c86190`), one stack argument each.
pub(crate) const WORLD_OBJECT_LOAD_BINARY: u32 = 0x00c8_5f20;
pub(crate) const WORLD_OBJECT_SAVE_BINARY: u32 = 0x00c8_6060;
pub(crate) const WORLD_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00c8_6190;
/// `bhkWorldObject` base destructor body (`00c85790`).
pub(crate) const WORLD_OBJECT_DESTRUCT: u32 = 0x00c8_5790;
/// `bhkWorld::Remove` (`00c68bf0`, `bool`) and the call `UpdateCFilter` makes on the object
/// slot `0x98` answers (`00c946e0`, `(object, entity, 0, 1)`, RET 0xC).
const WORLD_REMOVE: u32 = 0x00c6_8bf0;
const WORLD_UPDATE_ENTITY_FILTER: u32 = 0x00c9_46e0;
/// `hkpEntity::deactivate` (`00c9c240`) and `hkpEntity::isActive` (`00c9bff0`, one out byte).
const ENTITY_DEACTIVATE: u32 = 0x00c9_c240;
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// `NiGetViewerString` (`00a7d330`, cdecl), its boolean form (`00a7d370`, `(name, flag)`),
/// the string form (`00a7d5e0`, `(name, text)`) and the `sprintf`-style formatter
/// (`00406d00`, `(buffer, size, format, value)`), all cdecl.
pub(crate) const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
pub(crate) const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
pub(crate) const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
pub(crate) const FORMAT_TEXT: u32 = 0x0040_6d00;
/// The viewer-string array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
pub(crate) const STRINGS_GROW: u32 = 0x0096_ad30;
pub(crate) const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Counter of live `bhkEntity` objects.
const ENTITY_COUNT: u32 = 0x0126_81fc;
/// `bhkEntity` vtable written by its destructor, and the `NiRTTI` global of the class.
const ENTITY_VTABLE: u32 = 0x0103_02bc;
const ENTITY_RTTI: u32 = 0x0126_8200;
/// Name strings: `Active`, `Fixed`, `SimIsland` and the `0x%08X` format.
const ACTIVE_NAME: u32 = 0x0109_c938;
const FIXED_NAME: u32 = 0x010c_6084;
const SIM_ISLAND_NAME: u32 = 0x010c_6f94;
const POINTER_FORMAT: u32 = 0x010c_54b0;

layout! {
    /// `bhkEntity` (Xbox PDB), 0x14 bytes; the fields this unit uses.
    pub struct BhkEntity: 0x14 {
        /// `phkObject` (Xbox PDB): the Havok entity.
        0x08 phkObject: Ptr,
    }
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
pub(crate) fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Appends the viewer string `text` to the array `strings`: when the 16-bit size (`+0xA`)
/// has reached the capacity (`+0x8`) the array grows by the step (`+0xE`) plus the size
/// first, then the string is stored at the old size.
pub(crate) fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots (the argument is
/// the address of the count, `object + 4`).
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiPointer` release: `InterlockedDecrement(&object->m_uiRefCount)`, then `DeleteThis`
/// (virtual slot 4) when it reached 0.
pub(crate) fn release_ni_object(e: &mut Engine, object: u32) {
    let count = e
        .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .i32();
    if count == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The game heap's allocator (`00aa13e0`, cdecl, one size argument).
pub(crate) const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `bhkSerializable` clone lock (`00c876f0`) and unlock (`00c87700`), no arguments.
pub(crate) const CLONE_LOCK: u32 = 0x00c8_76f0;
pub(crate) const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator whose slot `+4`
/// allocates a size and slot `+8` frees `(pointer, size)`.
pub(crate) const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The game's memory manager (`011f6238`) and its `Allocate` (`00aa3e40`, one size argument).
pub(crate) const MEMORY_MANAGER: u32 = 0x011f_6238;
pub(crate) const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiGetViewerString` for a float (`00a7d550`, cdecl `(name, float)`), `NiPoint3`'s
/// (`00a59af0`, thiscall on the point with the name) and the copy of a Havok vector into a
/// `NiPoint3` (`004a3970`, cdecl `(point, vector)`).
pub(crate) const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
pub(crate) const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
pub(crate) const VECTOR_TO_POINT: u32 = 0x004a_3970;

/// Moves `size` bytes between `buffer` and the stream through the function pointer the
/// stream's reader/writer object keeps at `object + function_offset` (the reader at
/// `stream + 0x24c` with `+8`, the writer at `stream + 0x250` with `+0xC`): a cdecl call of
/// `(object, buffer, size, length, flag)` whose result is added to the object's `+4`
/// (the byte position). `length` is the initial value of the stack word whose address is
/// passed, or `None` for a null pointer.
pub(crate) fn stream_transfer(
    e: &mut Engine,
    object: u32,
    function_offset: u32,
    buffer: u32,
    size: u32,
    length: Option<u32>,
    flag: u32,
) {
    let function = e.mem.u32(object + function_offset);
    let moved = match length {
        Some(initial) => e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), initial);
            e.call(function, &args![object, buffer, size, cell, flag])
                .u32()
        }),
        None => e
            .call(function, &args![object, buffer, size, 0u32, flag])
            .u32(),
    };
    let position = e.mem.u32(object + 4);
    e.mem.set_u32(object + 4, position.wrapping_add(moved));
}

/// Allocates `size` bytes from the current Havok allocator (router `+0x10`, slot `+4`)
/// and stores the size in the 16-bit word at block `+4`.
pub(crate) fn havok_allocate(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// 16-byte aligned allocation from the memory manager: the block is moved up by
/// `0x10 - (address & 0xf)` bytes and that distance is kept in the byte before it.
pub(crate) fn aligned_allocate(e: &mut Engine, size: u32) -> u32 {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, size])
        .u32();
    let distance = 0x10 - (block as u8 & 0xf);
    let aligned = block.wrapping_add(u32::from(distance));
    e.mem.set_u8(aligned.wrapping_sub(1), distance);
    aligned
}

/// Test helpers shared with the units translated with this one.
#[cfg(test)]
pub(crate) mod testing {
    use super::*;

    /// Doubles for the two interlocked imports, working on the count in memory.
    pub(crate) fn interlocked_doubles(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
    }

    /// An object with a one-slot-per-word vtable whose slot 4 is the address `delete_slot`.
    pub(crate) fn ni_object(e: &mut Engine, ref_count: u32, delete_slot: u32) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, ref_count);
        e.mem.set_u32(vtable + 4, delete_slot);
        object
    }

    /// The argument words of every logged call to `addr`.
    pub(crate) fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine that logs calls.
    pub(crate) fn logging_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    /// Doubles for the viewer-string machinery: `NiGetViewerString` answers `0x7000 + name`,
    /// grow does nothing and set-at keeps the text and counts the entry.
    pub(crate) fn viewer_doubles(e: &mut Engine) {
        e.register(NI_GET_VIEWER_STRING, |_, a| Ret {
            eax: 0x7000 + a[0],
            ..Ret::default()
        });
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            // Keeps the text the slot holds at `+0x20 + 4 * index` and counts it.
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x20 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
    }

    /// Where `allocator_doubles` keeps the address of the router object.
    const ROUTER_CELL: u32 = 0x0900_0f00;

    /// Doubles for the Havok allocator route: `MEMORY_ROUTER` answers an object whose
    /// allocator (`+0x10`) has slot 4 allocating a zeroed block and slot 8 doing nothing,
    /// and `HEAP_ALLOCATE`, `MEMORY_MANAGER_ALLOCATE` (a block that is 3 bytes past an
    /// 8-aligned address) and the clone lock/unlock answer likewise.
    pub(crate) fn allocator_doubles(e: &mut Engine) {
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(vtable + 4, 0x0900_0f04);
        e.mem.set_u32(vtable + 8, 0x0900_0f08);
        e.map(ROUTER_CELL, 4);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(ROUTER_CELL),
            ..Ret::default()
        });
        e.register(0x0900_0f04, |e, a| Ret {
            eax: e.mem.alloc(a[1] + 8),
            ..Ret::default()
        });
        e.register(0x0900_0f08, |_, _| Ret::default());
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1] + 0x20) + 3,
            ..Ret::default()
        });
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
    }

    /// A viewer-string array header with the given size, capacity and step.
    pub(crate) fn strings_header(e: &mut Engine, size: u16, capacity: u16, step: u16) -> u32 {
        let strings = e.mem.alloc(0x80);
        e.mem.set_u16(strings + 8, capacity);
        e.mem.set_u16(strings + 0xa, size);
        e.mem.set_u16(strings + 0xe, step);
        strings
    }

    /// The texts `viewer_doubles` kept for entries `from..to` of the array `strings`.
    pub(crate) fn pushed_texts(e: &Engine, strings: u32, from: u32, to: u32) -> Vec<u32> {
        (from..to)
            .map(|i| e.mem.u32(strings + 0x20 + 4 * i))
            .collect()
    }
}

// Translated from 00c9d4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::Kill` (Xbox PDB): when there is a Havok entity, releases its local frame
/// (`+0x210`, through `hkReferencedObject::removeReference` when set) and clears the
/// pointer; then tail-jumps to `bhkSerializable::Kill`, whose `EAX` it returns.
pub fn bhk_entity_kill(e: &mut Engine, this: Ptr<BhkEntity>) -> u32 {
    let entity = e.get(this, BhkEntity::phkObject).addr();
    if entity != 0 {
        let frame = e.mem.u32(entity + 0x210);
        if frame != 0 {
            e.call(REMOVE_REFERENCE, &args![frame]);
        }
        e.mem.set_u32(entity + 0x210, 0);
    }
    e.call(SERIALIZABLE_KILL, &args![this]).u32()
}

// Translated from 00c9d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::LoadBinary` (Xbox PDB): the `bhkWorldObject` load, then, when the stream's
/// byte at `+0x1E5` is not 1, gets the cinfo through virtual slot `0xB4` (`CreateCinfo`;
/// its out argument is the stack word that held the stream) and swaps the bytes of the
/// 16-bit value at `cinfo + 0x1A`.
pub fn bhk_entity_load_binary(e: &mut Engine, this: Ptr<BhkEntity>, stream: u32) {
    e.call(WORLD_OBJECT_LOAD_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let cinfo = e
            .with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), stream);
                e.vcall(this.addr(), 0xb4, &args![slot])
            })
            .u32();
        let value = e.mem.u16(cinfo + 0x1a);
        e.mem.set_u16(cinfo + 0x1a, value.swap_bytes());
    }
}

// Translated from 00c9d530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::SaveBinary` (Xbox PDB): gets the cinfo through virtual slot `0xB4` (its out
/// argument points at an unused stack byte), swaps the bytes of the 16-bit value at
/// `cinfo + 0x1A` when the stream's byte at `+0x1E4` is not 1, runs the `bhkWorldObject`
/// save, and swaps them back under the same test (read again).
pub fn bhk_entity_save_binary(e: &mut Engine, this: Ptr<BhkEntity>, stream: u32) {
    let cinfo = e
        .with_stack(4, |e, slot| {
            e.vcall(this.addr(), 0xb4, &args![slot.addr() + 3])
        })
        .u32();
    if e.mem.u8(stream + 0x1e4) != 1 {
        let value = e.mem.u16(cinfo + 0x1a);
        e.mem.set_u16(cinfo + 0x1a, value.swap_bytes());
    }
    e.call(WORLD_OBJECT_SAVE_BINARY, &args![this, stream]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let value = e.mem.u16(cinfo + 0x1a);
        e.mem.set_u16(cinfo + 0x1a, value.swap_bytes());
    }
}

// Translated from 00c9d590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::Remove` (Xbox PDB): asks virtual slot `0x94` for an object and, when it
/// exists and its word at `+0x350` is set, calls `00c68bf0` on that word with the Havok
/// entity (`+8`) and answers its result; otherwise answers `false`.
pub fn bhk_entity_remove(e: &mut Engine, this: Ptr<BhkEntity>) -> bool {
    let world_object = e.vcall(this.addr(), 0x94, &args![]).u32();
    let world = if world_object != 0 {
        e.mem.u32(world_object + 0x350)
    } else {
        0
    };
    if world != 0 {
        let entity = e.get(this, BhkEntity::phkObject);
        e.call(WORLD_REMOVE, &args![world, entity]).bool()
    } else {
        false
    }
}

// Translated from 00c9d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntity::UpdateCFilter` (Xbox PDB): asks virtual slot `0x98` for an object and, when
/// there is one, calls `00c946e0` on it with the Havok entity (`+8`), `0` and `1`.
pub fn bhk_entity_update_c_filter(e: &mut Engine, this: Ptr<BhkEntity>) {
    let world = e.vcall(this.addr(), 0x98, &args![]).u32();
    if world != 0 {
        let entity = e.get(this, BhkEntity::phkObject);
        e.call(
            WORLD_UPDATE_ENTITY_FILTER,
            &args![world, entity, 0u32, 1u32],
        );
    }
}

// Translated from 00c9d600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tail-jump helper of `bhkEntity` (no PDB name): when `this` and its Havok entity (`+8`)
/// exist, jumps to `hkpEntity::deactivate` (`00c9c240`) with the entity.
pub fn fn_00c9d600(e: &mut Engine, this: Ptr<BhkEntity>) {
    if this.addr() != 0 {
        let entity = e.get(this, BhkEntity::phkObject);
        if entity.addr() != 0 {
            e.call(ENTITY_DEACTIVATE, &args![entity]);
        }
    }
}

// Translated from 00c9d620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9d620 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkEntity::~bhkEntity` (Xbox PDB): installs the `bhkEntity` vtable, releases the Havok
/// entity's local frame like `Kill` does, runs `bhkSerializable::Kill`, decrements the live
/// entity counter and runs the `bhkWorldObject` destructor body.
pub fn bhk_entity_destruct(e: &mut Engine, this: Ptr<BhkEntity>) {
    e.mem.set_u32(this.addr(), ENTITY_VTABLE);
    let entity = e.get(this, BhkEntity::phkObject).addr();
    if entity != 0 {
        let frame = e.mem.u32(entity + 0x210);
        if frame != 0 {
            e.call(REMOVE_REFERENCE, &args![frame]);
        }
        e.mem.set_u32(entity + 0x210, 0);
    }
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(ENTITY_COUNT);
    e.mem.set_u32(ENTITY_COUNT, count.wrapping_sub(1));
    e.call(WORLD_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00c9d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9d6b0 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkEntity::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10, runs
/// the `bhkWorldObject` strings, then appends the class name, `Active` (the out byte of
/// `hkpEntity::isActive`, false without an entity), `Fixed` (the entity's byte at `+0xE8`
/// equal to 5) and `SimIsland` (the entity's island pointer at `+0xCC`, formatted
/// `0x%08X` into a 0x40-byte buffer).
pub fn bhk_entity_get_viewer_strings(e: &mut Engine, this: Ptr<BhkEntity>, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(WORLD_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(ENTITY_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);

    let entity = if this.addr() != 0 {
        e.get(this, BhkEntity::phkObject).addr()
    } else {
        0
    };
    let active = if entity != 0 {
        e.with_stack(4, |e, out| {
            e.call(ENTITY_IS_ACTIVE, &args![entity, out]);
            e.mem.u8(out.addr()) != 0
        })
    } else {
        false
    };
    let text = e
        .call(NI_GET_VIEWER_STRING_BOOL, &args![ACTIVE_NAME, active])
        .u32();
    push_viewer_string(e, strings, text);

    let fixed = entity != 0 && e.mem.u8(entity + 0xe8) == 5;
    let text = e
        .call(NI_GET_VIEWER_STRING_BOOL, &args![FIXED_NAME, fixed])
        .u32();
    push_viewer_string(e, strings, text);

    let island = if entity != 0 {
        e.mem.u32(entity + 0xcc)
    } else {
        0
    };
    let buffer = e.with_stack(0x40, |e, buffer| {
        e.call(FORMAT_TEXT, &args![buffer, 0x40u32, POINTER_FORMAT, island]);
        e.call(NI_GET_VIEWER_STRING_TEXT, &args![SIM_ISLAND_NAME, buffer])
            .u32()
    });
    push_viewer_string(e, strings, buffer);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_d4c0, bhk_entity_kill(Ptr<BhkEntity>) -> u32),
        entry!(0x00c9_d4f0, bhk_entity_load_binary(Ptr<BhkEntity>, u32)),
        entry!(0x00c9_d530, bhk_entity_save_binary(Ptr<BhkEntity>, u32)),
        entry!(0x00c9_d590, bhk_entity_remove(Ptr<BhkEntity>) -> bool),
        entry!(0x00c9_d5d0, bhk_entity_update_c_filter(Ptr<BhkEntity>)),
        entry!(0x00c9_d600, fn_00c9d600(Ptr<BhkEntity>)),
        entry!(0x00c9_d620, bhk_entity_destruct(Ptr<BhkEntity>)),
        entry!(
            0x00c9_d6b0,
            bhk_entity_get_viewer_strings(Ptr<BhkEntity>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::testing::*;
    use super::*;

    #[test]
    fn kill_releases_the_frame_kept_here_and_clears_it() {
        let mut e = logging_engine();
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(SERIALIZABLE_KILL, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(this + 8, entity);
        e.mem.set_u32(entity + 0x210, 0x6000);
        assert_eq!(e.call(0x00c9_d4c0, &args![this]).u32(), 0x1234);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(entity + 0x210), 0);
        // No local frame: no release. No Havok entity: nothing touched.
        e.call(0x00c9_d4c0, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 1);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c9_d4c0, &args![this]);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL).len(), 3);
    }

    #[test]
    fn load_binary_swaps_the_cinfo_value_unless_the_stream_byte_is_one() {
        let mut e = logging_engine();
        e.register(WORLD_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0x40);
        e.register(0x0900_00b4, |e, a| {
            // The out argument is a stack word that holds the stream.
            assert_eq!(e.mem.u32(a[1]), 0x0900_3000);
            Ret {
                eax: e.mem.u32(0x0900_1000),
                ..Ret::default()
            }
        });
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, cinfo);
        let this = e.mem.alloc(0x14);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        let stream = 0x0900_3000;
        e.map(stream, 0x200);
        e.mem.set_u16(cinfo + 0x1a, 0x1234);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00c9_d4f0, &args![this, stream]);
        assert_eq!(e.mem.u16(cinfo + 0x1a), 0x1234);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00c9_d4f0, &args![this, stream]);
        assert_eq!(e.mem.u16(cinfo + 0x1a), 0x3412);
        assert_eq!(
            calls_to(&e, WORLD_OBJECT_LOAD_BINARY),
            vec![vec![this, stream]; 2]
        );
    }

    #[test]
    fn save_binary_swaps_around_the_world_object_save() {
        let mut e = logging_engine();
        let cinfo = e.mem.alloc(0x40);
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, cinfo);
        e.register(0x0900_00b4, |e, _| Ret {
            eax: e.mem.u32(0x0900_1000),
            ..Ret::default()
        });
        e.register(WORLD_OBJECT_SAVE_BINARY, |e, _| {
            // Inside the save the value is in the swapped order.
            let cinfo = e.mem.u32(0x0900_1000);
            let seen = e.mem.u16(cinfo + 0x1a);
            e.mem.set_u16(cinfo + 0x30, seen);
            Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0xb4, 0x0900_00b4);
        let stream = e.mem.alloc(0x200);
        e.mem.set_u16(cinfo + 0x1a, 0x1234);
        e.call(0x00c9_d530, &args![this, stream]);
        assert_eq!(e.mem.u16(cinfo + 0x30), 0x3412);
        assert_eq!(e.mem.u16(cinfo + 0x1a), 0x1234);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call(0x00c9_d530, &args![this, stream]);
        assert_eq!(e.mem.u16(cinfo + 0x30), 0x1234);
        assert_eq!(e.mem.u16(cinfo + 0x1a), 0x1234);
    }

    /// An object whose vtable slots `0x94` and `0x98` answer `world_object` / `world`.
    fn object_with_worlds(e: &mut Engine, world_object: u32, world: u32) -> u32 {
        e.map(0x0900_2000, 8);
        e.mem.set_u32(0x0900_2000, world_object);
        e.mem.set_u32(0x0900_2004, world);
        e.register(0x0900_0094, |e, _| Ret {
            eax: e.mem.u32(0x0900_2000),
            ..Ret::default()
        });
        e.register(0x0900_0098, |e, _| Ret {
            eax: e.mem.u32(0x0900_2004),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x94, 0x0900_0094);
        e.mem.set_u32(vtable + 0x98, 0x0900_0098);
        e.mem.set_u32(this + 8, 0x5555);
        this
    }

    #[test]
    fn remove_asks_the_world_the_object_lives_in() {
        let mut e = logging_engine();
        e.register(WORLD_REMOVE, |_, _| Ret {
            eax: 0xffff_ff01,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x360);
        e.mem.set_u32(holder + 0x350, 0x7777);
        let this = object_with_worlds(&mut e, holder, 0);
        assert!(e.call(0x00c9_d590, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE), vec![vec![0x7777, 0x5555]]);
        e.mem.set_u32(holder + 0x350, 0);
        assert!(!e.call(0x00c9_d590, &args![this]).bool());
        e.mem.set_u32(0x0900_2000, 0);
        assert!(!e.call(0x00c9_d590, &args![this]).bool());
        assert_eq!(calls_to(&e, WORLD_REMOVE).len(), 1);
    }

    #[test]
    fn update_c_filter_passes_the_entity_zero_and_one() {
        let mut e = logging_engine();
        e.register(WORLD_UPDATE_ENTITY_FILTER, |_, _| Ret::default());
        let this = object_with_worlds(&mut e, 0, 0x8888);
        e.call(0x00c9_d5d0, &args![this]);
        assert_eq!(
            calls_to(&e, WORLD_UPDATE_ENTITY_FILTER),
            vec![vec![0x8888, 0x5555, 0, 1]]
        );
        e.mem.set_u32(0x0900_2004, 0);
        e.call(0x00c9_d5d0, &args![this]);
        assert_eq!(calls_to(&e, WORLD_UPDATE_ENTITY_FILTER).len(), 1);
    }

    #[test]
    fn deactivate_helper_needs_the_object_and_its_entity() {
        let mut e = logging_engine();
        e.register(ENTITY_DEACTIVATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        e.call(0x00c9_d600, &args![this]);
        assert!(calls_to(&e, ENTITY_DEACTIVATE).is_empty());
        e.mem.set_u32(this + 8, 0x5555);
        e.call(0x00c9_d600, &args![this]);
        e.call(0x00c9_d600, &args![0u32]);
        assert_eq!(calls_to(&e, ENTITY_DEACTIVATE), vec![vec![0x5555]]);
    }

    #[test]
    fn destructor_counts_down_and_runs_the_base_destructor() {
        let mut e = logging_engine();
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(SERIALIZABLE_KILL, |_, _| Ret::default());
        e.register(WORLD_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.map(ENTITY_COUNT, 4);
        e.mem.set_u32(ENTITY_COUNT, 5);
        let this = e.mem.alloc(0x14);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(this + 8, entity);
        e.mem.set_u32(entity + 0x210, 0x6000);
        e.call(0x00c9_d620, &args![this]);
        assert_eq!(e.mem.u32(this), ENTITY_VTABLE);
        assert_eq!(e.mem.u32(ENTITY_COUNT), 4);
        assert_eq!(e.mem.u32(entity + 0x210), 0);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x6000]]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn viewer_strings_list_class_active_fixed_and_island() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        e.register(WORLD_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| Ret {
            eax: 0x8000 + a[1],
            ..Ret::default()
        });
        e.register(ENTITY_IS_ACTIVE, |e, a| {
            e.mem.set_u8(a[1], 2);
            Ret::default()
        });
        e.register(FORMAT_TEXT, |e, a| {
            e.mem.set_cstr(a[0], b"0xFEEDBEEF");
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING_TEXT, |e, a| Ret {
            eax: a[1] + e.mem.u8(a[1]) as u32,
            ..Ret::default()
        });
        e.map(ENTITY_RTTI, 4);
        e.mem.set_u32(ENTITY_RTTI, 0x300);
        let this = e.mem.alloc(0x14);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(this + 8, entity);
        e.mem.set_u8(entity + 0xe8, 5);
        e.mem.set_u32(entity + 0xcc, 0xfeed_beef);
        let strings = strings_header(&mut e, 1, 4, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 7);
        e.call(0x00c9_d6b0, &args![this, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 7);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_BOOL),
            vec![vec![ACTIVE_NAME, 1], vec![FIXED_NAME, 1]]
        );
        let format = calls_to(&e, FORMAT_TEXT);
        assert_eq!(format.len(), 1);
        assert_eq!(format[0][1..], [0x40, POINTER_FORMAT, 0xfeed_beef]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 4);
        // Without an entity every flag is false and the island is 0.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c9_d6b0, &args![this, strings]);
        let flags = calls_to(&e, NI_GET_VIEWER_STRING_BOOL);
        assert_eq!(flags[2..], [[ACTIVE_NAME, 0], [FIXED_NAME, 0]]);
        assert_eq!(calls_to(&e, FORMAT_TEXT)[1][3], 0);
    }

    #[test]
    fn viewer_string_array_grows_when_full() {
        let mut e = logging_engine();
        viewer_doubles(&mut e);
        let strings = strings_header(&mut e, 4, 4, 3);
        push_viewer_string(&mut e, strings, 0x99);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 7]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 4]);
    }
}
