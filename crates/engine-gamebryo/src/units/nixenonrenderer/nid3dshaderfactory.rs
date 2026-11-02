//! `NiXenonRenderer/nid3dshaderfactory.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::ALLOCATE;

/// `00aa0390` (`thiscall`, no stack word, no unit): the base constructor.
const BASE_CONSTRUCTOR: u32 = 0x00aa_0390;
/// `00fdf054`: import slot of `InitializeCriticalSection` (`stdcall`, one word).
const INITIALIZE_CRITICAL_SECTION: u32 = 0x00fd_f054;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `00aa1070` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
/// `00ec61c0` (`cdecl`, `memset`: block, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// `00a29680` (`cdecl`, one word here): the empty call the exe makes with a string.
const EMPTY_CALL: u32 = 0x00a2_9680;
/// `0126fa0c`: the shader factory object `fn_00e92e60` builds.
const FACTORY_SLOT: u32 = 0x0126_fa0c;
/// `011f6044`: a second pointer to the same object.
const FACTORY_ALIAS_SLOT: u32 = 0x011f_6044;

/// Builds a hash-map-like object of `size` pointer slots in 0x14 bytes (the vtables
/// `first` then `second`, the size at `+4`, a zeroed slot block of `block_bytes` at `+8`,
/// the word at `+0xc` 0, the byte at `+0x10` `flag`); answers it, or 0 when the allocation
/// fails.
fn make_slot_table(
    e: &mut Engine,
    first: u32,
    second: u32,
    size: u32,
    block_bytes: u32,
    flag: u8,
) -> u32 {
    let table = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x14u32],
        )
        .u32();
    if table == 0 {
        return 0;
    }
    e.mem.set_u32(table, first);
    e.mem.set_u32(table + 4, size);
    e.mem.set_u32(table + 0xc, 0);
    let slots = e.call(ALLOCATE_BLOCK, &args![block_bytes]).u32();
    let bytes = e.mem.u32(table + 4).wrapping_mul(4);
    e.mem.set_u32(table + 8, slots);
    e.call(MEMSET, &args![slots, 0u32, bytes]);
    e.mem.set_u8(table + 0x10, flag);
    e.mem.set_u32(table, second);
    table
}

// Translated from 00e92cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderfactory.cpp`; the body of a constructor,
/// `thiscall`, no stack word): runs the base constructor `00aa0390`, sets the vtable
/// `010f0cc4`, clears the word at `+0x24`, initializes the critical section at `+0x80`,
/// then makes two 0x14-byte tables kept at `+0x28` (vtables `010f0bc0` then `010f0c80`,
/// 0x3b slots from a 0xec-byte block, flag byte 1) and `+0x30` (vtables `010f0c20` then
/// `010f0ca0`, 0x25 slots from a 0x94-byte block, flag byte 0); a table whose allocation
/// fails is stored as null. Returns `this`.
pub fn fn_00e92cf0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, 0x010f_0cc4);
    e.mem.set_u32(base + 0x24, 0);
    e.call(INITIALIZE_CRITICAL_SECTION, &args![base + 0x80]);
    let first = make_slot_table(e, 0x010f_0bc0, 0x010f_0c80, 0x3b, 0xec, 1);
    e.mem.set_u32(base + 0x28, first);
    let second = make_slot_table(e, 0x010f_0c20, 0x010f_0ca0, 0x25, 0x94, 0);
    e.mem.set_u32(base + 0x30, second);
    this
}

// Translated from 00e92e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::_SDMInit` (Xbox PDB; run by the start-up `00e91dc0`, no arguments):
/// while `0126fa0c` is null, allocates 0x100 bytes (`00aa13e0`), constructs the factory
/// with `fn_00e92cf0` (null when the allocation fails) and keeps it in `0126fa0c` and
/// `011f6044`; when `011f6044` is null it calls `00a29680` with the string at `010f0d40`.
pub fn ni_d3d_shader_factory_sdm_init(e: &mut Engine) {
    if e.mem.u32(FACTORY_SLOT) == 0 {
        let block = e.call(ALLOCATE, &args![0x100u32]).u32();
        let factory = if block != 0 {
            fn_00e92cf0(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(FACTORY_SLOT, factory);
        e.mem.set_u32(FACTORY_ALIAS_SLOT, factory);
    }
    if e.mem.u32(FACTORY_ALIAS_SLOT) == 0 {
        e.call(EMPTY_CALL, &args![0x010f_0d40u32]);
    }
}

/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The recursive lock of the shared item free list (`011c5f80`: owner, count at +4), taken
/// by `0040fbf0` (`thiscall`, one word the function does not read) and released inline.
const FREE_LIST_LOCK_011C5F80: u32 = 0x011c_5f80;
const LOCK_ACQUIRE_0040FBF0: u32 = 0x0040_fbf0;
/// The head of the free list (`011c5f58`).
const FREE_LIST_HEAD_011C5F58: u32 = 0x011c_5f58;

// Translated from 00e6e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *,NiPointer<NiShaderLibrary> >::DeleteItem` (engine map, from the
/// vtable; the body fits), `thiscall` with one stack word, the item (next +0, key +4, value
/// +8); `this` is not read. Releases the value's reference (`InterlockedDecrement` on its
/// count at +4, deleted through its vtable slot +4 when that answers 0) and clears it; then,
/// under the free-list lock `011c5f80` (`0040fbf0` with 0), clears the key, links the item in
/// front of the free list at `011c5f58` and releases the lock inline (the count at
/// `011c5f84` decremented, the owner cleared when it reaches 0).
pub fn ni_t_pointer_map_delete_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    let item = item.addr();
    let value = e.mem.u32(item + 8);
    if value != 0 {
        let left = e
            .call(IMPORT_INTERLOCKED_DECREMENT, &args![value + 4])
            .u32();
        if left == 0 {
            e.vcall(value, 4, &args![]);
        }
        e.mem.set_u32(item + 8, 0);
    }
    e.call(LOCK_ACQUIRE_0040FBF0, &args![FREE_LIST_LOCK_011C5F80, 0u32]);
    e.mem.set_u32(item + 4, 0);
    let head = e.global::<u32>(FREE_LIST_HEAD_011C5F58);
    e.mem.set_u32(item, head);
    let count = e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4).wrapping_sub(1);
    e.set_global::<u32>(FREE_LIST_LOCK_011C5F80 + 4, count);
    e.set_global::<u32>(FREE_LIST_HEAD_011C5F58, item);
    if count == 0 {
        e.set_global::<u32>(FREE_LIST_LOCK_011C5F80, 0);
    }
}

// Translated from 00e91f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`), no arguments: the shader factory
/// word at `0126fa0c`.
pub fn fn_00e91f10(e: &mut Engine) -> u32 {
    e.mem.u32(FACTORY_SLOT)
}

use crate::types::NiTPointerMap;

/// `vsprintf_s(buffer, size, format, va_list)` (LIBCMT, `cdecl`).
const VSPRINTF_S: u32 = 0x00ec_6bb5;

// Translated from 00a305b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSprintf` (engine map), `cdecl` with a variable argument list: with a size of 0 answers 1
/// and writes nothing; else hands the variable arguments, as a `va_list` that points at them,
/// to `vsprintf_s(buffer, size, format, ...)`, writes 0 to the buffer's last byte
/// (`buffer[size - 1]`) and answers `vsprintf_s`'s result. `variable_arguments` are the words
/// the caller pushed after `format`, in order; they are copied to memory because the `va_list`
/// is the address of the first one.
pub fn ni_sprintf(
    e: &mut Engine,
    buffer: Ptr,
    size: u32,
    format: Ptr,
    variable_arguments: &[u32],
) -> i32 {
    if size == 0 {
        return 1;
    }
    let list = e.mem.alloc(4 * variable_arguments.len().max(1) as u32);
    for (i, word) in variable_arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = e.call(VSPRINTF_S, &args![buffer, size, format, list]).i32();
    e.mem.free(list);
    e.mem
        .set_u8(buffer.addr().wrapping_add(size).wrapping_sub(1), 0);
    result
}

/// The uniform form of [`ni_sprintf`]: it takes any number of argument words, so it cannot go
/// through `entry!`. The first three words are the buffer, its size and the format; the rest
/// are the variable arguments.
fn ni_sprintf_uniform(e: &mut Engine, a: &[u32]) -> Ret {
    ni_sprintf(e, Ptr::new(a[0]), a[1], Ptr::new(a[2]), &a[3..]).into_ret()
}

/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

/// An `NiPointer` let go: the object's count (+4) down through `InterlockedDecrement` and, at
/// zero, its virtual slot 4 (delete).
fn release_reference(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00a4d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,char const *,NiPointer<NiShaderLibrary> >
/// ::SetValue` (engine map; a virtual slot), `thiscall` with three stack words, `this` not
/// read: the item, the key and the value (an `NiPointer` passed by value, which the function
/// lets go at its end). The key goes to the item's +4; the item's value (+8) gets an
/// `NiPointer` assignment (unless it holds the value already: the old one let go, the new one
/// stored and, when not null, counted up; a null value ends the function there). Then the
/// by-value pointer is let go (count down, deleted at zero).
pub fn ni_t_map_base_set_value(e: &mut Engine, _this: Ptr, item: Ptr, key: u32, value: u32) {
    let item = item.addr();
    e.mem.set_u32(item.wrapping_add(4), key);
    let held = e.mem.u32(item.wrapping_add(8));
    if held != value {
        release_reference(e, held);
        e.mem.set_u32(item.wrapping_add(8), value);
        if value == 0 {
            return;
        }
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
    }
    release_reference(e, value);
}

// Translated from 00b4f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiShader::DynamicCast` (engine map), `thiscall` with an RTTI record: walks the object's
/// RTTI (virtual slot 8) and its bases (+4); answers `this` when the record is among them,
/// else 0.
pub fn ni_shader_dynamic_cast(e: &mut Engine, this: Ptr, rtti: u32) -> u32 {
    let mut current = e.vcall(this.addr(), 8, &args![]).u32();
    while current != 0 {
        if current == rtti {
            return this.addr();
        }
        current = e.mem.u32(current.wrapping_add(4));
    }
    0
}

// Translated from 00e91e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::SetRenderer` (engine map), `thiscall` with the renderer. Unless it is
/// the one at +0x24 already, stores it and hands it to every shader (virtual slot 0x1c of
/// each), walking them through the factory's virtual slots 0x38 (first) and 0x3c (next).
pub fn ni_d3d_shader_factory_set_renderer(e: &mut Engine, this: Ptr, renderer: u32) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x24)) == renderer {
        return;
    }
    e.mem.set_u32(base.wrapping_add(0x24), renderer);
    let mut shader = e.vcall(base, 0x38, &args![]).u32();
    while shader != 0 {
        e.vcall(shader, 0x1c, &args![renderer]);
        shader = e.vcall(base, 0x3c, &args![]).u32();
    }
}

// Translated from 00e92080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`; no arguments): deletes the shader
/// factory at `0126fa0c` (its slot 0 with 1) when there is one and clears the word.
pub fn fn_00e92080(e: &mut Engine) {
    let factory = e.mem.u32(FACTORY_SLOT);
    if factory != 0 {
        e.vcall(factory, 0, &args![1u32]);
        e.mem.set_u32(FACTORY_SLOT, 0);
    }
}

/// The key format `"%s%d"` (`01094c50`) and the shader map's lookup `00a53c30` (unnamed;
/// `thiscall`: the key, the address of the word it fills; answers a byte).
const SHADER_KEY_FORMAT_01094C50: u32 = 0x0109_4c50;
const MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;

// Translated from 00e921d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::FindShader` (engine map), `thiscall` with a name and a number:
/// without the shader map (+0x28) answers 0. Else the key `"%s%d"` of the two is written to a
/// 0x104-byte stack buffer ([`ni_sprintf`]) and looked up in the map (`00a53c30`, filling a
/// stack word cleared first); answers the word when the lookup answers true, else 0.
// Translation incomplete: 00e921d0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_d3d_shader_factory_find_shader(e: &mut Engine, this: Ptr, name: u32, number: u32) -> u32 {
    let map = e.mem.u32(this.addr().wrapping_add(0x28));
    if map == 0 {
        return 0;
    }
    e.with_stack(0x108, |e, frame| {
        let found = frame.addr();
        let buffer = found + 4;
        e.mem.set_u32(found, 0);
        ni_sprintf(
            e,
            Ptr::new(buffer),
            0x104,
            Ptr::new(SHADER_KEY_FORMAT_01094C50),
            &[name, number],
        );
        let map = e.mem.u32(this.addr().wrapping_add(0x28));
        let hit = e.call(MAP_LOOKUP_00A53C30, &args![map, buffer, found]).u8();
        if hit != 0 {
            e.mem.u32(found)
        } else {
            0
        }
    })
}

