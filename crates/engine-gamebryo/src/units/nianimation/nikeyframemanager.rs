//! `NiAnimation/nikeyframemanager.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiKeyframeManager::ms_RTTI` (the class's `NiRTTI` record).
const NI_KEYFRAME_MANAGER_RTTI: u32 = 0x011f_3fb4;

// Translated from 00a4da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_keyframe_manager_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_KEYFRAME_MANAGER_RTTI)
}

// Translated from 00a4d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A string hash for a table whose bucket count is the word at +4 (thiscall, `RET 4`):
/// `h = h * 33 + c` over the bytes of `name` (each sign-extended), then `h % buckets`
/// (unsigned).
pub fn fn_00a4d6d0(e: &mut Engine, this: Ptr, name: Ptr) -> u32 {
    let mut hash = 0u32;
    let mut p = name.addr();
    loop {
        let c = e.mem.u8(p);
        if c == 0 {
            break;
        }
        hash = (hash << 5)
            .wrapping_add(hash)
            .wrapping_add(c as i8 as i32 as u32);
        p += 1;
    }
    hash % e.mem.u32(this.addr() + 4)
}

/// The vtables of `NiKeyframeManager` and of its sequence map's classes, by RTTI:
/// `NiTStringTemplateMap<NiTPointerMap<..>, NiPointer<NiSequence>>` (`01098f08`),
/// `NiTPointerMap<const char *, NiPointer<NiSequence>>` (`01098ee8`), `NiTMapBase<..>`
/// (`01098ec8`) and `NiTStringPointerMap<NiPointer<NiSequence>>` (`01098f28`); and the
/// `NiTPrimitiveArray<unsigned int>` / `NiTPrimitiveArray<char *>` vtables (`0101a3ac`,
/// `01021024`) with their `NiTArray` bases' (`0101a3a4`, `0102102c`).
const NI_KEYFRAME_MANAGER_VTABLE: u32 = 0x0109_8f4c;
const STRING_TEMPLATE_MAP_VTABLE: u32 = 0x0109_8f08;
const POINTER_MAP_VTABLE: u32 = 0x0109_8ee8;
const MAP_BASE_VTABLE: u32 = 0x0109_8ec8;
const STRING_POINTER_MAP_VTABLE: u32 = 0x0109_8f28;
const UINT_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0101_a3ac;
const STRING_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0102_1024;
const UINT_ARRAY_VTABLE: u32 = 0x0101_a3a4;
const STRING_ARRAY_VTABLE: u32 = 0x0102_102c;
/// `NiKeyframeManager` size; its sequence map +0x34 (bucket count +4, buckets +8, count +0xc,
/// copy-strings byte +0x10 of the map).
const NI_KEYFRAME_MANAGER_SIZE: u32 = 0x48;
const SEQUENCE_MAP: u32 = 0x34;
const MAP_BUCKET_COUNT: u32 = 0x04;
const MAP_BUCKETS: u32 = 0x08;
const MAP_COUNT: u32 = 0x0c;
const MAP_COPY_STRINGS: u32 = 0x10;
/// The global the old-file loading keeps its link table in (`011f3fb0`: four arrays, see
/// `00a4d780`) between `LoadBinary` and `LinkObject`.
const OLD_LINKS: u32 = 0x011f_3fb0;
/// Callees: `00b601a0` (unnamed; thiscall on a map), `00aa1070` / `00aa10f0` (cdecl:
/// allocate / free), the memory manager singleton (`011f6238`) with its `Allocate`
/// (`00aa3e40`) and `Deallocate` (`00aa4060`), `00aa13e0` (cdecl, one word: the Ni
/// allocator), the sized delete (`00aa1460`) and `memset` (`00ec61c0`).
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const NI_SIZED_DELETE: u32 = 0x00aa_1460;
const MEMSET: u32 = 0x00ec_61c0;
/// `NiTimeController`'s constructor (`00a6d220`), destructor (`00a6d290`), `LinkObject`
/// (`00a6d500`) and `LoadBinary` (`00a6d5e0`), thiscall.
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
/// The stream (thiscall): `GetObjectFromLinkID` (`00a63430`), `GetNumberOfLinkIDs`
/// (`00a63460`), `SetNumberOfLinkIDs` (`00a64700`), `LoadCString` (`00a64ce0`, the address
/// to store it at) and `00a645b0` (reads a link ID).
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const SET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_4700;
const LOAD_CSTRING: u32 = 0x00a6_4ce0;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// The sequence callees (unit `nisequence.cpp`; thiscall): the constructor (`00a47bd0`,
/// `(name, 12, 12)`), `00a47a20` (`(name, object)`), `00a48070` (cdecl, four words: finds a
/// sequence) and `00a473d0` (one word); the map's `SetAt` (`00e927f0`, `(name, sequence)`,
/// the sequence passed counted, by value).
const SEQUENCE_CONSTRUCT: u32 = 0x00a4_7bd0;
const SEQUENCE_ADD: u32 = 0x00a4_7a20;
const FIND_SEQUENCE: u32 = 0x00a4_8070;
const SEQUENCE_00A473D0: u32 = 0x00a4_73d0;
const MAP_SET_AT: u32 = 0x00e9_27f0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots) and `DeleteThis` (virtual
/// slot 1).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const SLOT_DELETE_THIS: u32 = 0x04;
/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB names, PC offsets),
/// and the binary stream's position (+4) and read function (+8).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
/// The viewer-string array's resize (`0096ad30`) and `SetAt` (`0096ae90`), thiscall; the
/// old link table uses them on its arrays.
const ARRAY_GROW: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a4d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<..>::~NiTStringTemplateMap` (Xbox PDB): stores its vtable; when the
/// map copies its strings (+0x10), each bucket's entries' keys are freed (`00aa10f0`; the
/// entries themselves are not); then the `NiTPointerMap` and `NiTMapBase` vtables, each with
/// `00b601a0`, and the bucket array is freed.
pub fn ni_t_string_template_map_destruct(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    let copies = e.mem.u8(map.wrapping_add(MAP_COPY_STRINGS)) != 0;
    e.mem.set_u32(map, STRING_TEMPLATE_MAP_VTABLE);
    if copies {
        let mut bucket = 0u32;
        while bucket < e.mem.u32(map.wrapping_add(MAP_BUCKET_COUNT)) {
            let buckets = e.mem.u32(map.wrapping_add(MAP_BUCKETS));
            let mut entry = e.mem.u32(buckets.wrapping_add(bucket * 4));
            while entry != 0 {
                let key = e.mem.u32(entry.wrapping_add(4));
                entry = e.mem.u32(entry);
                e.call(NI_FREE, &args![key]);
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(map, POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(map.wrapping_add(MAP_BUCKETS));
    e.call(NI_FREE, &args![buckets]);
}

// Translated from 00a4d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiTMapBase<..>`'s scalar deleting destructor: its vtable,
/// `00b601a0`, the bucket array freed, and the map itself back to the memory manager when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_00a4d700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(MAP_BUCKETS));
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a4d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiTPointerMap<..>`'s scalar deleting destructor: its vtable
/// and `00b601a0`, then `NiTMapBase`'s vtable and `00b601a0`, the bucket array freed, and the
/// map itself back to the memory manager when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00a4d740(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr().wrapping_add(MAP_BUCKETS));
    e.call(NI_FREE, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a4d780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nikeyframemanager.cpp`): the constructor of the old-file
