//! `BSHavok/bhkextradata.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkExtraData` is a 0x24-byte `NiExtraData` that embeds an array of references at `+0xC`
//! (vtable, then data pointer `+0x10`, capacity `+0x14`, used size `+0x18`, number of
//! non-null entries `+0x1C`, growth step `+0x20`). The array routines here take the array
//! itself (`this + 0xC`) as their `this`, with the fields at `+4`, `+8`, `+0xC`, `+0x10`
//! and `+0x14`.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{
    enter_scope, push_viewer_string, release_ni_object, stream_transfer, INTERLOCKED_DECREMENT,
    INTERLOCKED_INCREMENT, NI_GET_VIEWER_STRING,
};

/// The game heap's raw allocate (`00aa1070`, cdecl) and free (`00aa10f0`, cdecl) and the
/// memory manager's `Free` (`00aa4060`, thiscall on `011f6238`).
const RAW_ALLOCATE: u32 = 0x00aa_1070;
const RAW_FREE: u32 = 0x00aa_10f0;
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `NiExtraData` base methods this unit calls: constructor (`00a7b2e0`), destructor
/// (`00a7b300`), `LoadBinary` (`00a7b4a0`), `RegisterStreamables` (`00a7b290`),
/// `SaveBinary` (`00a7b2c0`), `GetViewerStrings` (`00a7b580`), `LinkObject` (`00a7b3b0`)
/// and `CopyMembers` (`00a7b330`, `(this, clone, context)`), plus the tail target of
/// `ProcessClone` (`00a5d430`).
const BASE_CONSTRUCT: u32 = 0x00a7_b2e0;
const BASE_DESTRUCT: u32 = 0x00a7_b300;
const BASE_LOAD_BINARY: u32 = 0x00a7_b4a0;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
const BASE_SAVE_BINARY: u32 = 0x00a7_b2c0;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
const BASE_LINK_OBJECT: u32 = 0x00a7_b3b0;
const BASE_COPY_MEMBERS: u32 = 0x00a7_b330;
const PROCESS_CLONE_TARGET: u32 = 0x00a5_d430;
/// `NiStream` helpers: the call `LoadBinary` makes on the stream (`00a64670`), the number of
/// link ids (`00a63460`) and the next object from a link id (`00a63430`).
const STREAM_AFTER_LOAD: u32 = 0x00a6_4670;
const STREAM_LINK_ID_COUNT: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The array's `SetAt` (`00cc1a10`, `(array, index, address of the reference)`).
const ARRAY_SET_AT: u32 = 0x00cc_1a10;
/// This unit's `SetSize` (`00cc1b40`), `RemoveAll` (`00cc1cf0`), constructor (`00cc1d80`),
/// `AddObject` (`00cc2080`) and clone-copy (`00cc2120`).
const ARRAY_SET_SIZE: u32 = 0x00cc_1b40;
const ARRAY_REMOVE_ALL: u32 = 0x00cc_1cf0;
const CONSTRUCT: u32 = 0x00cc_1d80;
const ADD_OBJECT: u32 = 0x00cc_2080;
const COPY_INTO_CLONE: u32 = 0x00cc_2120;
/// Fixed-string interning (`00a5b690`, cdecl, one argument) and `NiExtraData::SetName`
/// (`00a5b950`, `(this, address of the handle)`) as used by the constructor, and the
/// factory's name argument for it (`011f49e0`).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
const SET_NAME: u32 = 0x00a5_b950;
const FACTORY_NAME: u32 = 0x011f_49e0;
/// The empty fixed-string handle (`0109b220` holds it).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// Vtables the constructor installs (object and array), the array vtable the destructor
/// installs, the object's name string `HKE` and the class `NiRTTI` global.
const OBJECT_VTABLE: u32 = 0x010c_94a4;
const ARRAY_VTABLE: u32 = 0x010c_9498;
const ARRAY_DESTRUCTOR_VTABLE: u32 = 0x010c_9490;
const NAME_STRING: u32 = 0x010c_9488;
const EXTRA_DATA_RTTI: u32 = 0x0126_8598;
/// Size of a `bhkExtraData`.
const OBJECT_SIZE: u32 = 0x24;