/// `00b601a0` (unnamed; `thiscall` on a map, no stack words): empties the map.
const MAP_REMOVE_ALL_00B601A0: u32 = 0x00b6_01a0;

// Translated from 00e92460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveAllShaders` (engine map), `thiscall`: empties the shader map
/// (+0x28, `00b601a0`, a tail jump) when there is one.
pub fn ni_d3d_shader_factory_remove_all_shaders(e: &mut Engine, this: Ptr) {
    let map = e.mem.u32(this.addr().wrapping_add(0x28));
    if map != 0 {
        e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
    }
}

/// The free `00aa10f0` (`cdecl`, a block) and the tables of the two string maps' destructors
/// (RTTI `NiTStringTemplateMap<NiTPointerMap<char const *, NiShader *>, NiShader *>`, its
/// `NiTPointerMap` and `NiTMapBase`; the same three for `NiPointer<NiShaderLibrary>`).
const FREE_00AA10F0: u32 = 0x00aa_10f0;
const SHADER_STRING_MAP_VTABLES: [u32; 3] = [0x010f_0c00, 0x010f_0be0, 0x010f_0bc0];
const LIBRARY_STRING_MAP_VTABLES: [u32; 3] = [0x010f_0c60, 0x010f_0c40, 0x010f_0c20];

/// The shared body of the two `NiTStringTemplateMap` destructors: the first table; when the
/// map copies its keys (the byte +0x10), every item's key (+4) in every bucket is freed
/// (`00aa10f0`, the next item read first); then the second table and `00b601a0`, the third
/// table and `00b601a0` again, and the bucket array (+8) freed.
fn string_map_down(e: &mut Engine, this: Ptr<NiTPointerMap>, tables: [u32; 3]) {
    let base = this.addr();
    e.mem.set_u32(base, tables[0]);
    if e.mem.u8(base.wrapping_add(0x10)) != 0 {
        let mut bucket = 0u32;
        while bucket < e.get(this, NiTPointerMap::m_uiHashSize) {
            let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
            let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            while item != 0 {
                let key = e.mem.u32(item.wrapping_add(4));
                item = e.mem.u32(item);
                e.call(FREE_00AA10F0, &args![key]);
            }
            bucket += 1;
        }
    }
    e.mem.set_u32(base, tables[1]);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    e.mem.set_u32(base, tables[2]);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![base]);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(FREE_00AA10F0, &args![buckets]);
}

// Translated from 00e92520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the
/// `NiTStringTemplateMap<NiTPointerMap<char const *, NiShader *>, NiShader *>` destructor (the
/// RTTI of the table it stores), `thiscall`: with key copies (+0x10) every key is freed, then
/// the `NiTPointerMap` and `NiTMapBase` parts empty the map (`00b601a0`) and the bucket array
/// is freed (`00aa10f0`).
pub fn fn_00e92520(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_down(e, this, SHADER_STRING_MAP_VTABLES);
}

// Translated from 00e92590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`; a virtual slot of several maps,
/// `thiscall` with two words, `this` not read): whether the two keys are the same word.
pub fn fn_00e92590(_e: &mut Engine, _this: Ptr, first: u32, second: u32) -> bool {
    first == second
}

// Translated from 00e925a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the
/// `NiTStringTemplateMap<NiTPointerMap<char const *, NiPointer<NiShaderLibrary>>, ...>`
/// destructor (the RTTI of the table it stores), `thiscall`: as [`fn_00e92520`] with its own
/// tables.
pub fn fn_00e925a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    string_map_down(e, this, LIBRARY_STRING_MAP_VTABLES);
}

/// The memory manager's deallocate (`00aa4060`, `thiscall` on `011f6238`, the block) and the
/// sized free `00aa1460` (`cdecl`: block, size).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const FREE_00AA1460: u32 = 0x00aa_1460;

// Translated from 00e92690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *,NiShader *>,NiShader *>::scalar deleting
/// destructor` (engine map), `thiscall` with the flags: the destructor ([`fn_00e92520`]), then
/// with bit 0 the block given back to the memory manager (`00aa4060`). Answers `this`.
pub fn ni_t_string_template_map_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00e92520(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e92760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): an `NiTPointerMap` lookup with an
/// `NiPointer` result, `thiscall` with the key and the address of the result. The bucket is the
/// map's hash of the key (virtual slot 4); its items are compared through the map's virtual
/// slot 8 (the key, the item's key at +4). Not found: answers false. Found: the result gets an
/// `NiPointer` assignment of the item's value (+8) (unless it holds it already: the old one
/// let go, the new one counted up when not null) and true is answered.
pub fn fn_00e92760(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, result: Ptr) -> bool {
    let base = this.addr();
    let index = e.vcall(base, 4, &args![key]).u32();
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(buckets.wrapping_add(index.wrapping_mul(4)));
    loop {
        if item == 0 {
            return false;
        }
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(base, 8, &args![key, item_key]).u8() != 0 {
            break;
        }
        item = e.mem.u32(item);
    }
    let held = e.mem.u32(result.addr());
    if held != e.mem.u32(item.wrapping_add(8)) {
        release_reference(e, held);
        let value = e.mem.u32(item.wrapping_add(8));
        e.mem.set_u32(result.addr(), value);
        if value != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
        }
    }
    true
}

/// The library map's step to the next item (`00e6e630`, unnamed; `thiscall` on the map with
/// three words: the address of the iterator, of the key and of the value), the library's name
/// (`NiShaderLibrary::GetName`, `00d47a30`, `thiscall`) and the removal of a library by name
/// (`00e91fd0`, unnamed; `thiscall` on the library map, the name).
const MAP_STEP_00E6E630: u32 = 0x00e6_e630;
const LIBRARY_GET_NAME_00D47A30: u32 = 0x00d4_7a30;
const REMOVE_LIBRARY_00E91FD0: u32 = 0x00e9_1fd0;

/// The library map's next value from the iterator at `iterator` (`00e6e630` with a stack key
/// word and a stack `NiPointer` cleared first); the `NiPointer` is let go before the value is
/// answered.
fn next_library(e: &mut Engine, map: u32, iterator: u32) -> u32 {
    e.with_stack(8, |e, frame| {
        let value = frame.addr();
        let key = value + 4;
        e.mem.set_u32(value, 0);
        e.call(MAP_STEP_00E6E630, &args![map, iterator, key, value]);
        let library = e.mem.u32(value);
        release_reference(e, library);
        library
    })
}

// Translated from 00e92b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetFirstLibrary` (engine map), `thiscall`: without the library map
/// (+0x30) answers 0. Else the iterator (+0x2c) is set to the first item of the first non-empty
/// bucket (the bucket count read once), or null; with one, the next value is taken from it
/// (`00e6e630`, which also moves the iterator on) into a stack `NiPointer`, let go, and
/// answered; else 0.
pub fn ni_d3d_shader_factory_get_first_library(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let map = e.mem.u32(base.wrapping_add(0x30));
    if map == 0 {
        return 0;
    }
    let size = e.mem.u32(map.wrapping_add(4));
    let mut first = 0;
    if size != 0 {
        let buckets = e.mem.u32(map.wrapping_add(8));
        for bucket in 0..size {
            let item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
            if item != 0 {
                first = item;
                break;
            }
        }
    }
    e.mem.set_u32(base.wrapping_add(0x2c), first);
    if first == 0 {
        return 0;
    }
    next_library(e, map, base.wrapping_add(0x2c))
}

// Translated from 00e92c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveAllLibraries` (engine map), `thiscall`: without the library map
/// (+0x30) does nothing. Else, from the first library ([`ni_d3d_shader_factory_get_first_library`]),
/// each library is removed by its name (`GetName`, then `00e91fd0` on the map) and the next one
/// taken from the iterator (+0x2c) while the map and the iterator are set; then the map is
/// emptied (`00b601a0` on the word at +0x30, read again).
pub fn ni_d3d_shader_factory_remove_all_libraries(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x30)) == 0 {
        return;
    }
    let mut library = ni_d3d_shader_factory_get_first_library(e, this);
    while library != 0 {
        let name = e.call(LIBRARY_GET_NAME_00D47A30, &args![library]).u32();
        let map = e.mem.u32(base.wrapping_add(0x30));
        e.call(REMOVE_LIBRARY_00E91FD0, &args![map, name]);
        let map = e.mem.u32(base.wrapping_add(0x30));
        if map == 0 || e.mem.u32(base.wrapping_add(0x2c)) == 0 {
            break;
        }
        library = next_library(e, map, base.wrapping_add(0x2c));
    }
    let map = e.mem.u32(base.wrapping_add(0x30));
    e.call(MAP_REMOVE_ALL_00B601A0, &args![map]);
}

// Translated from 00e92cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *,NiPointer<NiShaderLibrary> >,
/// NiPointer<NiShaderLibrary> >::scalar deleting destructor` (engine map), `thiscall` with the
/// flags: the destructor ([`fn_00e925a0`]), then with bit 0 the block given back to the memory
/// manager (`00aa4060`). Answers `this`.
pub fn ni_t_string_template_map_library_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00e925a0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

/// The imported `DeleteCriticalSection` (import slot) and `NiShaderFactory::~NiShaderFactory`
/// (`00aa05a0`, `thiscall`).
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const NI_SHADER_FACTORY_DOWN_00AA05A0: u32 = 0x00aa_05a0;

// Translated from 00e92dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`nid3dshaderfactory.cpp`): the `NiD3DShaderFactory` destructor
/// (the RTTI of the table it stores), `thiscall`. Its table; the shader map (+0x28) emptied
/// (`00b601a0`) when there is one; all libraries removed
/// ([`ni_d3d_shader_factory_remove_all_libraries`]); both maps (+0x28, +0x30) deleted (their
/// slot 0 with 1) when set; +0x28, +0x30 and the renderer (+0x24) cleared; the critical
/// section at +0x80 deleted; then `NiShaderFactory::~NiShaderFactory` (a tail jump).
pub fn fn_00e92dc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010f_0cc4);
    let shaders = e.mem.u32(base.wrapping_add(0x28));
    if shaders != 0 {
        e.call(MAP_REMOVE_ALL_00B601A0, &args![shaders]);
    }
    ni_d3d_shader_factory_remove_all_libraries(e, this);
    for offset in [0x28u32, 0x30] {
        let map = e.mem.u32(base.wrapping_add(offset));
        if map != 0 {
            e.vcall(map, 0, &args![1u32]);
        }
    }
    e.mem.set_u32(base.wrapping_add(0x28), 0);
    e.mem.set_u32(base.wrapping_add(0x30), 0);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    e.call(
        IMPORT_DELETE_CRITICAL_SECTION,
        &args![base.wrapping_add(0x80)],
    );
    e.call(NI_SHADER_FACTORY_DOWN_00AA05A0, &args![base]);
}

// Translated from 00e92e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::scalar deleting destructor` (engine map), `thiscall` with the flags:
/// the destructor ([`fn_00e92dc0`]), then with bit 0 the 0x100 bytes freed (`00aa1460`).
/// Answers `this`.
pub fn ni_d3d_shader_factory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00e92dc0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_00AA1460, &args![this, 0x100u32]);
    }
    this
}

/// `NiXenonRenderer::PurgeAllD3DShaders` (`00e75380`, engine map; `thiscall`, no stack words).
const PURGE_ALL_D3D_SHADERS_00E75380: u32 = 0x00e7_5380;

// Translated from 00e91ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::PurgeAllShaderRendererData` (engine map), `thiscall`: with a renderer
/// (`m_pkRenderer` +0x24, Xbox PDB) its `PurgeAllD3DShaders` (`00e75380`, a jump).
pub fn ni_d3d_shader_factory_purge_all_shader_renderer_data(e: &mut Engine, this: Ptr) {
    let renderer = e.mem.u32(this.addr().wrapping_add(0x24));
    if renderer != 0 {
        e.call(PURGE_ALL_D3D_SHADERS_00E75380, &args![renderer]);
    }
}

// Translated from 00e91eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetMinorPixelShaderVersion` (engine map), `thiscall` (`this` not read)
/// with the version word: its low byte.
pub fn ni_d3d_shader_factory_get_minor_pixel_shader_version(
    _e: &mut Engine,
    _this: Ptr,
    version: u32,
) -> u32 {
    version & 0xff
}

// Translated from 00e91ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::CreatePixelShaderVersion` (engine map), `thiscall` (`this` not read)
/// with the major and minor numbers: `((major | 0xffffff00) << 8) | minor`.
pub fn ni_d3d_shader_factory_create_pixel_shader_version(
    _e: &mut Engine,
    _this: Ptr,
    major: u32,
    minor: u32,
) -> u32 {
    ((major | 0xffff_ff00) << 8) | minor
}

// Translated from 00e91ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetMajorPixelShaderVersion` (engine map), `thiscall` (`this` not read)
/// with the version word: its second byte.
pub fn ni_d3d_shader_factory_get_major_pixel_shader_version(
    _e: &mut Engine,
    _this: Ptr,
    version: u32,
) -> u32 {
    (version >> 8) & 0xff
}