/// link table `LoadBinary` keeps at `011f3fb0`, four empty primitive arrays (16 bytes each:
/// vtable, data, 16-bit capacity +8, size +0xa, used count +0xc, grow-by +0xe set to 1):
/// `unsigned int` at +0, `char *` at +0x10, `unsigned int` at +0x20 and `char *` at +0x30.
/// Returns `this`.
pub fn fn_00a4d780(e: &mut Engine, this: Ptr) -> Ptr {
    let table = this.addr();
    for (offset, vtable) in [
        (0u32, UINT_PRIMITIVE_ARRAY_VTABLE),
        (0x10, STRING_PRIMITIVE_ARRAY_VTABLE),
        (0x20, UINT_PRIMITIVE_ARRAY_VTABLE),
        (0x30, STRING_PRIMITIVE_ARRAY_VTABLE),
    ] {
        let array = table.wrapping_add(offset);
        e.mem.set_u16(array.wrapping_add(8), 0);
        e.mem.set_u16(array.wrapping_add(0xa), 0);
        e.mem.set_u32(array.wrapping_add(4), 0);
        e.mem.set_u16(array.wrapping_add(0xe), 1);
        e.mem.set_u16(array.wrapping_add(0xc), 0);
        e.mem.set_u32(array, vtable);
    }
    this
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at +4 and, when it
/// reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// Adds `sequence` to the manager's map under its name (+8): the map's `SetAt` with the
/// sequence passed by value as a counted `NiPointer` (counted here, released by the callee).
fn add_to_map(e: &mut Engine, manager: u32, sequence: u32) {
    let name = e.mem.u32(sequence.wrapping_add(8));
    e.call(INTERLOCKED_INCREMENT, &args![sequence.wrapping_add(4)]);
    e.call(
        MAP_SET_AT,
        &args![manager.wrapping_add(SEQUENCE_MAP), name, sequence],
    );
}

// Translated from 00a4d810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::LinkObject` (Xbox PDB): `NiTimeController::LinkObject`. From file
/// version `0x04010003` on, each linked object (`GetNumberOfLinkIDs`, `GetObjectFromLinkID`)
/// goes into the sequence map under its name. Before, the link table at `011f3fb0` (from
/// `LoadBinary`) makes the sequences: per entry of its third array (a count each) a new
/// `NiSequence` (0x34 bytes, `00a47bd0` with the name from the second array, which is then
/// freed; a failed allocation leaves a null sequence the exe writes through) takes the first
/// array's word (+0x30) and the next linked object (+0x2c, counted), then that many
/// (name from the fourth array, linked object) pairs (`00a47a20`, each name freed), and goes
/// into the map. Then the table's arrays are freed (base vtables stored first) and the table
/// goes back to the memory manager; the global is cleared.
pub fn ni_keyframe_manager_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let manager = this.addr();
    let stream = stream.addr();
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= 0x0401_0003 {
        let mut count = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
        while count != 0 {
            let sequence = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            add_to_map(e, manager, sequence);
            count -= 1;
        }
        return;
    }
    let mut table = e.mem.u32(OLD_LINKS);
    let count = u32::from(e.mem.u16(table.wrapping_add(0x2a)));
    let mut next_entry = 0u32;
    let mut index = 0u32;
    while index < count {
        let names = e.mem.u32(table.wrapping_add(0x14));
        let name = e.mem.u32(names.wrapping_add(index * 4));
        let block = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
        let sequence = if block != 0 {
            e.call(SEQUENCE_CONSTRUCT, &args![block, name, 0xcu32, 0xcu32])
                .u32()
        } else {
            0
        };
        e.call(NI_FREE, &args![name]);
        let words = e.mem.u32(e.mem.u32(OLD_LINKS).wrapping_add(4));
        let word = e.mem.u32(words.wrapping_add(index * 4));
        e.mem.set_u32(sequence.wrapping_add(0x30), word);
        let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let old = e.mem.u32(sequence.wrapping_add(0x2c));
        if old != object {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(sequence.wrapping_add(0x2c), object);
            if object != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
            }
        }
        let counts = e.mem.u32(e.mem.u32(OLD_LINKS).wrapping_add(0x24));
        let mut pairs = e.mem.u32(counts.wrapping_add(index * 4));
        while pairs != 0 {
            let entries = e.mem.u32(e.mem.u32(OLD_LINKS).wrapping_add(0x34));
            let entry_name = e.mem.u32(entries.wrapping_add(next_entry * 4));
            pairs -= 1;
            next_entry += 1;
            let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.call(SEQUENCE_ADD, &args![sequence, entry_name, object]);
            e.call(NI_FREE, &args![entry_name]);
        }
        add_to_map(e, manager, sequence);
        index += 1;
        table = e.mem.u32(OLD_LINKS);
    }
    if table != 0 {
        e.mem.set_u32(table.wrapping_add(0x30), STRING_ARRAY_VTABLE);
        let data = e.mem.u32(table.wrapping_add(0x34));
        e.call(NI_FREE, &args![data]);
        let data = e.mem.u32(table.wrapping_add(0x24));
        e.mem.set_u32(table.wrapping_add(0x20), UINT_ARRAY_VTABLE);
        e.call(NI_FREE, &args![data]);
        let data = e.mem.u32(table.wrapping_add(0x14));
        e.mem.set_u32(table.wrapping_add(0x10), STRING_ARRAY_VTABLE);
        e.call(NI_FREE, &args![data]);
        let data = e.mem.u32(table.wrapping_add(4));
        e.mem.set_u32(table, UINT_ARRAY_VTABLE);
        e.call(NI_FREE, &args![data]);
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, table]);
    }
    e.mem.set_u32(OLD_LINKS, 0);
}