// Translated from 00c9ae80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the array (the engine map lists it as
/// `NiTLargeArray<BLENDKEY>::_scalar_deleting_destructor_`, a folded copy of identical
/// code): installs the vtable `010c6b04`, frees the data block at `+4` (`00aa10f0`) and,
/// when bit 0 of `flags` is set, frees the object itself through the memory manager.
/// Answers `this`.
pub fn fn_00c9ae80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let data = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), 0x010c_6b04);
    e.call(RAW_FREE, &args![data]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00cc1960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::ProcessClone` (Xbox PDB): a tail jump to `00a5d430` with the same
/// argument; answers its `EAX`.
pub fn bhk_extra_data_process_clone(e: &mut Engine, this: Ptr, clone: u32) -> u32 {
    e.call(PROCESS_CLONE_TARGET, &args![this, clone]).u32()
}

// Translated from 00cc1970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::LoadBinary` (Xbox PDB): the `NiExtraData` load, then `00a64670` on the
/// stream.
pub fn bhk_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_AFTER_LOAD, &args![stream]);
}

// Translated from 00cc1990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::RegisterStreamables` (Xbox PDB): runs the `NiExtraData` registration, then
/// registers every non-null entry among the first `+0x1C` slots of the array data (`+0x10`)
/// through its virtual slot `0x54`. Answers the base registration's `AL`.
pub fn bhk_extra_data_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    let registered = e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool();
    let count = e.mem.u32(this.addr() + 0x1c);
    let mut index = 0;
    while index < count {
        let data = e.mem.u32(this.addr() + 0x10);
        let object = e.mem.u32(data.wrapping_add(index * 4));
        if object != 0 {
            e.vcall(object, 0x54, &args![stream]);
        }
        index += 1;
    }
    registered
}

// Translated from 00cc1ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::SaveBinary` (Xbox PDB): the `NiExtraData` save, then the entry count
/// (`+0x1C`) written as 4 bytes through the stream's writer (`stream + 0x250`, function at
/// `+0xC`, length cell initialised to 4, flag 1), then each non-null entry among the
/// first `count` of the array data written through stream virtual slot `0x30`.
pub fn bhk_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let count = e.mem.u32(this.addr() + 0x1c);
    let writer = e.mem.u32(stream + 0x250);
    e.with_stack(4, |e, buffer| {
        e.mem.set_u32(buffer.addr(), count);
        stream_transfer(e, writer, 0xc, buffer.addr(), 4, Some(4), 1);
    });
    let mut index = 0;
    while index < count {
        let data = e.mem.u32(this.addr() + 0x10);
        let object = e.mem.u32(data.wrapping_add(index * 4));
        if object != 0 {
            e.vcall(stream, 0x30, &args![object]);
        }
        index += 1;
    }
}

// Translated from 00cc1b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1b40 - the FS:[0] exception-unwinding frame is not modeled.
/// `SetSize` of the array of references (no PDB name; `this` is the array): nothing when
/// `new_size` equals the capacity (`+8`). When it is below the used size (`+0xC`) the
/// entries from `new_size` on are released and cleared (the non-null count at `+0x10`
/// drops by one for each) and the used size becomes `new_size`. Then the capacity is set,
/// a block of `4 * new_size` bytes is allocated (`00aa1070`; none when `new_size` is 0),
/// the used entries are copied over with a reference added to each one that differs from
/// what the new block already holds (the exe reads the fresh block there, so the Rust
/// model sees zeros), the new block's entries from the used size to the capacity are
/// released and cleared when non-null, and the old block is freed (`00aa10f0`).
pub fn fn_00cc1b40(e: &mut Engine, this: Ptr, new_size: u32) {
    let array = this.addr();
    if new_size == e.mem.u32(array + 8) {
        return;
    }
    if new_size < e.mem.u32(array + 0xc) {
        let mut index = new_size;
        loop {
            let slot = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4));
            let object = e.mem.u32(slot);
            if object != 0 {
                release_ni_object(e, object);
                e.mem.set_u32(slot, 0);
                let count = e.mem.u32(array + 0x10);
                e.mem.set_u32(array + 0x10, count.wrapping_add(0xffff_ffff));
            }
            index += 1;
            if index >= e.mem.u32(array + 0xc) {
                break;
            }
        }
        e.mem.set_u32(array + 0xc, new_size);
    }
    let old_block = e.mem.u32(array + 4);
    e.mem.set_u32(array + 8, new_size);
    if new_size > 0 {
        let block = e.call(RAW_ALLOCATE, &args![new_size.wrapping_mul(4)]).u32();
        e.mem.set_u32(array + 4, block);
        if e.mem.u32(array + 0xc) > 0 {
            let mut index = 0u32;
            loop {
                let new_block = e.mem.u32(array + 4);
                let offset = index.wrapping_mul(4);
                let existing = e.mem.u32(new_block.wrapping_add(offset));
                let wanted = e.mem.u32(old_block.wrapping_add(offset));
                if existing != wanted {
                    if existing != 0 {
                        release_ni_object(e, existing);
                    }
                    e.mem.set_u32(new_block.wrapping_add(offset), wanted);
                    if wanted != 0 {
                        e.call(INTERLOCKED_INCREMENT, &args![wanted.wrapping_add(4)]);
                    }
                }
                index += 1;
                if index >= e.mem.u32(array + 0xc) {
                    break;
                }
            }
        }
        let mut index = e.mem.u32(array + 0xc);
        if index < e.mem.u32(array + 8) {
            loop {
                let slot = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4));
                let object = e.mem.u32(slot);
                if object != 0 {
                    release_ni_object(e, object);
                    e.mem.set_u32(slot, 0);
                }
                index += 1;
                if index >= e.mem.u32(array + 8) {
                    break;
                }
            }
        }
    } else {
        e.mem.set_u32(array + 4, 0);
    }
    e.call(RAW_FREE, &args![old_block]);
}