// Translated from 00e91ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::CreateVertexShaderVersion` (engine map), `thiscall` (`this` not read)
/// with the major and minor numbers: `((major | 0xfffffe00) << 8) | minor`.
pub fn ni_d3d_shader_factory_create_vertex_shader_version(
    _e: &mut Engine,
    _this: Ptr,
    major: u32,
    minor: u32,
) -> u32 {
    ((major | 0xffff_fe00) << 8) | minor
}

/// `NiD3DShaderProgramFactory::GetInstance` (`00e92eb0`, engine map; no arguments) and the
/// program factory's directory functions (engine map; `thiscall`): `GetFirstProgramDirectory`
/// (`00e92fe0`), `GetNextProgramDirectory` (`00e93000`), `AddProgramDirectory` (`00e93960`)
/// and `RemoveProgramDirectory` (`00e93bb0`) with one stack word, `RemoveAllProgramDirectories`
/// (`00e93cc0`) with none.
const PROGRAM_FACTORY_INSTANCE_00E92EB0: u32 = 0x00e9_2eb0;
const FIRST_PROGRAM_DIRECTORY_00E92FE0: u32 = 0x00e9_2fe0;
const NEXT_PROGRAM_DIRECTORY_00E93000: u32 = 0x00e9_3000;
const ADD_PROGRAM_DIRECTORY_00E93960: u32 = 0x00e9_3960;
const REMOVE_PROGRAM_DIRECTORY_00E93BB0: u32 = 0x00e9_3bb0;
const REMOVE_ALL_PROGRAM_DIRECTORIES_00E93CC0: u32 = 0x00e9_3cc0;

/// The program factory's function `target` (a jump with the caller's stack word, when it has
/// one), or 0 (the null factory in EAX) without a factory.
fn on_program_factory(e: &mut Engine, target: u32, word: Option<u32>) -> u32 {
    let factory = e.call(PROGRAM_FACTORY_INSTANCE_00E92EB0, &args![]).u32();
    if factory == 0 {
        return 0;
    }
    match word {
        Some(word) => e.call(target, &args![factory, word]).u32(),
        None => e.call(target, &args![factory]).u32(),
    }
}

// Translated from 00e91f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetFirstProgramDirectory` (engine map), `thiscall` (`this` not read)
/// with one word: the program factory's `GetFirstProgramDirectory` with it
/// ([`on_program_factory`]).
pub fn ni_d3d_shader_factory_get_first_program_directory(
    e: &mut Engine,
    _this: Ptr,
    word: u32,
) -> u32 {
    on_program_factory(e, FIRST_PROGRAM_DIRECTORY_00E92FE0, Some(word))
}

// Translated from 00e91f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetNextProgramDirectory` (engine map): as
/// [`ni_d3d_shader_factory_get_first_program_directory`] with `GetNextProgramDirectory`.
pub fn ni_d3d_shader_factory_get_next_program_directory(
    e: &mut Engine,
    _this: Ptr,
    word: u32,
) -> u32 {
    on_program_factory(e, NEXT_PROGRAM_DIRECTORY_00E93000, Some(word))
}

// Translated from 00e91f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::AddProgramDirectory` (engine map): as
/// [`ni_d3d_shader_factory_get_first_program_directory`] with `AddProgramDirectory`.
pub fn ni_d3d_shader_factory_add_program_directory(e: &mut Engine, _this: Ptr, word: u32) -> u32 {
    on_program_factory(e, ADD_PROGRAM_DIRECTORY_00E93960, Some(word))
}

// Translated from 00e91f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveProgramDirectory` (engine map): as
/// [`ni_d3d_shader_factory_get_first_program_directory`] with `RemoveProgramDirectory`.
pub fn ni_d3d_shader_factory_remove_program_directory(
    e: &mut Engine,
    _this: Ptr,
    word: u32,
) -> u32 {
    on_program_factory(e, REMOVE_PROGRAM_DIRECTORY_00E93BB0, Some(word))
}

// Translated from 00e91fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveAllProgramDirectories` (engine map), `thiscall` (`this` not
/// read), no stack words: the program factory's `RemoveAllProgramDirectories`
/// ([`on_program_factory`]).
pub fn ni_d3d_shader_factory_remove_all_program_directories(e: &mut Engine, _this: Ptr) -> u32 {
    on_program_factory(e, REMOVE_ALL_PROGRAM_DIRECTORIES_00E93CC0, None)
}

/// The renderer string `00e91fc0` answers (`010f005c`).
const RENDERER_STRING_010F005C: u32 = 0x010f_005c;

// Translated from 00e91fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (slot +0x58 of the factory vtable `010f0cc4`, which the Xbox PDB
/// names `GetRendererString`), `thiscall`, `this` not read: the string at `010f005c`.
pub fn ni_d3d_shader_factory_get_renderer_string(_e: &mut Engine, _this: Ptr) -> u32 {
    RENDERER_STRING_010F005C
}

/// What `RetrieveShader` calls: the log object (`011f6388`; its slot 0x14, `thiscall` with five
/// words), `NiShaderFactory::ReportError` (`00aa0020`, `cdecl`), the imported
/// `EnterCriticalSection`/`LeaveCriticalSection`, `NiXenonRenderer::RegisterD3DShader`
/// (`00e752b0`, `thiscall`, the shader), `NiD3DShaderInterface::SetD3DRenderer` (`00e818d0`,
/// `thiscall`, the renderer) and its messages.
const LOG_011F6388: u32 = 0x011f_6388;
const REPORT_ERROR_00AA0020: u32 = 0x00aa_0020;
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
const REGISTER_D3D_SHADER_00E752B0: u32 = 0x00e7_52b0;
const SET_D3D_RENDERER_00E818D0: u32 = 0x00e8_18d0;
const NO_RENDERER_010F0B84: u32 = 0x010f_0b84;
const NO_NAME_010F0B20: u32 = 0x010f_0b20;
const NOT_FOUND_010F0B54: u32 = 0x010f_0b54;

// Translated from 00e920a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RetrieveShader` (engine map), `thiscall` with three words: the name,
/// the implementation and a byte asking for a report. Without a renderer (+0x24) the log
/// (`011f6388`, when set) gets `"Attempting to RetrieveShader w/out a valid renderer!"` (its
/// slot 0x14 with 4, the text and three zeros) and 0 is answered; without a name (or an empty
/// one) the error is reported (`00aa0020` with 1 and 0) and 0 answered. The shader is looked up
/// (`FindShader`, slot 0xc); not found, under the factory's lock (+0x80) the first library is
/// taken (slot 0x38), the lookup repeated, and while it finds nothing each library (from the
/// next, slot 0x3c) is asked (its slot 8 with the renderer, the name and the implementation);
/// a shader one answers is inserted (slot 0x18 with the implementation), registered with the
/// renderer (`00e752b0`) and given it (`SetD3DRenderer`). With the byte set and no shader the
/// failure is reported with the name and implementation. Answers the shader.
pub fn ni_d3d_shader_factory_retrieve_shader(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    implementation: u32,
    report: u8,
) -> u32 {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x24)) == 0 {
        let log = e.mem.u32(LOG_011F6388);
        if log != 0 {
            e.vcall(
                log,
                0x14,
                &args![4u32, NO_RENDERER_010F0B84, 0u32, 0u32, 0u32],
            );
        }
        return 0;
    }
    if name == 0 || e.mem.u8(name) == 0 {
        e.call(REPORT_ERROR_00AA0020, &args![1u32, 0u32, NO_NAME_010F0B20]);
        return 0;
    }
    let mut shader = e.vcall(base, 0xc, &args![name, implementation]).u32();
    if shader == 0 {
        let lock = base.wrapping_add(0x80);
        e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![lock]);
        let mut library = e.vcall(base, 0x38, &args![]).u32();
        shader = e.vcall(base, 0xc, &args![name, implementation]).u32();
        if shader == 0 {
            while library != 0 {
                let renderer = e.mem.u32(base.wrapping_add(0x24));
                shader = e
                    .vcall(library, 8, &args![renderer, name, implementation])
                    .u32();
                if shader != 0 {
                    e.vcall(base, 0x18, &args![shader, implementation]);
                    let renderer = e.mem.u32(base.wrapping_add(0x24));
                    e.call(REGISTER_D3D_SHADER_00E752B0, &args![renderer, shader]);
                    let renderer = e.mem.u32(base.wrapping_add(0x24));
                    e.call(SET_D3D_RENDERER_00E818D0, &args![shader, renderer]);
                    break;
                }
                library = e.vcall(base, 0x3c, &args![]).u32();
            }
        }
        e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
    }
    if report != 0 && shader == 0 {
        e.call(
            REPORT_ERROR_00AA0020,
            &args![1u32, 0u32, NOT_FOUND_010F0B54, name, implementation],
        );
    }
    shader
}

/// The shader's implementation (`00cc0760`, a getter of the word at +0xc, `m_uiImplementation`
/// in the Xbox PDB; the engine map names the shared body `NiD3DShaderProgram::GetName`).
const IMPLEMENTATION_00CC0760: u32 = 0x00cc_0760;

/// The libraries walked under the factory's lock (+0x80), from the first (slot 0x38) through
/// the next (slot 0x3c), until `ask` answers true for one; answers whether one did.
fn any_library(e: &mut Engine, base: u32, ask: impl Fn(&mut Engine, u32) -> bool) -> bool {
    let lock = base.wrapping_add(0x80);
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![lock]);
    let mut library = e.vcall(base, 0x38, &args![]).u32();
    let mut released = false;
    while library != 0 {
        if ask(e, library) {
            released = true;
            break;
        }
        library = e.vcall(base, 0x3c, &args![]).u32();
    }
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![lock]);
    released
}

// Translated from 00e92270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::ReleaseShaderFromLibrary` (engine map), `thiscall` with the name and the
/// implementation. False without a name (or an empty one). Unless the shader is found
/// (`FindShader`, slot 0xc) or one found for implementation -1 has this implementation
/// (`00cc0760`), false. Else each library ([`any_library`]) is asked to release it (its slot
/// 0x10 with the name and implementation); true when one answers true.
pub fn ni_d3d_shader_factory_release_shader_from_library(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    implementation: u32,
) -> bool {
    let base = this.addr();
    if name == 0 || e.mem.u8(name) == 0 {
        return false;
    }
    if e.vcall(base, 0xc, &args![name, implementation]).u32() == 0 {
        let any = e.vcall(base, 0xc, &args![name, 0xffff_ffffu32]).u32();
        if any == 0 || e.call(IMPLEMENTATION_00CC0760, &args![any]).u32() != implementation {
            return false;
        }
    }
    any_library(e, base, |e, library| {
        e.vcall(library, 0x10, &args![name, implementation]).bool()
    })
}

/// The address of a shader's name (`00a62660`, a getter of +8, `m_kName` in the Xbox PDB; the
/// engine map names the shared body `NiBSplineBasisData::GetDegree3Basis`).
const NAME_ADDRESS_00A62660: u32 = 0x00a6_2660;

// Translated from 00e92330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::ReleaseShaderFromLibrary` (engine map, `_ov2`), `thiscall` with the
/// shader. False for a null shader. Unless `FindShader` (slot 0xc) with its name (`00a62660`)
/// and implementation (`00cc0760`), or else with its name and -1, answers this shader, false.
/// Else each library ([`any_library`]) is asked to release it (its slot 0xc with the shader);
/// true when one answers true.
pub fn ni_d3d_shader_factory_release_shader_from_library_ov2(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
) -> bool {
    let base = this.addr();
    if shader == 0 {
        return false;
    }
    let name_at = e.call(NAME_ADDRESS_00A62660, &args![shader]).u32();
    let name = e.mem.u32(name_at);
    let implementation = e.call(IMPLEMENTATION_00CC0760, &args![shader]).u32();
    let found = e.vcall(base, 0xc, &args![name, implementation]).u32();
    if found == 0 || found != shader {
        let name_at = e.call(NAME_ADDRESS_00A62660, &args![shader]).u32();
        let name = e.mem.u32(name_at);
        let found = e.vcall(base, 0xc, &args![name, 0xffff_ffffu32]).u32();
        if found == 0 || found != shader {
            return false;
        }
    }
    any_library(e, base, |e, library| {
        e.vcall(library, 0xc, &args![shader]).bool()
    })
}

// Translated from 00e923f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveShader` (engine map), `thiscall` with the name and the
/// implementation: with a shader map (+0x28) the key `"%s%d"` of the two is written to a
/// 0x104-byte stack buffer ([`ni_sprintf`]) and removed from the map (`00e91fd0`).
// Translation incomplete: 00e923f0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_d3d_shader_factory_remove_shader(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    implementation: u32,
) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x28)) == 0 {
        return;
    }
    e.with_stack(0x104, |e, buffer| {
        ni_sprintf(
            e,
            buffer,
            0x104,
            Ptr::new(SHADER_KEY_FORMAT_01094C50),
            &[name, implementation],
        );
        let map = e.mem.u32(base.wrapping_add(0x28));
        e.call(REMOVE_LIBRARY_00E91FD0, &args![map, buffer]);
    });
}