// Translated from 00a4da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringPointerMap<NiPointer<NiSequence>>::_scalar_deleting_destructor_` (Xbox PDB): the
/// string template map's destructor (`00a4d660`), then the map back to the memory manager
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_string_pointer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_t_string_template_map_destruct(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00a4da20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiKeyframeManager` constructor: the `NiTimeController`
/// constructor, the vtable, and the sequence map (37 buckets in a new zeroed block of 0x94
/// bytes from `00aa1070`, count 0, strings not copied, the `NiTStringPointerMap` vtable).
/// Returns `this`.
pub fn fn_00a4da20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![this]);
    let manager = this.addr();
    let map = manager.wrapping_add(SEQUENCE_MAP);
    e.mem.set_u32(manager, NI_KEYFRAME_MANAGER_VTABLE);
    e.mem.set_u32(map, MAP_BASE_VTABLE);
    e.mem.set_u32(map.wrapping_add(MAP_BUCKET_COUNT), 0x25);
    e.mem.set_u32(map.wrapping_add(MAP_COUNT), 0);
    let buckets = e.call(NI_ALLOC, &args![0x94u32]).u32();
    let size = e
        .mem
        .u32(map.wrapping_add(MAP_BUCKET_COUNT))
        .wrapping_mul(4);
    e.mem.set_u32(map.wrapping_add(MAP_BUCKETS), buckets);
    e.call(MEMSET, &args![buckets, 0u32, size]);
    e.mem.set_u8(map.wrapping_add(MAP_COPY_STRINGS), 0);
    e.mem.set_u32(map, STRING_POINTER_MAP_VTABLE);
    this
}

// Translated from 00a4da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::_scalar_deleting_destructor_` (Xbox PDB): the sequence map's
/// destructor (`00a4d660`), the `NiTimeController` destructor (`00a6d290`) and, when bit 0 of
/// `flags` is set, the sized delete of 0x48 bytes. Returns `this`.
pub fn ni_keyframe_manager_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let map = Ptr::new(this.addr().wrapping_add(SEQUENCE_MAP));
    ni_t_string_template_map_destruct(e, map);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_SIZED_DELETE, &args![this, NI_KEYFRAME_MANAGER_SIZE]);
    }
    this
}

// Translated from 00a4dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nikeyframemanager.cpp`; thiscall, three words): finds a
/// sequence (`00a48070` with `(first, third, &found, second)`); none gives false. Otherwise,
/// with a non-null `second`, `00a473d0` runs on the sequence with it; the sequence goes into
/// the map under its name (+8; counted while passed) and its reference is dropped. True.
pub fn fn_00a4dac0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> bool {
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, 0);
        e.call(FIND_SEQUENCE, &args![first, third, slot, second]);
        let sequence = e.mem.u32(slot);
        if sequence == 0 {
            return false;
        }
        if second != 0 {
            e.call(SEQUENCE_00A473D0, &args![sequence, second]);
        }
        let sequence = e.mem.u32(slot);
        let name = e.mem.u32(sequence.wrapping_add(8));
        let held = e.mem.u32(slot);
        if held != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![held.wrapping_add(4)]);
        }
        e.call(
            MAP_SET_AT,
            &args![this.addr().wrapping_add(SEQUENCE_MAP), name, sequence],
        );
        let held = e.mem.u32(slot);
        if held != 0 {
            release(e, held);
        }
        true
    })
}