// Translated from 00cc1cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1cf0 - the FS:[0] exception-unwinding frame is not modeled.
/// `RemoveAll` of the array of references (no PDB name; `this` is the array): releases and
/// clears every non-null entry among the first `+0xC` slots, then zeroes the non-null count
/// (`+0x10`) and the used size (`+0xC`).
pub fn fn_00cc1cf0(e: &mut Engine, this: Ptr) {
    let array = this.addr();
    let mut index = 0u32;
    if e.mem.u32(array + 0xc) > 0 {
        loop {
            let slot = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4));
            let object = e.mem.u32(slot);
            if object != 0 {
                release_ni_object(e, object);
                e.mem.set_u32(slot, 0);
            }
            index += 1;
            if index >= e.mem.u32(array + 0xc) {
                break;
            }
        }
    }
    e.mem.set_u32(array + 0x10, 0);
    e.mem.set_u32(array + 0xc, 0);
}

// Translated from 00cc1d80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1d80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData` constructor (no PDB name): runs the `NiExtraData` constructor, installs the
/// vtable, zeroes the array fields (`+0x10`, `+0x14`, `+0x18`, `+0x1C`), sets the growth step
/// (`+0x20`) to 1 and the array vtable (`+0xC`), interns the name `HKE` (`00a5b690`), gives it
/// to `SetName` (`00a5b950`) and drops the handle again (a decrement of `handle - 8`
/// unless it is the empty string's). Answers `this`.
pub fn fn_00cc1d80(e: &mut Engine, this: Ptr) -> Ptr {
    let object = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(object, OBJECT_VTABLE);
    e.mem.set_u32(object + 0x14, 0);
    e.mem.set_u32(object + 0x20, 1);
    e.mem.set_u32(object + 0x18, 0);
    e.mem.set_u32(object + 0x1c, 0);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u32(object + 0xc, ARRAY_VTABLE);
    let handle = e.call(FIXED_STRING_CREATE, &args![NAME_STRING]).u32();
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), handle);
        e.call(SET_NAME, &args![this, cell]);
        let handle = e.mem.u32(cell.addr());
        if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    this
}

// Translated from 00cc1e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1e40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData` destructor body (no PDB name): installs the object vtable, empties the
/// array (`00cc1cf0`) with the allocation-scope word at 0x10, installs the array's
/// destructor vtable, frees its data block (`00aa10f0`) and runs the `NiExtraData`
/// destructor.
pub fn fn_00cc1e40(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, OBJECT_VTABLE);
    let (word, saved) = enter_scope(e);
    e.call(ARRAY_REMOVE_ALL, &args![object + 0xc]);
    e.mem.set_u32(word, saved);
    e.mem.set_u32(object + 0xc, ARRAY_DESTRUCTOR_VTABLE);
    let data = e.mem.u32(object + 0x10);
    e.call(RAW_FREE, &args![data]);
    e.call(BASE_DESTRUCT, &args![this]);
}