/// The shader map's step to the next item (`00b57f20`, unnamed; `thiscall` with three words:
/// the addresses of the iterator, the key and the value).
const SHADER_MAP_STEP_00B57F20: u32 = 0x00b5_7f20;

// Translated from 00e92470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RestoreAllShaderRendererData` (engine map), `thiscall`: nothing
/// without a renderer (+0x24) or a shader map (+0x28). Else from the first item of the first
/// non-empty bucket (the bucket count read once), each shader the map's step (`00b57f20`, with
/// stack words for the iterator, the key and the value, the value cleared once first) answers
/// is restored unless its slot 0x48 answers true: `SetD3DRenderer` with the renderer, its slot
/// 0xd0 (`RecreateRendererData` in the Xbox PDB) and `RegisterD3DShader` (`00e752b0`). Ends
/// when the iterator is null.
pub fn ni_d3d_shader_factory_restore_all_shader_renderer_data(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base.wrapping_add(0x24)) == 0 {
        return;
    }
    let map = e.mem.u32(base.wrapping_add(0x28));
    if map == 0 {
        return;
    }
    e.with_stack(0xc, |e, frame| {
        let value = frame.addr();
        let iterator = value + 4;
        let key = value + 8;
        e.mem.set_u32(value, 0);
        let size = e.mem.u32(map.wrapping_add(4));
        let mut first = 0;
        if size != 0 {
            let buckets = e.mem.u32(map.wrapping_add(8));
            for bucket in 0..size {
                let item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
                if item != 0 {
                    first = item;
                    break;
                }
            }
        }
        e.mem.set_u32(iterator, first);
        while e.mem.u32(iterator) != 0 {
            let map = e.mem.u32(base.wrapping_add(0x28));
            e.call(SHADER_MAP_STEP_00B57F20, &args![map, iterator, key, value]);
            let shader = e.mem.u32(value);
            if shader != 0 && !e.vcall(shader, 0x48, &args![]).bool() {
                let renderer = e.mem.u32(base.wrapping_add(0x24));
                e.call(SET_D3D_RENDERER_00E818D0, &args![shader, renderer]);
                e.vcall(shader, 0xd0, &args![]);
                let renderer = e.mem.u32(base.wrapping_add(0x24));
                e.call(REGISTER_D3D_SHADER_00E752B0, &args![renderer, shader]);
            }
        }
    });
}

/// The map destructors' common end: the bucket array (+8) freed (`00aa10f0`), then with bit 0
/// of `flags` the map given back to the memory manager (`00aa4060`). Answers `this`.
fn map_free(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let buckets = e.mem.u32(this.addr().wrapping_add(8));
    e.call(FREE_00AA10F0, &args![buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

/// A map destructor level: its table stored and the map emptied (`00b601a0`).
fn map_level_down(e: &mut Engine, this: Ptr, table: u32) {
    e.mem.set_u32(this.addr(), table);
    e.call(MAP_REMOVE_ALL_00B601A0, &args![this]);
}

// Translated from 00e92610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,char const *,NiShader *>::scalar deleting
/// destructor` (engine map), `thiscall` with the flags: its table (`010f0bc0`), the map emptied
/// (`00b601a0`), then [`map_free`].
pub fn ni_t_map_base_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    map_level_down(e, this, SHADER_STRING_MAP_VTABLES[2]);
    map_free(e, this, flags)
}

// Translated from 00e92650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *,NiShader *>::scalar deleting destructor` (engine map),
/// `thiscall` with the flags: its table (`010f0be0`) and `00b601a0`, the `NiTMapBase` table
/// (`010f0bc0`) and `00b601a0` again, then [`map_free`].
pub fn ni_t_pointer_map_shader_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    map_level_down(e, this, SHADER_STRING_MAP_VTABLES[1]);
    map_level_down(e, this, SHADER_STRING_MAP_VTABLES[2]);
    map_free(e, this, flags)
}

// Translated from 00e926b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,char const *,NiPointer<NiShaderLibrary> >::
/// scalar deleting destructor` (engine map), `thiscall` with the flags: its table
/// (`010f0c20`), the map emptied (`00b601a0`), then [`map_free`].
pub fn ni_t_map_base_library_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    map_level_down(e, this, LIBRARY_STRING_MAP_VTABLES[2]);
    map_free(e, this, flags)
}

// Translated from 00e926f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<char const *,NiPointer<NiShaderLibrary> >::scalar deleting destructor`
/// (engine map), `thiscall` with the flags: its table (`010f0c40`) and `00b601a0`, the
/// `NiTMapBase` table (`010f0c20`) and `00b601a0` again, then [`map_free`].
pub fn ni_t_pointer_map_library_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    map_level_down(e, this, LIBRARY_STRING_MAP_VTABLES[1]);
    map_level_down(e, this, LIBRARY_STRING_MAP_VTABLES[2]);
    map_free(e, this, flags)
}

/// `NiD3DShaderInterface`'s RTTI record (`0126f75c`).
const SHADER_INTERFACE_RTTI_0126F75C: u32 = 0x0126_f75c;

// Translated from 00e92730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::IsDefaultImplementation` (engine map), `thiscall` (`this` not read)
/// with the shader: false for a null shader or one that is not an `NiD3DShaderInterface`
/// ([`ni_shader_dynamic_cast`] with `0126f75c`); else what its slot 0xe4
/// (`GetIsBestImplementation`) answers.
pub fn ni_d3d_shader_factory_is_default_implementation(
    e: &mut Engine,
    _this: Ptr,
    shader: u32,
) -> bool {
    if shader == 0 {
        return false;
    }
    let interface = ni_shader_dynamic_cast(e, Ptr::new(shader), SHADER_INTERFACE_RTTI_0126F75C);
    if interface == 0 {
        return false;
    }
    e.vcall(interface, 0xe4, &args![]).bool()
}

/// An item's `NiPointer` value (+8) given `value` as the code expands the assignment (unless it
/// holds it already: the old one let go, the new one stored and, when not null, counted up; a
/// null value ends it there), then the by-value `NiPointer` argument let go.
fn assign_item_value(e: &mut Engine, item: u32, value: u32) {
    let held = e.mem.u32(item.wrapping_add(8));
    if held != value {
        release_reference(e, held);
        e.mem.set_u32(item.wrapping_add(8), value);
        if value == 0 {
            return;
        }
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
    }
    release_reference(e, value);
}

/// The map's item allocator (`0043a010`, `thiscall` on the map's +0xc; the engine map names the
/// shared body `NiTPointerListBase<..>::AddHead`).
const NEW_ITEM_0043A010: u32 = 0x0043_a010;

// Translated from 00e927f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderfactory.cpp`): the `NiTPointerMap` set-at of a
/// map with `NiPointer` values (the library map), `thiscall` with the key and the value (an
/// `NiPointer` passed by value, let go at the end). The bucket is the map's hash of the key
/// (slot 4); an item whose key matches (slot 8) keeps its key unless the map copies keys (the
/// byte +0x10 clear: the key stored) and gets the value ([`assign_item_value`]). Else a new item
/// (`0043a010`) gets the key and a counted copy of the value through the map's slot 0xc, goes at
/// the head of the bucket, the count (+0xc) goes up and the argument is let go.
pub fn fn_00e927f0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    let bucket = e.vcall(base, 4, &args![key]).u32();
    let buckets = e.mem.u32(base.wrapping_add(8));
    let mut item = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(base, 8, &args![key, item_key]).u8() != 0 {
            if e.mem.u8(base.wrapping_add(0x10)) == 0 {
                e.mem.set_u32(item.wrapping_add(4), key);
            }
            assign_item_value(e, item, value);
            return;
        }
        item = e.mem.u32(item);
    }
    let count_at = base.wrapping_add(0xc);
    let item = e.call(NEW_ITEM_0043A010, &args![count_at]).u32();
    if value != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![value.wrapping_add(4)]);
    }
    e.vcall(base, 0xc, &args![item, key, value]);
    let buckets = e.mem.u32(base.wrapping_add(8));
    let head = e.mem.u32(buckets.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(item, head);
    let buckets = e.mem.u32(base.wrapping_add(8));
    e.mem
        .set_u32(buckets.wrapping_add(bucket.wrapping_mul(4)), item);
    let count = e.mem.u32(count_at);
    e.mem.set_u32(count_at, count.wrapping_add(1));
    release_reference(e, value);
}

/// The block allocation `00aa1070` (`cdecl`, the size) and `strcpy_s` (`00ec65a6`, `cdecl`:
/// destination, size, source).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const STRCPY_S_00EC65A6: u32 = 0x00ec_65a6;

// Translated from 00e92910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nid3dshaderfactory.cpp`; the set-value slot of four string
/// map vtables), `thiscall` with the item, the key and the value (an `NiPointer` passed by
/// value). A map that copies keys (the byte +0x10 set) stores a copy of the key in a new block
/// of its length plus one (`00aa1070`, then `strcpy_s`); else the key itself goes to the item's
/// +4. Then the item gets the value ([`assign_item_value`]).
pub fn fn_00e92910(e: &mut Engine, this: Ptr, item: u32, key: u32, value: u32) {
    if e.mem.u8(this.addr().wrapping_add(0x10)) != 0 {
        let mut length = 0u32;
        while e.mem.u8(key.wrapping_add(length)) != 0 {
            length += 1;
        }
        let size = length.wrapping_add(1);
        let copy = e.call(ALLOCATE_00AA1070, &args![size]).u32();
        e.mem.set_u32(item.wrapping_add(4), copy);
        e.call(STRCPY_S_00EC65A6, &args![copy, size, key]);
    } else {
        e.mem.set_u32(item.wrapping_add(4), key);
    }
    assign_item_value(e, item, value);
}

/// The shader map's set-at (`00a642a0`, `NiTStringTemplateMap<..,NiShader *>::SetAt` in the
/// engine map; `thiscall`: the key, the shader).
const SHADER_MAP_SET_AT_00A642A0: u32 = 0x00a6_42a0;

// Translated from 00e929c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::InsertShader` (engine map), `thiscall` with the shader and the
/// implementation: for a shader, the key `"%s%d"` of its name (the word `00a62660` answers the
/// address of) and the implementation is written to a 0x104-byte stack buffer ([`ni_sprintf`])
/// and, with a shader map (+0x28), the shader is set at it (`00a642a0`).
// Translation incomplete: 00e929c0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ni_d3d_shader_factory_insert_shader(
    e: &mut Engine,
    this: Ptr,
    shader: u32,
    implementation: u32,
) {
    if shader == 0 {
        return;
    }
    let base = this.addr();
    e.with_stack(0x104, |e, buffer| {
        let name_at = e.call(NAME_ADDRESS_00A62660, &args![shader]).u32();
        let name = e.mem.u32(name_at);
        ni_sprintf(
            e,
            buffer,
            0x104,
            Ptr::new(SHADER_KEY_FORMAT_01094C50),
            &[name, implementation],
        );
        let map = e.mem.u32(base.wrapping_add(0x28));
        if map != 0 {
            e.call(SHADER_MAP_SET_AT_00A642A0, &args![map, buffer, shader]);
        }
    });
}

// Translated from 00e92a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::InsertLibrary` (engine map), `thiscall` with the library: for a library,
/// a counted `NiPointer` copy of it (`InterlockedIncrement`) goes with its name
/// (`NiShaderLibrary::GetName`) to the library map's (+0x30) set-at ([`fn_00e927f0`]).
pub fn ni_d3d_shader_factory_insert_library(e: &mut Engine, this: Ptr, library: u32) {
    if library == 0 {
        return;
    }
    e.call(
        IMPORT_INTERLOCKED_INCREMENT,
        &args![library.wrapping_add(4)],
    );
    let name = e.call(LIBRARY_GET_NAME_00D47A30, &args![library]).u32();
    let map = e.mem.u32(this.addr().wrapping_add(0x30));
    fn_00e927f0(e, Ptr::new(map), name, library);
}

// Translated from 00e92a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::FindLibrary` (engine map), `thiscall` with the name: looks it up in the
/// library map (+0x30, [`fn_00e92760`] into a stack `NiPointer` cleared first), lets the
/// `NiPointer` go and answers the library when found, else 0.
pub fn ni_d3d_shader_factory_find_library(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let map = e.mem.u32(this.addr().wrapping_add(0x30));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let found = fn_00e92760(e, Ptr::new(map), name, slot);
        let library = e.mem.u32(slot.addr());
        release_reference(e, library);
        if found {
            library
        } else {
            0
        }
    })
}