// Translated from 00a4db60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::CreateObject` (registered as the `"NiKeyframeManager"` loader at
/// `00a2bde0`; Ghidra has no function here): allocates 0x48 bytes and, when that succeeds,
/// jumps to the constructor (`00a4da20`) on it. Returns it, or 0.
pub fn ni_keyframe_manager_create_object(e: &mut Engine) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![NI_KEYFRAME_MANAGER_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    fn_00a4da20(e, Ptr::new(block)).addr()
}

/// One read of `size` bytes through the stream's input stream into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the position advances.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The array append the compiler inlined with the value in the stack word `slot`: grows the
/// array (16-bit size +0xA, capacity +8, grow-by +0xE) when full, then `SetAt` at the old
/// size with the slot's address.
fn append(e: &mut Engine, array: u32, slot: u32) {
    let size = u32::from(e.mem.u16(array.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(array.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(array.wrapping_add(0xe)));
        e.call(ARRAY_GROW, &args![array, step + size]);
    }
    e.call(ARRAY_SET_AT, &args![array, size, slot]);
}

/// The inline `NiTPrimitiveArray::SetAtGrow` of `value` at the array's size (data +4, 16-bit
/// size +0xA, capacity +8, used count +0xC, grow-by +0xE).
fn set_at_grow(e: &mut Engine, array: u32, value: u32) {
    let index = u32::from(e.mem.u16(array.wrapping_add(0xa)));
    if index >= u32::from(e.mem.u16(array.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(array.wrapping_add(0xe)));
        e.call(ARRAY_GROW, &args![array, step + index]);
    }
    let used = array.wrapping_add(0xc);
    if index >= u32::from(e.mem.u16(array.wrapping_add(0xa))) {
        e.mem.set_u16(array.wrapping_add(0xa), (index + 1) as u16);
        if value != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else if value != 0 {
        let data = e.mem.u32(array.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) == 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else {
        let data = e.mem.u32(array.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(0xffff));
        }
    }
    let data = e.mem.u32(array.wrapping_add(4));
    e.mem.set_u32(data.wrapping_add(index * 4), value);
}

// Translated from 00a4db80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiKeyframeManager::LoadBinary` (Xbox PDB): `NiTimeController::LoadBinary`, a count (4
/// bytes), then that many entries, each starting with a byte. From file version `0x04010003`:
/// a set byte reads two strings (`LoadCString`) and a word, adds the sequence `00a4dac0` finds
/// for them (second string, first string, word) and frees both strings; a clear byte reads a
/// link ID, and the number of those becomes the stream's link-ID count. Before: a string is
/// read first; a set byte reads a second string and a word and adds as above; a clear byte
/// makes a new link table (0x40 bytes, `00a4d780`; the exe writes through a null one when the
/// allocation fails) at `011f3fb0` holding the string, a word, a link ID read, and a count of
/// (string, link ID) pairs read into its arrays. The words and the count byte are read
/// through the stack words the exe uses (the argument's own word holds the byte).
pub fn ni_keyframe_manager_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    // Frame (offsets from the stack pointer after the saved registers): words +0x10, +0x14,
    // +0x18, +0x1c, +0x20, +0x28, +0x2c, +0x30, then the argument word +0x38.
    e.with_stack(0x3c, |e, frame| {
        let at = |offset: u32| frame.addr() + offset;
        let argument = at(0x38);
        e.mem.set_u32(argument, stream);
        let old = e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x0401_0003;
        if old {
            stream_read(e, stream, at(0x20), 4, 4, argument);
            let mut remaining = e.mem.u32(at(0x20));
            while remaining != 0 {
                remaining = remaining.wrapping_sub(1);
                e.mem.set_u32(at(0x20), remaining);
                stream_read(e, stream, argument, 1, 1, at(0x1c));
                e.mem.set_u32(at(0x18), 0);
                e.call(LOAD_CSTRING, &args![stream, at(0x18)]);
                if e.mem.u8(argument) != 0 {
                    e.mem.set_u32(at(0x1c), 0);
                    e.call(LOAD_CSTRING, &args![stream, at(0x1c)]);
                    stream_read(e, stream, at(0x2c), 4, 4, at(0x28));
                    let word = e.mem.u32(at(0x2c));
                    let first = e.mem.u32(at(0x18));
                    let second = e.mem.u32(at(0x1c));
                    fn_00a4dac0(e, this, second, first, word);
                    let second = e.mem.u32(at(0x1c));
                    e.call(NI_FREE, &args![second]);
                    let first = e.mem.u32(at(0x18));
                    e.call(NI_FREE, &args![first]);
                } else {
                    let block = e
                        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x40u32])
                        .u32();
                    let table = if block != 0 {
                        fn_00a4d780(e, Ptr::new(block)).addr()
                    } else {
                        0
                    };
                    e.mem.set_u32(OLD_LINKS, table);
                    append(e, table.wrapping_add(0x10), at(0x18));
                    stream_read(e, stream, at(0x30), 4, 4, at(0x28));
                    let table = e.mem.u32(OLD_LINKS);
                    append(e, table, at(0x30));
                    e.call(STREAM_READ_LINK_ID, &args![stream]);
                    stream_read(e, stream, at(0x10), 4, 4, at(0x28));
                    let table = e.mem.u32(OLD_LINKS);
                    append(e, table.wrapping_add(0x20), at(0x10));
                    while e.mem.u32(at(0x10)) != 0 {
                        let left = e.mem.u32(at(0x10)).wrapping_sub(1);
                        e.mem.set_u32(at(0x10), left);
                        e.mem.set_u32(at(0x1c), 0);
                        e.call(LOAD_CSTRING, &args![stream, at(0x1c)]);
                        let table = e.mem.u32(OLD_LINKS);
                        let name = e.mem.u32(at(0x1c));
                        set_at_grow(e, table.wrapping_add(0x30), name);
                        e.call(STREAM_READ_LINK_ID, &args![stream]);
                    }
                    let left = e.mem.u32(at(0x10)).wrapping_sub(1);
                    e.mem.set_u32(at(0x10), left);
                }
                remaining = e.mem.u32(at(0x20));
            }
            return;
        }
        stream_read(e, stream, at(0x14), 4, 4, argument);
        let mut links = 0u32;
        while e.mem.u32(at(0x14)) != 0 {
            let left = e.mem.u32(at(0x14)).wrapping_sub(1);
            e.mem.set_u32(at(0x14), left);
            stream_read(e, stream, argument, 1, 1, at(0x30));
            if e.mem.u8(argument) != 0 {
                e.mem.set_u32(at(0x20), 0);
                e.call(LOAD_CSTRING, &args![stream, at(0x20)]);
                e.mem.set_u32(at(0x1c), 0);
                e.call(LOAD_CSTRING, &args![stream, at(0x1c)]);
                stream_read(e, stream, at(0x2c), 4, 4, at(0x30));
                let word = e.mem.u32(at(0x2c));
                let first = e.mem.u32(at(0x20));
                let second = e.mem.u32(at(0x1c));
                fn_00a4dac0(e, this, second, first, word);
                let first = e.mem.u32(at(0x20));
                e.call(NI_FREE, &args![first]);
                let second = e.mem.u32(at(0x1c));
                e.call(NI_FREE, &args![second]);
            } else {
                e.call(STREAM_READ_LINK_ID, &args![stream]);
                links += 1;
            }
        }
        let left = e.mem.u32(at(0x14)).wrapping_sub(1);
        e.mem.set_u32(at(0x14), left);
        e.call(SET_NUMBER_OF_LINK_IDS, &args![stream, links]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4da80, ni_keyframe_manager_get_rtti() -> Ptr),
        entry!(0x00a4_d6d0, fn_00a4d6d0(Ptr, Ptr) -> u32),
        entry!(0x00a4_d660, ni_t_string_template_map_destruct(Ptr)),
        entry!(0x00a4_d700, fn_00a4d700(Ptr, u32) -> Ptr),
        entry!(0x00a4_d740, fn_00a4d740(Ptr, u32) -> Ptr),
        entry!(0x00a4_d780, fn_00a4d780(Ptr) -> Ptr),
        entry!(0x00a4_d810, ni_keyframe_manager_link_object(Ptr, Ptr)),
        entry!(
            0x00a4_da00,
            ni_t_string_pointer_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a4_da20, fn_00a4da20(Ptr) -> Ptr),
        entry!(
            0x00a4_da90,
            ni_keyframe_manager_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a4_dac0, fn_00a4dac0(Ptr, u32, u32, u32) -> bool),
        entry!(0x00a4_db60, ni_keyframe_manager_create_object() -> u32),
        entry!(0x00a4_db80, ni_keyframe_manager_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_keyframe_manager_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a4da80, &args![]).u32(), NI_KEYFRAME_MANAGER_RTTI);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn string_hash_modulo_the_bucket_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 37);
        let name = e.mem.alloc(8);
        for (i, b) in [b'a', b'b', 0xe9u8].iter().enumerate() {
            e.mem.set_u8(name + i as u32, *b);
        }
        // 0xe9 is a negative char: -23.
        let hash = (u32::from(b'a') * 33 + u32::from(b'b')) * 33;
        let want = hash.wrapping_sub(23) % 37;
        assert_eq!(e.call(0x00a4_d6d0, &args![this, name]).u32(), want);
        let empty = e.mem.alloc(1);
        assert_eq!(e.call(0x00a4_d6d0, &args![this, empty]).u32(), 0);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The interlocked import slots, logged and counting in memory.
    fn interlocked(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                ret(count)
            });
        }
    }

    /// A map with two buckets: the first holds two entries (keys 0x100, 0x200), the second
    /// none.
    fn map(e: &mut Engine, copies: bool) -> u32 {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, 0x200);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 0x100);
        e.mem.set_u32(buckets, first);
        e.mem.set_u32(map + MAP_BUCKET_COUNT, 2);
        e.mem.set_u32(map + MAP_BUCKETS, buckets);
        e.mem.set_u8(map + MAP_COPY_STRINGS, u8::from(copies));
        map
    }

    #[test]
    fn string_template_map_destructor_frees_copied_keys() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_FREE, 0);
        let vtables = Rc::new(RefCell::new(vec![]));
        let seen = vtables.clone();
        e.register_double(MAP_REMOVE_ALL, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        let copying = map(&mut e, true);
        let buckets = e.mem.u32(copying + MAP_BUCKETS);
        e.call(0x00a4_d660, &args![copying]);
        assert_eq!(
            calls(&log, NI_FREE),
            [vec![0x100], vec![0x200], vec![buckets]]
        );
        assert_eq!(*vtables.borrow(), [POINTER_MAP_VTABLE, MAP_BASE_VTABLE]);
        let plain = map(&mut e, false);
        let buckets = e.mem.u32(plain + MAP_BUCKETS);
        e.call(0x00a4_d660, &args![plain]);
        assert_eq!(calls(&log, NI_FREE).len(), 4);
        assert_eq!(calls(&log, NI_FREE)[3], [buckets]);
    }

    #[test]
    fn map_deleting_destructors_free_and_delete_on_bit_0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_FREE, 0);
        logged(&mut e, &log, MAP_REMOVE_ALL, 0);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        let a = map(&mut e, false);
        assert_eq!(e.call(0x00a4_d700, &args![a, 0u32]).u32(), a);
        assert_eq!(e.mem.u32(a), MAP_BASE_VTABLE);
        assert_eq!(calls(&log, MAP_REMOVE_ALL), [vec![a]]);
        assert!(calls(&log, MEMORY_MANAGER_DEALLOCATE).is_empty());
        e.call(0x00a4_d740, &args![a, 1u32]);
        assert_eq!(calls(&log, MAP_REMOVE_ALL).len(), 3);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            [vec![MEMORY_MANAGER, a]]
        );
        e.call(0x00a4_da00, &args![a, 1u32]);
        assert_eq!(calls(&log, MEMORY_MANAGER_DEALLOCATE).len(), 2);
        assert_eq!(calls(&log, NI_FREE).len(), 3);
    }

    #[test]
    fn link_table_constructor_makes_four_empty_arrays() {
        let mut e = Engine::new();
        let table = e.mem.alloc(0x40);
        e.mem.write(table, &[0xaa; 0x40]);
        assert_eq!(e.call(0x00a4_d780, &args![table]).u32(), table);
        for (offset, vtable) in [
            (0u32, UINT_PRIMITIVE_ARRAY_VTABLE),
            (0x10, STRING_PRIMITIVE_ARRAY_VTABLE),
            (0x20, UINT_PRIMITIVE_ARRAY_VTABLE),
            (0x30, STRING_PRIMITIVE_ARRAY_VTABLE),
        ] {
            assert_eq!(e.mem.u32(table + offset), vtable);
            assert_eq!(e.mem.u32(table + offset + 4), 0);
            assert_eq!(e.mem.u32(table + offset + 8), 0);
            assert_eq!(e.mem.u16(table + offset + 0xc), 0);
            assert_eq!(e.mem.u16(table + offset + 0xe), 1);
        }
    }

    #[test]
    fn constructor_and_create_object_build_the_manager() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_CONSTRUCT, 0);
        let buckets = e.mem.alloc(0x94);
        logged(&mut e, &log, NI_ALLOC, buckets);
        logged(&mut e, &log, MEMSET, 0);
        let block = Rc::new(Cell::new(0u32));
        let answer = block.clone();
        e.register_double(NI_ALLOCATE, move |_, _| ret(answer.get()));
        assert_eq!(e.call(0x00a4_db60, &args![]).u32(), 0);
        let manager = e.mem.alloc(0x48);
        e.mem.set_u8(manager + SEQUENCE_MAP + MAP_COPY_STRINGS, 1);
        block.set(manager);
        assert_eq!(e.call(0x00a4_db60, &args![]).u32(), manager);
        assert_eq!(e.mem.u32(manager), NI_KEYFRAME_MANAGER_VTABLE);
        let map = manager + SEQUENCE_MAP;
        assert_eq!(e.mem.u32(map), STRING_POINTER_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + MAP_BUCKET_COUNT), 0x25);
        assert_eq!(e.mem.u32(map + MAP_BUCKETS), buckets);
        assert_eq!(e.mem.u8(map + MAP_COPY_STRINGS), 0);
        assert_eq!(calls(&log, NI_ALLOC), [vec![0x94]]);
        assert_eq!(calls(&log, MEMSET), [vec![buckets, 0, 0x94]]);
        assert_eq!(calls(&log, TIME_CONTROLLER_CONSTRUCT), [vec![manager]]);
    }

    #[test]
    fn scalar_deleting_destructor_tears_down_the_map() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_FREE, 0);
        logged(&mut e, &log, MAP_REMOVE_ALL, 0);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        logged(&mut e, &log, NI_SIZED_DELETE, 0);
        let manager = e.mem.alloc(0x48);
        e.call(0x00a4_da90, &args![manager, 0u32]);
        assert_eq!(
            calls(&log, MAP_REMOVE_ALL),
            [vec![manager + 0x34], vec![manager + 0x34]]
        );
        assert!(calls(&log, NI_SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00a4_da90, &args![manager, 1u32]).u32(), manager);
        assert_eq!(calls(&log, NI_SIZED_DELETE), [vec![manager, 0x48]]);
        assert_eq!(calls(&log, TIME_CONTROLLER_DESTRUCT).len(), 2);
    }

    /// A sequence object (name at +8) with a `DeleteThis` double.
    fn sequence(e: &mut Engine, name: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, 0x00ee_9000);
        let sequence = e.mem.alloc(0x34);
        e.mem.set_u32(sequence, vtable);
        e.mem.set_u32(sequence + 8, name);
        sequence
    }

    #[test]
    fn fn_00a4dac0_adds_the_found_sequence() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        logged(&mut e, &log, 0x00ee_9000, 0);
        logged(&mut e, &log, SEQUENCE_00A473D0, 0);
        logged(&mut e, &log, MAP_SET_AT, 0);
        let found = Rc::new(Cell::new(0u32));
        let answer = found.clone();
        let finds = log.clone();
        e.register_double(FIND_SEQUENCE, move |e, a| {
            finds.borrow_mut().push((FIND_SEQUENCE, a.to_vec()));
            assert_eq!(e.mem.u32(a[2]), 0);
            e.mem.set_u32(a[2], answer.get());
            Ret::default()
        });
        let manager = e.mem.alloc(0x48);
        assert!(!e
            .call(0x00a4_dac0, &args![manager, 1u32, 2u32, 3u32])
            .bool());
        let held = sequence(&mut e, 0x4444);
        e.mem.set_u32(held + 4, 1);
        found.set(held);
        assert!(e
            .call(0x00a4_dac0, &args![manager, 1u32, 2u32, 3u32])
            .bool());
        assert_eq!(&calls(&log, FIND_SEQUENCE)[1][..2], [1, 3]);
        assert_eq!(calls(&log, FIND_SEQUENCE)[1][3], 2);
        assert_eq!(calls(&log, SEQUENCE_00A473D0), [vec![held, 2]]);
        assert_eq!(
            calls(&log, MAP_SET_AT),
            [vec![manager + 0x34, 0x4444, held]]
        );
        // Counted while passed, then dropped: back to 1, not deleted.
        assert_eq!(e.mem.u32(held + 4), 1);
        assert!(calls(&log, 0x00ee_9000).is_empty());
        assert!(e
            .call(0x00a4_dac0, &args![manager, 1u32, 0u32, 3u32])
            .bool());
        assert_eq!(calls(&log, SEQUENCE_00A473D0).len(), 1);
    }

    #[test]
    fn link_object_of_new_files_adds_each_linked_sequence() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        logged(&mut e, &log, TIME_CONTROLLER_LINK_OBJECT, 0);
        logged(&mut e, &log, MAP_SET_AT, 0);
        logged(&mut e, &log, GET_NUMBER_OF_LINK_IDS, 2);
        let a = sequence(&mut e, 0x10);
        let b = sequence(&mut e, 0x20);
        let objects = RefCell::new(vec![a, b]);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(objects.borrow_mut().remove(0))
        });
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0401_0003);
        let manager = e.mem.alloc(0x48);
        e.call(0x00a4_d810, &args![manager, stream]);
        assert_eq!(
            calls(&log, MAP_SET_AT),
            [vec![manager + 0x34, 0x10, a], vec![manager + 0x34, 0x20, b]]
        );
        assert_eq!(e.mem.u32(a + 4), 1);
    }

    #[test]
    fn link_object_of_old_files_builds_sequences_from_the_link_table() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        for address in [
            TIME_CONTROLLER_LINK_OBJECT,
            MAP_SET_AT,
            NI_FREE,
            SEQUENCE_ADD,
            MEMORY_MANAGER_DEALLOCATE,
        ] {
            logged(&mut e, &log, address, 0);
        }
        let made = sequence(&mut e, 0x55);
        let blocks = e.mem.alloc(0x34);
        e.register_double(NI_ALLOCATE, move |_, _| ret(blocks));
        logged(&mut e, &log, SEQUENCE_CONSTRUCT, made);
        let first_object = e.mem.alloc(8);
        let objects = RefCell::new(vec![first_object, 0x7200, 0x7300]);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(objects.borrow_mut().remove(0))
        });
        // A table with one sequence: name 0x111, word 0x222, two entries 0x333 and 0x444.
        let table = e.mem.alloc(0x40);
        let words = e.mem.alloc(4);
        e.mem.set_u32(words, 0x222);
        let names = e.mem.alloc(4);
        e.mem.set_u32(names, 0x111);
        let counts = e.mem.alloc(4);
        e.mem.set_u32(counts, 2);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(entries, 0x333);
        e.mem.set_u32(entries + 4, 0x444);
        e.mem.set_u32(table + 4, words);
        e.mem.set_u32(table + 0x14, names);
        e.mem.set_u32(table + 0x24, counts);
        e.mem.set_u16(table + 0x2a, 1);
        e.mem.set_u32(table + 0x34, entries);
        e.map(OLD_LINKS, 4);
        e.mem.set_u32(OLD_LINKS, table);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0401_0002);
        let manager = e.mem.alloc(0x48);
        e.call(0x00a4_d810, &args![manager, stream]);
        assert_eq!(
            calls(&log, SEQUENCE_CONSTRUCT),
            [vec![blocks, 0x111, 0xc, 0xc]]
        );
        assert_eq!(e.mem.u32(made + 0x30), 0x222);
        assert_eq!(e.mem.u32(made + 0x2c), first_object);
        assert_eq!(e.mem.u32(first_object + 4), 1);
        assert_eq!(
            calls(&log, SEQUENCE_ADD),
            [vec![made, 0x333, 0x7200], vec![made, 0x444, 0x7300]]
        );
        assert_eq!(calls(&log, MAP_SET_AT), [vec![manager + 0x34, 0x55, made]]);
        assert_eq!(
            calls(&log, NI_FREE),
            [
                vec![0x111],
                vec![0x333],
                vec![0x444],
                vec![entries],
                vec![counts],
                vec![names],
                vec![words]
            ]
        );
        assert_eq!(e.mem.u32(table), UINT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(table + 0x10), STRING_ARRAY_VTABLE);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            [vec![MEMORY_MANAGER, table]]
        );
        assert_eq!(e.mem.u32(OLD_LINKS), 0);
    }

    /// A stream whose reads come from `words` (each read takes the next word and stores
    /// `size` bytes of it).
    fn stream(e: &mut Engine, version: u32, words: Vec<u32>) -> u32 {
        let queue = RefCell::new(words);
        e.register_double(0x00ee_9100, move |e, a| {
            let word = queue.borrow_mut().remove(0);
            let bytes = word.to_le_bytes();
            e.mem.write(a[1], &bytes[..a[2] as usize]);
            ret(a[2])
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + BINARY_STREAM_READ, 0x00ee_9100);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        stream
    }

    /// `LoadCString` handing out 0x9001, 0x9002, ... in turn.
    fn strings(e: &mut Engine, log: &Log) {
        let next = Cell::new(0x9000u32);
        let record = log.clone();
        e.register_double(LOAD_CSTRING, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            next.set(next.get() + 1);
            e.mem.set_u32(a[1], next.get());
            record.borrow_mut().push((LOAD_CSTRING, vec![next.get()]));
            Ret::default()
        });
    }

    #[test]
    fn load_binary_of_new_files_adds_sequences_and_counts_links() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_LOAD_BINARY, 0);
        logged(&mut e, &log, STREAM_READ_LINK_ID, 0);
        logged(&mut e, &log, SET_NUMBER_OF_LINK_IDS, 0);
        logged(&mut e, &log, NI_FREE, 0);
        strings(&mut e, &log);
        // fn_00a4dac0 is in this file: its callee is doubled to record the words.
        let found = log.clone();
        e.register_double(FIND_SEQUENCE, move |_, a| {
            found.borrow_mut().push((FIND_SEQUENCE, a.to_vec()));
            Ret::default()
        });
        // Three entries: a set byte with word 7, a clear byte, a clear byte.
        let stream = stream(&mut e, 0x0401_0003, vec![3, 1, 7, 0, 0]);
        let manager = e.mem.alloc(0x48);
        e.call(0x00a4_db80, &args![manager, stream]);
        let find = calls(&log, FIND_SEQUENCE);
        assert_eq!(find.len(), 1);
        // (second string, word, &found, first string)
        assert_eq!((find[0][0], find[0][1], find[0][3]), (0x9002, 7, 0x9001));
        assert_eq!(calls(&log, NI_FREE), [vec![0x9001], vec![0x9002]]);
        assert_eq!(calls(&log, STREAM_READ_LINK_ID).len(), 2);
        assert_eq!(calls(&log, SET_NUMBER_OF_LINK_IDS), [vec![stream, 2]]);
    }

    #[test]
    fn load_binary_of_old_files_fills_the_link_table() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_LOAD_BINARY, 0);
        logged(&mut e, &log, STREAM_READ_LINK_ID, 0);
        logged(&mut e, &log, NI_FREE, 0);
        strings(&mut e, &log);
        e.register_double(FIND_SEQUENCE, |_, _| Ret::default());
        let table = e.mem.alloc(0x40);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, table);
        let appended = Rc::new(RefCell::new(vec![]));
        let record = appended.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            record.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.register(ARRAY_GROW, |e, a| {
            // A bigger data block holding the old entries, and the new capacity.
            let data = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            for i in 0..u32::from(e.mem.u16(a[0] + 0xa)) {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.map(OLD_LINKS, 4);
        // Two entries: a clear byte (word 5, two pairs), then a set byte (word 9).
        let stream = stream(&mut e, 0x0401_0002, vec![2, 0, 5, 2, 1, 9]);
        let manager = e.mem.alloc(0x48);
        e.call(0x00a4_db80, &args![manager, stream]);
        assert_eq!(e.mem.u32(OLD_LINKS), table);
        assert_eq!(e.mem.u32(table + 0x10), STRING_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(
            *appended.borrow(),
            [
                (table + 0x10, 0, 0x9001),
                (table, 0, 5),
                (table + 0x20, 0, 2)
            ]
        );
        // The pairs' strings went into the fourth array.
        let data = e.mem.u32(table + 0x34);
        assert_eq!([e.mem.u32(data), e.mem.u32(data + 4)], [0x9002, 0x9003]);
        assert_eq!(e.mem.u16(table + 0x3a), 2);
        assert_eq!(e.mem.u16(table + 0x3c), 2);
        assert_eq!(calls(&log, STREAM_READ_LINK_ID).len(), 3);
        // The second entry's strings (0x9004 first, 0x9005 second) are freed.
        assert_eq!(calls(&log, NI_FREE), [vec![0x9005], vec![0x9004]]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [vec![MEMORY_MANAGER, 0x40]]
        );
    }
}