// Translated from 00cc1ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1ef0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData` factory (no PDB name): allocates 0x24 bytes from the game heap, constructs
/// the object in them when that worked (`00cc1d80`), then calls `SetName` (`00a5b950`) on the
/// result with the word `011f49e0` (also when the result is null, as the exe does).
/// Answers the object or 0 (takes no arguments).
pub fn fn_00cc1ef0(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![OBJECT_SIZE]).u32();
    let object = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(SET_NAME, &args![object, FACTORY_NAME]);
    object
}

// Translated from 00cc1f80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc1f80 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData::GetViewerStrings` (Xbox PDB): with the allocation-scope word at 0x10, runs
/// the `NiExtraData` strings, appends the class name, then calls virtual slot `0x60` with
/// the strings on every non-null entry among the first `+0x1C` slots of the array data.
pub fn bhk_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(EXTRA_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let count = e.mem.u32(this.addr() + 0x1c);
    let mut index = 0;
    while index < count {
        let data = e.mem.u32(this.addr() + 0x10);
        let object = e.mem.u32(data.wrapping_add(index * 4));
        if object != 0 {
            e.vcall(object, 0x60, &args![strings]);
        }
        index += 1;
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cc2080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2080 - the FS:[0] exception-unwinding frame is not modeled.
/// Adds an object to the array (no PDB name): for a non-null `object`, takes a reference,
/// grows the array (`00cc1b40`, to the growth step `+0x20` plus the used size) when the used
/// size (`+0x18`) has reached the capacity (`+0x14`), stores the object at the used size
/// through `SetAt` (`00cc1a10`, with the address of a cell holding it) and drops the
/// temporary reference again.
pub fn fn_00cc2080(e: &mut Engine, this: Ptr, object: u32) {
    if object == 0 {
        return;
    }
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    let array = this.addr() + 0xc;
    let used = e.mem.u32(array + 0xc);
    if used >= e.mem.u32(array + 8) {
        let growth = e.mem.u32(array + 0x14);
        e.call(ARRAY_SET_SIZE, &args![array, growth.wrapping_add(used)]);
    }
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), object);
        e.call(ARRAY_SET_AT, &args![array, used, cell]);
    });
    release_ni_object(e, object);
}