// Translated from 00e92ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::RemoveLibrary` (engine map), `thiscall` with the name: looks it up in
/// the library map (+0x30, [`fn_00e92760`] into a stack `NiPointer` cleared first). Found with a
/// reference count (+4) of 2 (the map's and the stack's): removed from the map (`00e91fd0`) and
/// the stack reference let go (no null check). Otherwise the stack reference is let go when set.
pub fn ni_d3d_shader_factory_remove_library(e: &mut Engine, this: Ptr, name: u32) {
    let base = this.addr();
    let map = e.mem.u32(base.wrapping_add(0x30));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let found = fn_00e92760(e, Ptr::new(map), name, slot);
        let library = e.mem.u32(slot.addr());
        if found && e.mem.u32(library.wrapping_add(4)) == 2 {
            let map = e.mem.u32(base.wrapping_add(0x30));
            e.call(REMOVE_LIBRARY_00E91FD0, &args![map, name]);
            let left = e
                .call(
                    IMPORT_INTERLOCKED_DECREMENT,
                    &args![library.wrapping_add(4)],
                )
                .u32();
            if left == 0 {
                e.vcall(library, 4, &args![]);
            }
            return;
        }
        release_reference(e, library);
    });
}

// Translated from 00e92be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderFactory::GetNextLibrary` (engine map), `thiscall`: 0 without the library map
/// (+0x30) or the iterator (+0x2c); else the next value from the iterator (the same step as
/// [`ni_d3d_shader_factory_get_first_library`]).
pub fn ni_d3d_shader_factory_get_next_library(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let map = e.mem.u32(base.wrapping_add(0x30));
    if map == 0 || e.mem.u32(base.wrapping_add(0x2c)) == 0 {
        return 0;
    }
    next_library(e, map, base.wrapping_add(0x2c))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_1ea0,
            ni_d3d_shader_factory_purge_all_shader_renderer_data(Ptr)
        ),
        entry!(
            0x00e9_1eb0,
            ni_d3d_shader_factory_get_minor_pixel_shader_version(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1ec0,
            ni_d3d_shader_factory_create_pixel_shader_version(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00e9_1ee0,
            ni_d3d_shader_factory_get_major_pixel_shader_version(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1ef0,
            ni_d3d_shader_factory_create_vertex_shader_version(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00e9_1f20,
            ni_d3d_shader_factory_get_first_program_directory(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1f40,
            ni_d3d_shader_factory_get_next_program_directory(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1f60,
            ni_d3d_shader_factory_add_program_directory(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1f80,
            ni_d3d_shader_factory_remove_program_directory(Ptr, u32) -> u32
        ),
        entry!(
            0x00e9_1fa0,
            ni_d3d_shader_factory_remove_all_program_directories(Ptr) -> u32
        ),
        entry!(0x00e9_1fc0, ni_d3d_shader_factory_get_renderer_string(Ptr) -> u32),
        entry!(
            0x00e9_20a0,
            ni_d3d_shader_factory_retrieve_shader(Ptr, u32, u32, u8) -> u32
        ),
        entry!(
            0x00e9_2270,
            ni_d3d_shader_factory_release_shader_from_library(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x00e9_2330,
            ni_d3d_shader_factory_release_shader_from_library_ov2(Ptr, u32) -> bool
        ),
        entry!(
            0x00e9_23f0,
            ni_d3d_shader_factory_remove_shader(Ptr, u32, u32)
        ),
        entry!(
            0x00e9_2470,
            ni_d3d_shader_factory_restore_all_shader_renderer_data(Ptr)
        ),
        entry!(
            0x00e9_2610,
            ni_t_map_base_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_2650,
            ni_t_pointer_map_shader_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_26b0,
            ni_t_map_base_library_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_26f0,
            ni_t_pointer_map_library_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00e9_2730,
            ni_d3d_shader_factory_is_default_implementation(Ptr, u32) -> bool
        ),
        entry!(0x00e9_27f0, fn_00e927f0(Ptr, u32, u32)),
        entry!(0x00e9_2910, fn_00e92910(Ptr, u32, u32, u32)),
        entry!(
            0x00e9_29c0,
            ni_d3d_shader_factory_insert_shader(Ptr, u32, u32)
        ),
        entry!(0x00e9_2a40, ni_d3d_shader_factory_insert_library(Ptr, u32)),
        entry!(0x00e9_2a70, ni_d3d_shader_factory_find_library(Ptr, u32) -> u32),
        entry!(0x00e9_2ae0, ni_d3d_shader_factory_remove_library(Ptr, u32)),
        entry!(0x00e9_2be0, ni_d3d_shader_factory_get_next_library(Ptr) -> u32),
        entry!(0x00e9_1f10, fn_00e91f10() -> u32),
        entry!(0x00e6_e830, ni_t_pointer_map_delete_item(Ptr, Ptr)),
        entry!(0x00e9_2cf0, fn_00e92cf0(Ptr) -> Ptr),
        entry!(0x00e9_2e60, ni_d3d_shader_factory_sdm_init()),
        (0x00a3_05b0, ni_sprintf_uniform as AbiFn),
        entry!(0x00a4_d5f0, ni_t_map_base_set_value(Ptr, Ptr, u32, u32)),
        entry!(0x00b4_f580, ni_shader_dynamic_cast(Ptr, u32) -> u32),
        entry!(0x00e9_1e60, ni_d3d_shader_factory_set_renderer(Ptr, u32)),
        entry!(0x00e9_2080, fn_00e92080()),
        entry!(0x00e9_21d0, ni_d3d_shader_factory_find_shader(Ptr, u32, u32) -> u32),
        entry!(0x00e9_2460, ni_d3d_shader_factory_remove_all_shaders(Ptr)),
        entry!(0x00e9_2520, fn_00e92520(Ptr<NiTPointerMap>)),
        entry!(0x00e9_2590, fn_00e92590(Ptr, u32, u32) -> bool),
        entry!(0x00e9_25a0, fn_00e925a0(Ptr<NiTPointerMap>)),
        entry!(
            0x00e9_2690,
            ni_t_string_template_map_shader_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00e9_2760,
            fn_00e92760(Ptr<NiTPointerMap>, u32, Ptr) -> bool
        ),
        entry!(0x00e9_2b50, ni_d3d_shader_factory_get_first_library(Ptr) -> u32),
        entry!(0x00e9_2c40, ni_d3d_shader_factory_remove_all_libraries(Ptr)),
        entry!(
            0x00e9_2cd0,
            ni_t_string_template_map_library_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            )
                -> Ptr<NiTPointerMap>
        ),
        entry!(0x00e9_2dc0, fn_00e92dc0(Ptr)),
        entry!(0x00e9_2e30, ni_d3d_shader_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0126_f000, 0x2000);
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INITIALIZE_CRITICAL_SECTION, |_, _| 1u32.into_ret());
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(EMPTY_CALL, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructor_builds_two_tables() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0xcc; 0x100]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_2cf0, &args![this]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, INITIALIZE_CRITICAL_SECTION),
            vec![vec![this + 0x80]]
        );
        assert_eq!(e.mem.u32(this), 0x010f_0cc4);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        let first = e.mem.u32(this + 0x28);
        assert_eq!(e.mem.u32(first), 0x010f_0c80);
        assert_eq!(e.mem.u32(first + 4), 0x3b);
        assert_eq!(e.mem.u32(first + 0xc), 0);
        assert_eq!(e.mem.u8(first + 0x10), 1);
        let slots = e.mem.u32(first + 8);
        assert_eq!(e.mem.bytes(slots, 0xec), vec![0; 0xec]);
        let second = e.mem.u32(this + 0x30);
        assert_eq!(e.mem.u32(second), 0x010f_0ca0);
        assert_eq!(e.mem.u32(second + 4), 0x25);
        assert_eq!(e.mem.u8(second + 0x10), 0);
        let allocations = calls_to(&e, ALLOCATE_BLOCK);
        assert_eq!(allocations, vec![vec![0xec], vec![0x94]]);
    }

    #[test]
    fn constructor_stores_null_for_a_failed_table() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x100);
        e.call(0x00e9_2cf0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }

    #[test]
    fn start_up_makes_the_factory_once() {
        let mut e = engine();
        e.call(0x00e9_2e60, &args![]);
        let factory = e.mem.u32(FACTORY_SLOT);
        assert_ne!(factory, 0);
        assert_eq!(e.mem.u32(FACTORY_ALIAS_SLOT), factory);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), factory);
        assert!(calls_to(&e, ALLOCATE).is_empty());
        assert!(calls_to(&e, EMPTY_CALL).is_empty());
    }

    #[test]
    fn start_up_with_a_failed_allocation_calls_the_empty_function() {
        let mut e = engine();
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00e9_2e60, &args![]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), 0);
        assert_eq!(calls_to(&e, EMPTY_CALL), vec![vec![0x010f_0d40]]);
    }
}

#[cfg(test)]
mod delete_item_tests {
    use super::*;

    const VALUE_DELETE: u32 = 0x0e20_0000;

    fn engine(left: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.register_double(IMPORT_INTERLOCKED_DECREMENT, move |_, _| left.into_ret());
        e.register(LOCK_ACQUIRE_0040FBF0, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            let count = e.mem.u32(a[0] + 4) + 1;
            e.mem.set_u32(a[0] + 4, count);
            Ret::default()
        });
        e.register(VALUE_DELETE, |_, _| Ret::default());
        e.put_vtable(0x0e10_0000, &[0, VALUE_DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn delete_item_releases_the_value_and_frees_the_item() {
        let mut e = engine(0);
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x0e10_0000);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item, 0x1234);
        e.mem.set_u32(item + 4, 0x5678);
        e.mem.set_u32(item + 8, value);
        e.set_global::<u32>(FREE_LIST_HEAD_011C5F58, 0x4444);
        e.call(0x00e6_e830, &args![0x9999u32, item]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![value + 4]]
        );
        assert_eq!(calls(&e, VALUE_DELETE), vec![vec![value]]);
        assert_eq!(
            calls(&e, LOCK_ACQUIRE_0040FBF0),
            vec![vec![FREE_LIST_LOCK_011C5F80, 0]]
        );
        assert_eq!(e.mem.u32(item), 0x4444);
        assert_eq!(e.mem.u32(item + 4), 0);
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_HEAD_011C5F58), item);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80), 0);
    }

    #[test]
    fn delete_item_keeps_a_referenced_value_and_a_held_lock() {
        let mut e = engine(1);
        e.set_global::<u32>(FREE_LIST_LOCK_011C5F80 + 4, 1);
        let value = e.mem.alloc(8);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 8, value);
        e.call(0x00e6_e830, &args![0u32, item]);
        assert!(calls(&e, VALUE_DELETE).is_empty());
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80 + 4), 1);
        assert_eq!(e.global::<u32>(FREE_LIST_LOCK_011C5F80), 0x77);
        // Without a value nothing is released.
        let mut e = engine(0);
        let item = e.mem.alloc(0xc);
        e.call(0x00e6_e830, &args![0u32, item]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.global::<u32>(FREE_LIST_HEAD_011C5F58), item);
    }
}

#[cfg(test)]
mod b000w_tests {
    use super::*;

    #[test]
    fn fn_00e91f10_reads_the_factory_word() {
        let mut e = Engine::new();
        e.map(FACTORY_SLOT, 4);
        e.mem.set_u32(FACTORY_SLOT, 0x4321);
        assert_eq!(e.call(0x00e9_1f10, &args![]).u32(), 0x4321);
    }
}

#[cfg(test)]
mod b051_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const OBJECT_VTABLE: u32 = 0x6000;
    const DELETE: u32 = 0x7004;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for no_op in [
            FREE_00AA10F0,
            FREE_00AA1460,
            MEMORY_MANAGER_FREE,
            MAP_REMOVE_ALL_00B601A0,
            REMOVE_LIBRARY_00E91FD0,
            IMPORT_DELETE_CRITICAL_SECTION,
            NI_SHADER_FACTORY_DOWN_00AA05A0,
            DELETE,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        // The interlocked doubles move the count they are given and answer it.
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.put_vtable(OBJECT_VTABLE, &[0, DELETE]);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn ni_sprintf_passes_the_arguments_and_ends_the_buffer() {
        let mut e = engine();
        e.register(VSPRINTF_S, |e, a| {
            let first = e.mem.u32(a[3]);
            let second = e.mem.u32(a[3] + 4);
            e.mem.write(a[0], &[b'x'; 8]);
            (first + second).into_ret()
        });
        let buffer = e.mem.alloc(8);
        assert_eq!(e.call(0x00a3_05b0, &args![buffer, 0u32, 0x99u32]).i32(), 1);
        assert!(calls(&e, VSPRINTF_S).is_empty());
        let result = e
            .call(0x00a3_05b0, &args![buffer, 8u32, 0x99u32, 3u32, 4u32])
            .i32();
        assert_eq!(result, 7);
        let call = &calls(&e, VSPRINTF_S)[0];
        assert_eq!(&call[..3], &[buffer, 8, 0x99]);
        assert_eq!(e.mem.u8(buffer + 6), b'x');
        assert_eq!(e.mem.u8(buffer + 7), 0);
    }

    #[test]
    fn ni_t_map_base_set_value_assigns_and_lets_go_of_the_argument() {
        let mut e = engine();
        let item = e.mem.alloc(0xc);
        let old = object(&mut e, 1);
        let new = object(&mut e, 1);
        e.mem.set_u32(item + 8, old);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x55u32, new]);
        assert_eq!(e.mem.u32(item + 4), 0x55);
        assert_eq!(e.mem.u32(item + 8), new);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        // Counted up by the item, down by the argument.
        assert_eq!(e.mem.u32(new + 4), 1);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x56u32, new]);
        assert_eq!(e.mem.u32(new + 4), 0);
        assert_eq!(calls(&e, DELETE), vec![vec![old], vec![new]]);
        e.call(0x00a4_d5f0, &args![0u32, item, 0x57u32, 0u32]);
        assert_eq!(e.mem.u32(item + 8), 0);
    }

    #[test]
    fn ni_shader_dynamic_cast_walks_the_bases() {
        let mut e = engine();
        let records = e.mem.alloc(0x18);
        e.mem.set_u32(records + 4, records + 8);
        e.mem.set_u32(records + 0xc, 0);
        e.register_double(0x7008, move |_, _| records.into_ret());
        let shader = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0, 0, 0x7008]);
        e.mem.set_u32(shader, 0x6100);
        assert_eq!(
            e.call(0x00b4_f580, &args![shader, records + 8]).u32(),
            shader
        );
        assert_eq!(e.call(0x00b4_f580, &args![shader, records]).u32(), shader);
        assert_eq!(e.call(0x00b4_f580, &args![shader, 0x1234u32]).u32(), 0);
    }

    #[test]
    fn set_renderer_hands_the_renderer_to_every_shader() {
        let mut e = engine();
        let shaders = e.mem.alloc(0x10);
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0, 0, 0, 0, 0, 0, 0, 0x701c]);
        e.mem.set_u32(first, 0x6100);
        e.mem.set_u32(second, 0x6100);
        e.mem.set_u32(shaders, first);
        e.mem.set_u32(shaders + 4, second);
        e.register(0x701c, |_, _| Ret::default());
        e.register_double(0x7038, move |e, _| {
            e.mem.set_u32(shaders + 0xc, 1);
            e.mem.u32(shaders).into_ret()
        });
        e.register_double(0x703c, move |e, _| {
            let at = e.mem.u32(shaders + 0xc);
            e.mem.set_u32(shaders + 0xc, at + 1);
            e.mem.u32(shaders + 4 * at).into_ret()
        });
        let mut slots = vec![0u32; 0x10];
        slots[0xe] = 0x7038;
        slots[0xf] = 0x703c;
        e.put_vtable(0x6200, &slots);
        let factory = e.mem.alloc(0x28);
        e.mem.set_u32(factory, 0x6200);
        e.call(0x00e9_1e60, &args![factory, 0x99u32]);
        assert_eq!(e.mem.u32(factory + 0x24), 0x99);
        assert_eq!(
            calls(&e, 0x701c),
            vec![vec![first, 0x99], vec![second, 0x99]]
        );
        e.call(0x00e9_1e60, &args![factory, 0x99u32]);
        assert_eq!(calls(&e, 0x7038).len(), 1);
    }

    #[test]
    fn fn_00e92080_deletes_the_factory() {
        let mut e = engine();
        e.call(0x00e9_2080, &args![]);
        e.register(0x7000, |_, _| Ret::default());
        let factory = e.mem.alloc(4);
        e.put_vtable(0x6100, &[0x7000]);
        e.mem.set_u32(factory, 0x6100);
        e.mem.set_u32(FACTORY_SLOT, factory);
        e.call(0x00e9_2080, &args![]);
        assert_eq!(calls(&e, 0x7000), vec![vec![factory, 1]]);
        assert_eq!(e.mem.u32(FACTORY_SLOT), 0);
    }

    #[test]
    fn find_shader_looks_the_key_up() {
        let mut e = engine();
        let hit = e.mem.alloc(4);
        e.register(VSPRINTF_S, |_, _| 0u32.into_ret());
        e.register_double(MAP_LOOKUP_00A53C30, move |e, a| {
            e.mem.set_u32(a[2], 0x4321);
            e.mem.u32(hit).into_ret()
        });
        let factory = e.mem.alloc(0x34);
        assert_eq!(e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(), 0);
        assert!(calls(&e, VSPRINTF_S).is_empty());
        e.mem.set_u32(factory + 0x28, 0x5000);
        assert_eq!(e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(), 0);
        e.mem.set_u32(hit, 0x101);
        assert_eq!(
            e.call(0x00e9_21d0, &args![factory, 0x10u32, 3u32]).u32(),
            0x4321
        );
        let format = &calls(&e, VSPRINTF_S)[0];
        assert_eq!(format[1..3], [0x104, SHADER_KEY_FORMAT_01094C50]);
        let lookup = &calls(&e, MAP_LOOKUP_00A53C30)[0];
        assert_eq!(lookup[0], 0x5000);
        assert_eq!(lookup[1], format[0]);
        assert_eq!(lookup[2] + 4, format[0]);
    }

    #[test]
    fn remove_all_shaders_empties_a_present_map() {
        let mut e = engine();
        let factory = e.mem.alloc(0x34);
        e.call(0x00e9_2460, &args![factory]);
        assert!(calls(&e, MAP_REMOVE_ALL_00B601A0).is_empty());
        e.mem.set_u32(factory + 0x28, 0x5000);
        e.call(0x00e9_2460, &args![factory]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![0x5000]]);
    }

    /// A string map of three buckets: items with keys 0x11, 0x12 in bucket 0 and 0x13 in 2.
    fn string_map(e: &mut Engine, copies: u8) -> u32 {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(0xc);
        let items = [e.mem.alloc(0xc), e.mem.alloc(0xc), e.mem.alloc(0xc)];
        e.mem.set_u32(items[0], items[1]);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(item + 4, 0x11 + i as u32);
        }
        e.mem.set_u32(buckets, items[0]);
        e.mem.set_u32(buckets + 8, items[2]);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, copies);
        map
    }

    #[test]
    fn string_map_destructors_free_copied_keys() {
        let mut e = engine();
        let map = string_map(&mut e, 1);
        let buckets = e.mem.u32(map + 8);
        e.call(0x00e9_2520, &args![map]);
        assert_eq!(
            calls(&e, FREE_00AA10F0),
            vec![vec![0x11], vec![0x12], vec![0x13], vec![buckets]]
        );
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL_00B601A0),
            vec![vec![map], vec![map]]
        );
        assert_eq!(e.mem.u32(map), 0x010f_0bc0);
        e.call_log = Some(vec![]);
        let map = string_map(&mut e, 0);
        let buckets = e.mem.u32(map + 8);
        e.call(0x00e9_25a0, &args![map]);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![buckets]]);
        assert_eq!(e.mem.u32(map), 0x010f_0c20);
    }

    #[test]
    fn string_map_deleting_destructors_free_with_bit_0() {
        let mut e = engine();
        for addr in [0x00e9_2690u32, 0x00e9_2cd0] {
            let map = string_map(&mut e, 0);
            assert_eq!(e.call(addr, &args![map, 0u32]).u32(), map);
            assert!(calls(&e, MEMORY_MANAGER_FREE).is_empty());
            e.call(addr, &args![map, 1u32]);
            assert_eq!(
                calls(&e, MEMORY_MANAGER_FREE),
                vec![vec![MEMORY_MANAGER_OBJECT, map]]
            );
            e.call_log = Some(vec![]);
        }
    }

    #[test]
    fn fn_00e92590_compares_the_words() {
        let mut e = engine();
        assert!(e.call(0x00e9_2590, &args![0u32, 5u32, 5u32]).bool());
        assert!(!e.call(0x00e9_2590, &args![0u32, 5u32, 6u32]).bool());
    }

    #[test]
    fn fn_00e92760_finds_the_value_and_assigns_it() {
        let mut e = engine();
        e.register(0x7104, |_, a| (a[1] % 3).into_ret());
        e.register(0x7108, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.put_vtable(0x6100, &[0, 0x7104, 0x7108]);
        let map = string_map(&mut e, 0);
        e.mem.set_u32(map, 0x6100);
        let value = object(&mut e, 1);
        let buckets = e.mem.u32(map + 8);
        let second = e.mem.u32(e.mem.u32(buckets));
        e.mem.set_u32(second + 8, value);
        let result = e.mem.alloc(4);
        let old = object(&mut e, 1);
        e.mem.set_u32(result, old);
        // Key 0x12 hashes to bucket 0 (0x12 % 3) and is its second item.
        assert!(e.call(0x00e9_2760, &args![map, 0x12u32, result]).bool());
        assert_eq!(e.mem.u32(result), value);
        assert_eq!(e.mem.u32(value + 4), 2);
        assert_eq!(calls(&e, DELETE), vec![vec![old]]);
        assert_eq!(calls(&e, 0x7108).len(), 2);
        // Held already: nothing moves.
        assert!(e.call(0x00e9_2760, &args![map, 0x12u32, result]).bool());
        assert_eq!(e.mem.u32(value + 4), 2);
        // Key 0x15 hashes to bucket 0 but no item matches.
        assert!(!e.call(0x00e9_2760, &args![map, 0x15u32, result]).bool());
    }

    /// A factory whose library map iterates `libraries` (the step double hands them out).
    fn library_factory(e: &mut Engine, libraries: Vec<u32>) -> u32 {
        let factory = e.mem.alloc(0x34);
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(buckets + 4, 0x9990);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(factory + 0x30, map);
        let mut next = 0usize;
        e.register_double(MAP_STEP_00E6E630, move |e, a| {
            let library = libraries[next];
            next += 1;
            e.mem.set_u32(a[3], library);
            if library != 0 {
                let count = e.mem.u32(library + 4) + 1;
                e.mem.set_u32(library + 4, count);
            }
            let more = if next < libraries.len() { 0x9991 } else { 0 };
            e.mem.set_u32(a[1], more);
            Ret::default()
        });
        e.register(LIBRARY_GET_NAME_00D47A30, |_, a| (a[0] + 1).into_ret());
        factory
    }

    #[test]
    fn get_first_library_starts_the_iterator() {
        let mut e = engine();
        let library = object(&mut e, 1);
        let factory = library_factory(&mut e, vec![library]);
        let map = e.mem.u32(factory + 0x30);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), library);
        let step = &calls(&e, MAP_STEP_00E6E630)[0];
        assert_eq!(step[..2], [map, factory + 0x2c]);
        assert_eq!(e.mem.u32(library + 4), 1);
        assert!(calls(&e, DELETE).is_empty());
        // An empty map: the iterator is cleared and nothing is taken.
        let buckets = e.mem.u32(map + 8);
        e.mem.set_u32(buckets + 4, 0);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), 0);
        assert_eq!(e.mem.u32(factory + 0x2c), 0);
        assert_eq!(calls(&e, MAP_STEP_00E6E630).len(), 1);
        e.mem.set_u32(factory + 0x30, 0);
        assert_eq!(e.call(0x00e9_2b50, &args![factory]).u32(), 0);
    }

    #[test]
    fn remove_all_libraries_removes_each_by_name() {
        let mut e = engine();
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        let factory = library_factory(&mut e, vec![first, second]);
        let map = e.mem.u32(factory + 0x30);
        e.call(0x00e9_2c40, &args![factory]);
        assert_eq!(
            calls(&e, REMOVE_LIBRARY_00E91FD0),
            vec![vec![map, first + 1], vec![map, second + 1]]
        );
        assert_eq!(calls(&e, MAP_STEP_00E6E630).len(), 2);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0), vec![vec![map]]);
        let factory = e.mem.alloc(0x34);
        e.call(0x00e9_2c40, &args![factory]);
        assert_eq!(calls(&e, MAP_REMOVE_ALL_00B601A0).len(), 1);
    }

    #[test]
    fn factory_destructors_take_everything_down() {
        let mut e = engine();
        e.register(0x7000, |_, _| Ret::default());
        e.put_vtable(0x6100, &[0x7000]);
        let factory = library_factory(&mut e, vec![0]);
        let libraries = e.mem.u32(factory + 0x30);
        let buckets = e.mem.u32(libraries + 8);
        e.mem.set_u32(buckets + 4, 0);
        e.mem.set_u32(libraries, 0x6100);
        let shaders = e.mem.alloc(4);
        e.mem.set_u32(shaders, 0x6100);
        e.mem.set_u32(factory + 0x28, shaders);
        e.mem.set_u32(factory + 0x24, 0x99);
        assert_eq!(e.call(0x00e9_2e30, &args![factory, 1u32]).u32(), factory);
        assert_eq!(e.mem.u32(factory), 0x010f_0cc4);
        assert_eq!(
            calls(&e, MAP_REMOVE_ALL_00B601A0),
            vec![vec![shaders], vec![libraries]]
        );
        assert_eq!(
            calls(&e, 0x7000),
            vec![vec![shaders, 1], vec![libraries, 1]]
        );
        for offset in [0x24u32, 0x28, 0x30] {
            assert_eq!(e.mem.u32(factory + offset), 0);
        }
        assert_eq!(
            calls(&e, IMPORT_DELETE_CRITICAL_SECTION),
            vec![vec![factory + 0x80]]
        );
        assert_eq!(
            calls(&e, NI_SHADER_FACTORY_DOWN_00AA05A0),
            vec![vec![factory]]
        );
        assert_eq!(calls(&e, FREE_00AA1460), vec![vec![factory, 0x100]]);
    }
}