// Translated from 00cc2120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc2120 - the FS:[0] exception-unwinding frame is not modeled.
/// Copies `this` into `clone` (no PDB name): with the allocation-scope word at 0x10, runs the
/// `NiExtraData` copy of `(this, clone, context)`, sizes the clone's array to the non-null
/// count (`+0x1C`) and adds to it, through `00cc2080`, the clone (virtual slot `0x48` with
/// `context`) of each non-null entry among the first `count` slots (a null entry is added
/// as null).
pub fn fn_00cc2120(e: &mut Engine, this: Ptr, clone: u32, context: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BASE_COPY_MEMBERS, &args![this, clone, context]);
    let count = e.mem.u32(this.addr() + 0x1c);
    e.call(ARRAY_SET_SIZE, &args![clone + 0xc, count]);
    let mut index = 0;
    while index < count {
        let data = e.mem.u32(this.addr() + 0x10);
        let object = e.mem.u32(data.wrapping_add(index * 4));
        let copy = if object != 0 {
            e.vcall(object, 0x48, &args![context]).u32()
        } else {
            0
        };
        e.call(ADD_OBJECT, &args![clone, copy]);
        index += 1;
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cc21f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc21f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData::LinkObject` (Xbox PDB): with the allocation-scope word at 0x10, runs the
/// `NiExtraData` link, then for each of the stream's link ids (`00a63460`) fetches the
/// linked object (`00a63430`) and adds it to the array (`00cc2080`) when it exists.
pub fn bhk_extra_data_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let (word, saved) = enter_scope(e);
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let mut remaining = e.call(STREAM_LINK_ID_COUNT, &args![stream]).u32();
    if remaining > 0 {
        loop {
            let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            if object != 0 {
                e.call(ADD_OBJECT, &args![this, object]);
            }
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00cc22a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc22a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkExtraData::CreateClone` (Xbox PDB): with the allocation-scope word at 0x10, allocates
/// 0x24 bytes from the game heap, constructs an object in them when that worked
/// (`00cc1d80`), runs `00cc2120` to copy `this` into it with `context` (also when the
/// object is null) and answers the object.
pub fn bhk_extra_data_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let block = e.call(HEAP_ALLOCATE, &args![OBJECT_SIZE]).u32();
    let clone = if block != 0 {
        e.call(CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(COPY_INTO_CLONE, &args![this, clone, context]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cc1f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc1e40`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x24 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_extra_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_1e40;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x24u32]);
    }
    this
}

// Translated from 00cc1e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkExtraData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8598.
pub fn bhk_extra_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8598
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_ae80, fn_00c9ae80(Ptr, u32) -> Ptr),
        entry!(0x00cc_1960, bhk_extra_data_process_clone(Ptr, u32) -> u32),
        entry!(0x00cc_1970, bhk_extra_data_load_binary(Ptr, u32)),
        entry!(
            0x00cc_1990,
            bhk_extra_data_register_streamables(Ptr, u32) -> bool
        ),
        entry!(0x00cc_1ad0, bhk_extra_data_save_binary(Ptr, u32)),
        entry!(0x00cc_1b40, fn_00cc1b40(Ptr, u32)),
        entry!(0x00cc_1cf0, fn_00cc1cf0(Ptr)),
        entry!(0x00cc_1d80, fn_00cc1d80(Ptr) -> Ptr),
        entry!(0x00cc1e30, bhk_extra_data_get_r_t_t_i() -> u32),
        entry!(0x00cc_1e40, fn_00cc1e40(Ptr)),
        entry!(0x00cc_1ef0, fn_00cc1ef0() -> u32),
        entry!(0x00cc1f60, bhk_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc_1f80, bhk_extra_data_get_viewer_strings(Ptr, u32)),
        entry!(0x00cc_2080, fn_00cc2080(Ptr, u32)),
        entry!(0x00cc_2120, fn_00cc2120(Ptr, u32, u32)),
        entry!(0x00cc_21f0, bhk_extra_data_link_object(Ptr, u32)),
        entry!(0x00cc_22a0, bhk_extra_data_create_clone(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{SCOPE_WORD, STRINGS_SET_AT};
    use super::*;

    /// A reference-counted object whose `DeleteThis` (slot 4) is the logged double
    /// `0x0900_0004`, with slot `0x54` (`register`), `0x60` (`viewer`) and `0x48` (`clone`)
    /// doubles `0x0900_0054`, `0x0900_0060` and `0x0900_0048`.
    fn object(e: &mut Engine, ref_count: u32) -> u32 {
        let object = ni_object(e, ref_count, 0x0900_0004);
        let vtable = e.mem.u32(object);
        e.mem.set_u32(vtable + 0x54, 0x0900_0054);
        e.mem.set_u32(vtable + 0x60, 0x0900_0060);
        e.mem.set_u32(vtable + 0x48, 0x0900_0048);
        object
    }

    fn object_doubles(e: &mut Engine) {
        interlocked_doubles(e);
        for slot in [0x0900_0004, 0x0900_0054, 0x0900_0060] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(0x0900_0048, |e, a| Ret {
            eax: a[0] + 0x1000 + e.mem.u32(a[1]),
            ..Ret::default()
        });
        e.register(RAW_FREE, |_, _| Ret::default());
        e.register(RAW_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
    }

    /// An array (`+4` data, `+8` capacity, `+0xC` used size, `+0x10` non-null count,
    /// `+0x14` growth step) holding `entries` with the capacity `capacity`.
    fn array(e: &mut Engine, entries: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(0x30);
        let data = e.mem.alloc(4 * capacity.max(1));
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(array + 8, capacity);
        e.mem.set_u32(array + 0xc, entries.len() as u32);
        let non_null = entries.iter().filter(|entry| **entry != 0).count();
        e.mem.set_u32(array + 0x10, non_null as u32);
        e.mem.set_u32(array + 0x14, 1);
        array
    }

    /// A `bhkExtraData` whose embedded array (`+0xC`) holds `entries`.
    fn extra_data(e: &mut Engine, entries: &[u32], capacity: u32) -> u32 {
        let this = e.mem.alloc(0x30);
        let data = e.mem.alloc(4 * capacity.max(1));
        for (index, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entry);
        }
        e.mem.set_u32(this + 0x10, data);
        e.mem.set_u32(this + 0x14, capacity);
        e.mem.set_u32(this + 0x18, entries.len() as u32);
        e.mem.set_u32(this + 0x1c, entries.len() as u32);
        e.mem.set_u32(this + 0x20, 1);
        this
    }

    #[test]
    fn array_deleting_destructor_frees_the_data_and_the_object_on_bit_0() {
        let mut e = logging_engine();
        e.register(RAW_FREE, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, 0x7000);
        assert_eq!(e.call(0x00c9_ae80, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_6b04);
        assert_eq!(calls_to(&e, RAW_FREE), vec![vec![0x7000]]);
        assert!(calls_to(&e, MEMORY_MANAGER_FREE).is_empty());
        e.call(0x00c9_ae80, &args![this, 3u32]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn process_clone_tail_jumps_and_answers_the_callee_result() {
        let mut e = logging_engine();
        e.register(PROCESS_CLONE_TARGET, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00cc_1960, &args![0x10u32, 0x20u32]).u32(), 0x21);
        assert_eq!(calls_to(&e, PROCESS_CLONE_TARGET), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn load_binary_runs_the_base_load_then_the_stream_call() {
        let mut e = logging_engine();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_AFTER_LOAD, |_, _| Ret::default());
        e.call(0x00cc_1970, &args![0x10u32, 0x20u32]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![0x00cc_1970, BASE_LOAD_BINARY, STREAM_AFTER_LOAD]
        );
        assert_eq!(calls_to(&e, STREAM_AFTER_LOAD), vec![vec![0x20]]);
    }

    #[test]
    fn register_streamables_visits_non_null_entries_and_answers_the_base_result() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| Ret {
            eax: 0xff01,
            ..Ret::default()
        });
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let this = extra_data(&mut e, &[a, 0, b], 4);
        assert!(e.call(0x00cc_1990, &args![this, 0x99u32]).bool());
        assert_eq!(
            calls_to(&e, 0x0900_0054),
            vec![vec![a, 0x99], vec![b, 0x99]]
        );
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00cc_1990, &args![this, 0x99u32]);
        assert_eq!(calls_to(&e, 0x0900_0054).len(), 2);
    }

    #[test]
    fn save_binary_writes_the_count_then_each_entry() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0900_0f10, |e, a| {
            // The buffer holds the count; the length cell starts at 4.
            assert_eq!(e.mem.u32(a[3]), 4);
            assert_eq!((a[2], a[4]), (4, 1));
            e.mem.set_u32(0x0900_4000, e.mem.u32(a[1]));
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        e.map(0x0900_4000, 4);
        e.register(0x0900_0030, |_, _| Ret::default());
        let stream = e.mem.alloc(0x300);
        let stream_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(stream, stream_vtable);
        e.mem.set_u32(stream_vtable + 0x30, 0x0900_0030);
        let writer = e.mem.alloc(0x20);
        e.mem.set_u32(writer + 4, 100);
        e.mem.set_u32(writer + 0xc, 0x0900_0f10);
        e.mem.set_u32(stream + 0x250, writer);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let this = extra_data(&mut e, &[a, 0, b], 4);
        e.call(0x00cc_1ad0, &args![this, stream]);
        assert_eq!(e.mem.u32(0x0900_4000), 3);
        assert_eq!(e.mem.u32(writer + 4), 104);
        assert_eq!(
            calls_to(&e, 0x0900_0030),
            vec![vec![stream, a], vec![stream, b]]
        );
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
    }

    #[test]
    fn set_size_grows_by_copying_entries_and_adding_references() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let this = array(&mut e, &[a, b], 2);
        let old = e.mem.u32(this + 4);
        e.call(0x00cc_1b40, &args![this, 4u32]);
        let block = e.mem.u32(this + 4);
        assert_ne!(block, old);
        assert_eq!(e.mem.u32(this + 8), 4);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(block), a);
        assert_eq!(e.mem.u32(block + 4), b);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!((e.mem.u32(a + 4), e.mem.u32(b + 4)), (2, 2));
        assert_eq!(calls_to(&e, RAW_ALLOCATE), vec![vec![16]]);
        assert_eq!(calls_to(&e, RAW_FREE), vec![vec![old]]);
        // The same capacity again does nothing.
        e.call(0x00cc_1b40, &args![this, 4u32]);
        assert_eq!(calls_to(&e, RAW_FREE).len(), 1);
    }

    #[test]
    fn set_size_shrinking_releases_the_dropped_entries() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let c = object(&mut e, 2);
        let this = array(&mut e, &[a, b, c], 3);
        e.call(0x00cc_1b40, &args![this, 1u32]);
        // b is deleted (its count reached 0), c only loses one reference.
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![b]]);
        assert_eq!(e.mem.u32(c + 4), 1);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 8), 1);
        let block = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(block), a);
        assert_eq!(e.mem.u32(a + 4), 2);
    }

    #[test]
    fn set_size_to_zero_drops_the_block() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        let a = object(&mut e, 1);
        let this = array(&mut e, &[a], 1);
        let old = e.mem.u32(this + 4);
        e.call(0x00cc_1b40, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls_to(&e, RAW_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, RAW_FREE), vec![vec![old]]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![a]]);
    }

    #[test]
    fn remove_all_releases_and_clears_every_entry() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        let a = object(&mut e, 1);
        let b = object(&mut e, 2);
        let this = array(&mut e, &[a, 0, b], 4);
        let data = e.mem.u32(this + 4);
        e.call(0x00cc_1cf0, &args![this]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![a]]);
        assert_eq!(e.mem.u32(b + 4), 1);
        assert_eq!(e.mem.u32(data), 0);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        // Empty array: the counters are zeroed all the same.
        e.mem.set_u32(this + 0x10, 5);
        e.call(0x00cc_1cf0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn constructor_sets_the_fields_and_drops_the_name_handle() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x0900_7008,
            ..Ret::default()
        });
        e.register(SET_NAME, |e, a| {
            // The cell holds the handle.
            assert_eq!(e.mem.u32(a[1]), 0x0900_7008);
            Ret::default()
        });
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x1234);
        e.map(0x0900_7000, 0x10);
        e.mem.set_u32(0x0900_7000, 5);
        let this = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00cc_1d80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x20), 1);
        for offset in [0x10, 0x14, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls_to(&e, FIXED_STRING_CREATE), vec![vec![NAME_STRING]]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT), vec![vec![0x0900_7000]]);
        assert_eq!(e.mem.u32(0x0900_7000), 4);
        // The empty string's handle is not released.
        e.register(FIXED_STRING_CREATE, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.register(SET_NAME, |_, _| Ret::default());
        e.call(0x00cc_1d80, &args![this]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn destructor_body_empties_the_array_under_the_scope_word() {
        let mut e = logging_engine();
        e.register(RAW_FREE, |_, _| Ret::default());
        e.register(BASE_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_REMOVE_ALL, |e, _| {
            // Called with the scope word raised.
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(0x0900_5000, e.mem.u32(word));
            Ret::default()
        });
        e.map(0x0900_5000, 4);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x10, 0x7777);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cc_1e40, &args![this]);
        assert_eq!(e.mem.u32(0x0900_5000), 0x10);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(e.mem.u32(this), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), ARRAY_DESTRUCTOR_VTABLE);
        assert_eq!(calls_to(&e, ARRAY_REMOVE_ALL), vec![vec![this + 0xc]]);
        assert_eq!(calls_to(&e, RAW_FREE), vec![vec![0x7777]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn factory_names_the_object_even_when_the_allocation_failed() {
        let mut e = logging_engine();
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: if a[0] == 0x24 {
                e.mem.u32(0x0900_1000)
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(SET_NAME, |_, _| Ret::default());
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, 0x4000);
        assert_eq!(e.call(0x00cc_1ef0, &args![]).u32(), 0x4001);
        e.mem.set_u32(0x0900_1000, 0);
        assert_eq!(e.call(0x00cc_1ef0, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, SET_NAME),
            vec![vec![0x4001, FACTORY_NAME], vec![0, FACTORY_NAME]]
        );
    }

    #[test]
    fn viewer_strings_append_the_class_and_ask_each_entry() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        viewer_doubles(&mut e);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.map(EXTRA_DATA_RTTI, 4);
        e.mem.set_u32(EXTRA_DATA_RTTI, 0x300);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let this = extra_data(&mut e, &[a, 0, b], 4);
        let strings = strings_header(&mut e, 0, 4, 2);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cc_1f80, &args![this, strings]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(pushed_texts(&e, strings, 0, 1), vec![0x7300]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 1);
        assert_eq!(
            calls_to(&e, 0x0900_0060),
            vec![vec![a, strings], vec![b, strings]]
        );
    }

    #[test]
    fn add_object_holds_a_reference_and_grows_a_full_array() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            // The cell holds the object, whose count includes the temporary reference.
            let object = e.mem.u32(a[2]);
            assert_eq!(e.mem.u32(object + 4), 2);
            Ret::default()
        });
        let this = extra_data(&mut e, &[0x1111, 0x2222], 2);
        let new = object(&mut e, 1);
        e.call(0x00cc_2080, &args![this, new]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![this + 0xc, 3]]);
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 1);
        assert_eq!(calls_to(&e, ARRAY_SET_AT)[0][..2], [this + 0xc, 2]);
        assert_eq!(e.mem.u32(new + 4), 1);
        // With room left no resize; a null object does nothing.
        e.mem.set_u32(this + 0x14, 8);
        e.call(0x00cc_2080, &args![this, new]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE).len(), 1);
        e.call(0x00cc_2080, &args![this, 0u32]);
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 2);
    }

    #[test]
    fn add_object_deletes_an_object_nobody_else_holds() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let this = extra_data(&mut e, &[], 4);
        let new = object(&mut e, 0);
        e.call(0x00cc_2080, &args![this, new]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![new]]);
    }

    #[test]
    fn copy_into_clone_adds_each_entry_clone_and_restores_the_scope() {
        let mut e = logging_engine();
        object_doubles(&mut e);
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ADD_OBJECT, |_, _| Ret::default());
        e.map(0x0900_6000, 4);
        let a = object(&mut e, 1);
        let b = object(&mut e, 1);
        let this = extra_data(&mut e, &[a, 0, b], 3);
        let clone = e.mem.alloc(0x30);
        let context = e.mem.alloc(8);
        e.mem.set_u32(context, 0x20);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cc_2120, &args![this, clone, context]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, context]]
        );
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![clone + 0xc, 3]]);
        // The 0x48 double answers `object + 0x1000 + word at context`.
        assert_eq!(
            calls_to(&e, ADD_OBJECT),
            vec![
                vec![clone, a + 0x1020],
                vec![clone, 0],
                vec![clone, b + 0x1020]
            ]
        );
        assert_eq!(
            calls_to(&e, 0x0900_0048),
            vec![vec![a, context], vec![b, context]]
        );
    }

    #[test]
    fn link_object_adds_each_linked_object() {
        let mut e = logging_engine();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(STREAM_LINK_ID_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            let mut answers = vec![0x50u32, 0, 0x70].into_iter();
            move |_, _| Ret {
                eax: answers.next().unwrap(),
                ..Ret::default()
            }
        });
        e.register(ADD_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        e.call(0x00cc_21f0, &args![this, 0x900u32]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, ADD_OBJECT),
            vec![vec![this, 0x50], vec![this, 0x70]]
        );
        assert_eq!(calls_to(&e, BASE_LINK_OBJECT), vec![vec![this, 0x900]]);
        // No link ids: nothing is fetched.
        e.register(STREAM_LINK_ID_COUNT, |_, _| Ret::default());
        e.call(0x00cc_21f0, &args![this, 0x900u32]);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 3);
    }

    #[test]
    fn create_clone_constructs_and_copies_even_without_a_block() {
        let mut e = logging_engine();
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: if a[0] == 0x24 {
                e.mem.u32(0x0900_1000)
            } else {
                0
            },
            ..Ret::default()
        });
        e.register(CONSTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(COPY_INTO_CLONE, |_, _| Ret::default());
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, 0x4000);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(e.call(0x00cc_22a0, &args![0x10u32, 0x20u32]).u32(), 0x4001);
        e.mem.set_u32(0x0900_1000, 0);
        assert_eq!(e.call(0x00cc_22a0, &args![0x10u32, 0x20u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, COPY_INTO_CLONE),
            vec![vec![0x10, 0x4001, 0x20], vec![0x10, 0, 0x20]]
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = logging_engine();
        e.register(0x00cc_1e40, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        assert_eq!(e.call(0x00cc_1f60, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc_1f60, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x24]]);
        assert_eq!(calls_to(&e, 0x00cc_1e40), vec![vec![this]; 2]);
    }

    #[test]
    fn get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_1e30, &args![]).u32(), 0x0126_8598);
    }
}