#[cfg(test)]
mod n008_tests {
    use super::*;

    /// The factory's vtable; its slots are doubles at `FACTORY_SLOTS + slot`.
    const FACTORY_VTABLE: u32 = 0x0e00_0000;
    const FACTORY_SLOTS: u32 = 0x0e01_0000;
    /// A library's (or shader's) vtable; slots at `OBJECT_SLOTS + slot`.
    const OBJECT_VTABLE: u32 = 0x0e02_0000;
    const OBJECT_SLOTS: u32 = 0x0e03_0000;
    /// A map's vtable (hash slot 4, compare slot 8, set-value slot 0xc).
    const MAP_VTABLE: u32 = 0x0e04_0000;
    const MAP_HASH: u32 = 0x0e05_0004;
    const MAP_COMPARE: u32 = 0x0e05_0008;
    const MAP_SET_VALUE: u32 = 0x0e05_000c;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011f_6000, 0x1000);
        let slots: Vec<u32> = (0..0x40).map(|i| FACTORY_SLOTS + 4 * i).collect();
        e.put_vtable(FACTORY_VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x40).map(|i| OBJECT_SLOTS + 4 * i).collect();
        e.put_vtable(OBJECT_VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        e.put_vtable(MAP_VTABLE, &[0, MAP_HASH, MAP_COMPARE, MAP_SET_VALUE]);
        e.register(MAP_HASH, |_, a| (a[1] & 1).into_ret());
        e.register(MAP_COMPARE, |_, a| u32::from(a[1] == a[2]).into_ret());
        e.register(MAP_SET_VALUE, |_, _| Ret::default());
        for no_op in [
            REPORT_ERROR_00AA0020,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            REGISTER_D3D_SHADER_00E752B0,
            SET_D3D_RENDERER_00E818D0,
            REMOVE_LIBRARY_00E91FD0,
            MAP_REMOVE_ALL_00B601A0,
            FREE_00AA10F0,
            MEMORY_MANAGER_FREE,
            SHADER_MAP_SET_AT_00A642A0,
            STRCPY_S_00EC65A6,
            PURGE_ALL_D3D_SHADERS_00E75380,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        // The key text "%s%d" writes: the name word and the number.
        e.register(VSPRINTF_S, |e, a| {
            let name = e.mem.u32(a[3]);
            let number = e.mem.u32(a[3] + 4);
            e.mem.set_u32(a[0], name);
            e.mem.set_u32(a[0] + 4, number);
            8u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn factory(e: &mut Engine) -> u32 {
        let factory = e.mem.alloc(0x100);
        e.mem.set_u32(factory, FACTORY_VTABLE);
        factory
    }

    fn object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, OBJECT_VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// A map of two buckets whose vtable is `MAP_VTABLE`, the copy byte `copies`.
    fn map(e: &mut Engine, copies: u8) -> u32 {
        let map = e.mem.alloc(0x14);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map, MAP_VTABLE);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u8(map + 0x10, copies);
        map
    }

    #[test]
    fn purge_all_shader_renderer_data_needs_a_renderer() {
        let mut e = engine();
        let factory = factory(&mut e);
        e.call(0x00e9_1ea0, &args![factory]);
        assert!(calls(&e, PURGE_ALL_D3D_SHADERS_00E75380).is_empty());
        e.mem.set_u32(factory + 0x24, 0x0e70_0000);
        e.call(0x00e9_1ea0, &args![factory]);
        assert_eq!(
            calls(&e, PURGE_ALL_D3D_SHADERS_00E75380),
            vec![vec![0x0e70_0000]]
        );
    }

    #[test]
    fn pixel_shader_version_parts() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00e9_1eb0, &args![0u32, 0xffff_0314u32]).u32(),
            0x14
        );
        assert_eq!(e.call(0x00e9_1ee0, &args![0u32, 0xffff_0314u32]).u32(), 3);
    }

    #[test]
    fn shader_versions_are_built_with_their_tags() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00e9_1ec0, &args![0u32, 3u32, 1u32]).u32(),
            0xffff_0301
        );
        assert_eq!(
            e.call(0x00e9_1ef0, &args![0u32, 3u32, 0u32]).u32(),
            0xfffe_0300
        );
    }

    #[test]
    fn program_directory_calls_go_to_the_program_factory() {
        for (addr, target) in [
            (0x00e9_1f20u32, FIRST_PROGRAM_DIRECTORY_00E92FE0),
            (0x00e9_1f40, NEXT_PROGRAM_DIRECTORY_00E93000),
            (0x00e9_1f60, ADD_PROGRAM_DIRECTORY_00E93960),
            (0x00e9_1f80, REMOVE_PROGRAM_DIRECTORY_00E93BB0),
        ] {
            let mut e = engine();
            e.register(PROGRAM_FACTORY_INSTANCE_00E92EB0, |_, _| 0u32.into_ret());
            e.register(target, |_, _| 0x55u32.into_ret());
            assert_eq!(e.call(addr, &args![0u32, 0x99u32]).u32(), 0);
            assert!(calls(&e, target).is_empty());
            e.register(PROGRAM_FACTORY_INSTANCE_00E92EB0, |_, _| {
                0x0e80_0000u32.into_ret()
            });
            assert_eq!(e.call(addr, &args![0u32, 0x99u32]).u32(), 0x55);
            assert_eq!(calls(&e, target), vec![vec![0x0e80_0000, 0x99]]);
        }
    }

    #[test]
    fn remove_all_program_directories_goes_to_the_program_factory() {
        let mut e = engine();
        e.register(PROGRAM_FACTORY_INSTANCE_00E92EB0, |_, _| {
            0x0e80_0000u32.into_ret()
        });
        e.register(REMOVE_ALL_PROGRAM_DIRECTORIES_00E93CC0, |_, _| {
            Ret::default()
        });
        e.call(0x00e9_1fa0, &args![0u32]);
        assert_eq!(
            calls(&e, REMOVE_ALL_PROGRAM_DIRECTORIES_00E93CC0),
            vec![vec![0x0e80_0000]]
        );
    }

    #[test]
    fn get_renderer_string_answers_the_text() {
        let mut e = engine();
        assert_eq!(e.call(0x00e9_1fc0, &args![0u32]).u32(), 0x010f_005c);
    }

    #[test]
    fn retrieve_shader_asks_the_libraries() {
        let mut e = engine();
        let factory = factory(&mut e);
        // No renderer: the log hears of it.
        let log = object(&mut e, 0);
        e.mem.set_u32(LOG_011F6388, log);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, b'a');
        assert_eq!(
            e.call(0x00e9_20a0, &args![factory, name, 2u32, 1u32]).u32(),
            0
        );
        assert_eq!(
            calls(&e, OBJECT_SLOTS + 0x14),
            vec![vec![log, 4, NO_RENDERER_010F0B84, 0, 0, 0]]
        );
        // No name: reported.
        e.mem.set_u32(factory + 0x24, 0x0e70_0000);
        assert_eq!(
            e.call(0x00e9_20a0, &args![factory, 0u32, 2u32, 1u32]).u32(),
            0
        );
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, NO_NAME_010F0B20]]
        );
        // Found at once: no lock.
        e.register(FACTORY_SLOTS + 0xc, |_, _| 0x0e90_0000u32.into_ret());
        e.call_log = Some(vec![]);
        let call = args![factory, name, 2u32, 1u32];
        assert_eq!(e.call(0x00e9_20a0, &call).u32(), 0x0e90_0000);
        assert!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).is_empty());
        // Not found: the second library has it.
        let first = object(&mut e, 1);
        let second = object(&mut e, 1);
        e.register(FACTORY_SLOTS + 0xc, |_, _| 0u32.into_ret());
        e.register_double(FACTORY_SLOTS + 0x38, move |_, _| first.into_ret());
        e.register_double(FACTORY_SLOTS + 0x3c, move |_, _| second.into_ret());
        e.register_double(OBJECT_SLOTS + 8, move |_, a| {
            if a[0] == second {
                0x0e91_0000u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_20a0, &call).u32(), 0x0e91_0000);
        assert_eq!(
            calls(&e, OBJECT_SLOTS + 8),
            vec![
                vec![first, 0x0e70_0000, name, 2],
                vec![second, 0x0e70_0000, name, 2]
            ]
        );
        assert_eq!(
            calls(&e, FACTORY_SLOTS + 0x18),
            vec![vec![factory, 0x0e91_0000, 2]]
        );
        assert_eq!(
            calls(&e, REGISTER_D3D_SHADER_00E752B0),
            vec![vec![0x0e70_0000, 0x0e91_0000]]
        );
        assert_eq!(
            calls(&e, SET_D3D_RENDERER_00E818D0),
            vec![vec![0x0e91_0000, 0x0e70_0000]]
        );
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![factory + 0x80]]
        );
        assert!(calls(&e, REPORT_ERROR_00AA0020).is_empty());
        // Nobody has it: reported with the byte set.
        e.register(OBJECT_SLOTS + 8, |_, _| 0u32.into_ret());
        e.register(FACTORY_SLOTS + 0x3c, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_20a0, &call).u32(), 0);
        assert_eq!(
            calls(&e, REPORT_ERROR_00AA0020),
            vec![vec![1, 0, NOT_FOUND_010F0B54, name, 2]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00e9_20a0, &args![factory, name, 2u32, 0u32]).u32(),
            0
        );
        assert!(calls(&e, REPORT_ERROR_00AA0020).is_empty());
    }

    #[test]
    fn release_shader_from_library_asks_until_one_releases() {
        let mut e = engine();
        let factory = factory(&mut e);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, b'a');
        let library = object(&mut e, 1);
        e.register_double(FACTORY_SLOTS + 0x38, move |_, _| library.into_ret());
        // Not found, and the -1 one has another implementation: false.
        let any = object(&mut e, 1);
        e.register_double(FACTORY_SLOTS + 0xc, move |_, a| {
            if a[2] == 0xffff_ffff {
                any.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(IMPLEMENTATION_00CC0760, |_, _| 7u32.into_ret());
        assert!(!e.call(0x00e9_2270, &args![factory, name, 2u32]).bool());
        assert!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).is_empty());
        // The -1 one has this implementation: the library releases it.
        e.register(OBJECT_SLOTS + 0x10, |_, _| 1u32.into_ret());
        assert!(e.call(0x00e9_2270, &args![factory, name, 7u32]).bool());
        assert_eq!(calls(&e, OBJECT_SLOTS + 0x10), vec![vec![library, name, 7]]);
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 1);
        // No library releases it: false after the walk.
        e.register(OBJECT_SLOTS + 0x10, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00e9_2270, &args![factory, name, 7u32]).bool());
        assert_eq!(calls(&e, FACTORY_SLOTS + 0x3c).len(), 1);
        // No name: false.
        assert!(!e.call(0x00e9_2270, &args![factory, 0u32, 7u32]).bool());
    }

    #[test]
    fn release_shader_from_library_ov2_needs_the_same_shader() {
        let mut e = engine();
        let factory = factory(&mut e);
        let shader = object(&mut e, 1);
        e.mem.set_u32(shader + 8, 0x0e60_0000);
        e.register(NAME_ADDRESS_00A62660, |_, a| (a[0] + 8).into_ret());
        e.register(IMPLEMENTATION_00CC0760, |_, _| 3u32.into_ret());
        let library = object(&mut e, 1);
        e.register_double(FACTORY_SLOTS + 0x38, move |_, _| library.into_ret());
        e.register(OBJECT_SLOTS + 0xc, |_, _| 1u32.into_ret());
        e.register_double(FACTORY_SLOTS + 0xc, move |_, a| {
            if a[2] == 3 {
                shader.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        assert!(e.call(0x00e9_2330, &args![factory, shader]).bool());
        assert_eq!(
            calls(&e, FACTORY_SLOTS + 0xc),
            vec![vec![factory, 0x0e60_0000, 3]]
        );
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xc), vec![vec![library, shader]]);
        // Another shader under the name: the -1 lookup decides.
        e.register(FACTORY_SLOTS + 0xc, |_, _| 0x0e61_0000u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e9_2330, &args![factory, shader]).bool());
        assert_eq!(calls(&e, FACTORY_SLOTS + 0xc)[1][2], 0xffff_ffff);
        assert!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).is_empty());
        assert!(!e.call(0x00e9_2330, &args![factory, 0u32]).bool());
    }

    #[test]
    fn remove_shader_removes_the_key() {
        let mut e = engine();
        let factory = factory(&mut e);
        e.call(0x00e9_23f0, &args![factory, 0x0e60_0000u32, 4u32]);
        assert!(calls(&e, REMOVE_LIBRARY_00E91FD0).is_empty());
        e.mem.set_u32(factory + 0x28, 0x0e62_0000);
        e.register(REMOVE_LIBRARY_00E91FD0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x0e60_0000);
            assert_eq!(e.mem.u32(a[1] + 4), 4);
            Ret::default()
        });
        e.call(0x00e9_23f0, &args![factory, 0x0e60_0000u32, 4u32]);
        assert_eq!(calls(&e, REMOVE_LIBRARY_00E91FD0)[0][0], 0x0e62_0000);
    }

    #[test]
    fn restore_all_shader_renderer_data_walks_the_shader_map() {
        let mut e = engine();
        let factory = factory(&mut e);
        e.mem.set_u32(factory + 0x24, 0x0e70_0000);
        let map = map(&mut e, 0);
        e.mem.set_u32(factory + 0x28, map);
        let buckets = e.mem.u32(map + 8);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(buckets + 4, item);
        let ready = object(&mut e, 1);
        let lost = object(&mut e, 1);
        // Two steps: the first answers a shader whose slot 0x48 says true, the second one that
        // needs restoring, and ends the walk.
        e.register_double(SHADER_MAP_STEP_00B57F20, move |e, a| {
            let step = e.mem.u32(a[1]);
            if step == item {
                e.mem.set_u32(a[3], ready);
                e.mem.set_u32(a[1], item + 1);
            } else {
                e.mem.set_u32(a[3], lost);
                e.mem.set_u32(a[1], 0);
            }
            Ret::default()
        });
        e.register_double(OBJECT_SLOTS + 0x48, move |_, a| {
            u32::from(a[0] == ready).into_ret()
        });
        e.call(0x00e9_2470, &args![factory]);
        assert_eq!(calls(&e, SHADER_MAP_STEP_00B57F20).len(), 2);
        assert_eq!(
            calls(&e, SET_D3D_RENDERER_00E818D0),
            vec![vec![lost, 0x0e70_0000]]
        );
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xd0), vec![vec![lost]]);
        assert_eq!(
            calls(&e, REGISTER_D3D_SHADER_00E752B0),
            vec![vec![0x0e70_0000, lost]]
        );
        // An empty map: no step.
        e.mem.set_u32(buckets + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2470, &args![factory]);
        assert!(calls(&e, SHADER_MAP_STEP_00B57F20).is_empty());
    }

    #[test]
    fn map_scalar_deleting_destructors_empty_each_level() {
        let shader = SHADER_STRING_MAP_VTABLES;
        let library = LIBRARY_STRING_MAP_VTABLES;
        for (addr, levels) in [
            (0x00e9_2610u32, vec![shader[2]]),
            (0x00e9_2650, vec![shader[1], shader[2]]),
            (0x00e9_26b0, vec![library[2]]),
            (0x00e9_26f0, vec![library[1], library[2]]),
        ] {
            for flags in [0u32, 1] {
                let mut e = engine();
                let map = map(&mut e, 0);
                let buckets = e.mem.u32(map + 8);
                let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
                let tables = seen.clone();
                e.register_double(MAP_REMOVE_ALL_00B601A0, move |e, a| {
                    tables.borrow_mut().push(e.mem.u32(a[0]));
                    Ret::default()
                });
                assert_eq!(e.call(addr, &args![map, flags]).u32(), map);
                assert_eq!(*seen.borrow(), levels);
                assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![buckets]]);
                let freed = if flags == 1 {
                    vec![vec![MEMORY_MANAGER_OBJECT, map]]
                } else {
                    vec![]
                };
                assert_eq!(calls(&e, MEMORY_MANAGER_FREE), freed);
            }
        }
    }

    #[test]
    fn is_default_implementation_asks_an_interface() {
        let mut e = engine();
        let shader = object(&mut e, 1);
        assert!(!e.call(0x00e9_2730, &args![0u32, 0u32]).bool());
        // Its RTTI is the interface's: slot 0xe4 decides.
        let rtti = e.mem.alloc(8);
        e.register_double(OBJECT_SLOTS + 8, move |_, _| rtti.into_ret());
        e.register(OBJECT_SLOTS + 0xe4, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00e9_2730, &args![0u32, shader]).bool());
        e.mem.set_u32(rtti + 4, SHADER_INTERFACE_RTTI_0126F75C);
        assert!(e.call(0x00e9_2730, &args![0u32, shader]).bool());
        assert_eq!(calls(&e, OBJECT_SLOTS + 0xe4), vec![vec![shader]]);
    }

    #[test]
    fn set_at_replaces_a_value_or_adds_an_item() {
        let mut e = engine();
        let map = map(&mut e, 0);
        let buckets = e.mem.u32(map + 8);
        let item = e.mem.alloc(0xc);
        let old = object(&mut e, 1);
        e.mem.set_u32(item + 4, 0x11);
        e.mem.set_u32(item + 8, old);
        e.mem.set_u32(buckets + 4, item);
        let value = object(&mut e, 1);
        // The key 0x11 hashes to bucket 1 and is found: the value replaced.
        e.call(0x00e9_27f0, &args![map, 0x11u32, value]);
        assert_eq!(e.mem.u32(item + 8), value);
        assert_eq!(e.mem.u32(value + 4), 1);
        assert_eq!(calls(&e, OBJECT_SLOTS + 4), vec![vec![old]]);
        // A new key (bucket 0): a new item at the bucket's head, the count up.
        let fresh = e.mem.alloc(0xc);
        e.register_double(NEW_ITEM_0043A010, move |_, _| fresh.into_ret());
        e.call(0x00e9_27f0, &args![map, 0x22u32, value]);
        assert_eq!(e.mem.u32(buckets), fresh);
        assert_eq!(e.mem.u32(fresh), 0);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(calls(&e, NEW_ITEM_0043A010), vec![vec![map + 0xc]]);
        assert_eq!(
            calls(&e, MAP_SET_VALUE),
            vec![vec![map, fresh, 0x22, value]]
        );
        // Counted up for the copy, down for the argument.
        assert_eq!(e.mem.u32(value + 4), 1);
    }

    #[test]
    fn set_value_copies_keys_for_a_copying_map() {
        let mut e = engine();
        let item = e.mem.alloc(0xc);
        let key = e.mem.alloc(4);
        e.mem.write(key, b"abc\0");
        let value = object(&mut e, 0);
        let plain = map(&mut e, 0);
        e.call(0x00e9_2910, &args![plain, item, key, value]);
        assert_eq!(e.mem.u32(item + 4), key);
        assert_eq!(e.mem.u32(item + 8), value);
        // Counted up for the item and down for the argument.
        assert_eq!(e.mem.u32(value + 4), 0);
        let copying = map(&mut e, 1);
        e.register(ALLOCATE_00AA1070, |_, _| 0x0e66_0000u32.into_ret());
        e.call(0x00e9_2910, &args![copying, item, key, value]);
        assert_eq!(calls(&e, ALLOCATE_00AA1070), vec![vec![4]]);
        assert_eq!(e.mem.u32(item + 4), 0x0e66_0000);
        assert_eq!(
            calls(&e, STRCPY_S_00EC65A6),
            vec![vec![0x0e66_0000, 4, key]]
        );
    }

    #[test]
    fn insert_shader_sets_the_key_in_the_shader_map() {
        let mut e = engine();
        let factory = factory(&mut e);
        let shader = object(&mut e, 1);
        e.mem.set_u32(shader + 8, 0x0e60_0000);
        e.register(NAME_ADDRESS_00A62660, |_, a| (a[0] + 8).into_ret());
        // Without a map the key is still written; nothing is set.
        e.call(0x00e9_29c0, &args![factory, shader, 5u32]);
        assert_eq!(calls(&e, VSPRINTF_S).len(), 1);
        assert!(calls(&e, SHADER_MAP_SET_AT_00A642A0).is_empty());
        e.mem.set_u32(factory + 0x28, 0x0e62_0000);
        e.register(SHADER_MAP_SET_AT_00A642A0, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x0e60_0000);
            assert_eq!(e.mem.u32(a[1] + 4), 5);
            Ret::default()
        });
        e.call(0x00e9_29c0, &args![factory, shader, 5u32]);
        let set = calls(&e, SHADER_MAP_SET_AT_00A642A0);
        assert_eq!((set[0][0], set[0][2]), (0x0e62_0000, shader));
        e.call_log = Some(vec![]);
        e.call(0x00e9_29c0, &args![factory, 0u32, 5u32]);
        assert!(calls(&e, VSPRINTF_S).is_empty());
    }

    #[test]
    fn insert_library_sets_it_by_name() {
        let mut e = engine();
        let factory = factory(&mut e);
        let map = map(&mut e, 0);
        e.mem.set_u32(factory + 0x30, map);
        let library = object(&mut e, 1);
        let fresh = e.mem.alloc(0xc);
        e.register_double(NEW_ITEM_0043A010, move |_, _| fresh.into_ret());
        e.register(LIBRARY_GET_NAME_00D47A30, |_, _| 0x44u32.into_ret());
        e.call(0x00e9_2a40, &args![factory, library]);
        assert_eq!(
            calls(&e, MAP_SET_VALUE),
            vec![vec![map, fresh, 0x44, library]]
        );
        // The argument copy, the item's copy, then the argument let go: one more count.
        assert_eq!(e.mem.u32(library + 4), 2);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2a40, &args![factory, 0u32]);
        assert!(calls(&e, LIBRARY_GET_NAME_00D47A30).is_empty());
    }

    /// A library map holding `library` under key 0x33 (bucket 1).
    fn library_map(e: &mut Engine, factory: u32, library: u32) {
        let map = map(e, 0);
        e.mem.set_u32(factory + 0x30, map);
        let buckets = e.mem.u32(map + 8);
        let item = e.mem.alloc(0xc);
        e.mem.set_u32(item + 4, 0x33);
        e.mem.set_u32(item + 8, library);
        e.mem.set_u32(buckets + 4, item);
    }

    #[test]
    fn find_library_answers_and_lets_go() {
        let mut e = engine();
        let factory = factory(&mut e);
        let library = object(&mut e, 1);
        library_map(&mut e, factory, library);
        assert_eq!(e.call(0x00e9_2a70, &args![factory, 0x33u32]).u32(), library);
        assert_eq!(e.mem.u32(library + 4), 1);
        assert_eq!(e.call(0x00e9_2a70, &args![factory, 0x35u32]).u32(), 0);
    }

    #[test]
    fn remove_library_removes_one_held_only_by_the_map() {
        let mut e = engine();
        let factory = factory(&mut e);
        let library = object(&mut e, 1);
        library_map(&mut e, factory, library);
        e.call(0x00e9_2ae0, &args![factory, 0x33u32]);
        let map = e.mem.u32(factory + 0x30);
        assert_eq!(calls(&e, REMOVE_LIBRARY_00E91FD0), vec![vec![map, 0x33]]);
        assert_eq!(e.mem.u32(library + 4), 1);
        // Held elsewhere too (count 3 with the stack's): only let go.
        e.mem.set_u32(library + 4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00e9_2ae0, &args![factory, 0x33u32]);
        assert!(calls(&e, REMOVE_LIBRARY_00E91FD0).is_empty());
        assert_eq!(e.mem.u32(library + 4), 2);
    }

    #[test]
    fn get_next_library_steps_the_iterator() {
        let mut e = engine();
        let factory = factory(&mut e);
        assert_eq!(e.call(0x00e9_2be0, &args![factory]).u32(), 0);
        let library = object(&mut e, 1);
        e.mem.set_u32(factory + 0x30, 0x0e63_0000);
        assert_eq!(e.call(0x00e9_2be0, &args![factory]).u32(), 0);
        e.mem.set_u32(factory + 0x2c, 0x0e64_0000);
        e.register_double(MAP_STEP_00E6E630, move |e, a| {
            e.mem.set_u32(a[3], library);
            e.mem.set_u32(library + 4, 2);
            Ret::default()
        });
        assert_eq!(e.call(0x00e9_2be0, &args![factory]).u32(), library);
        assert_eq!(e.mem.u32(library + 4), 1);
        let step = &calls(&e, MAP_STEP_00E6E630)[0];
        assert_eq!(step[..2], [0x0e63_0000, factory + 0x2c]);
    }
}
