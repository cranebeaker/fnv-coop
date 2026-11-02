//! `Havok SDK/hkversionpatchmanager.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dba580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00db_85e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dba580(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00db_85e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkDataClass` (`00dba650` constructor taking the implementation pointer, `00dba660` empty
/// destructor, `00dba670` `isNull`) and `hkDataObject` (`00dba760` constructor taking the
/// implementation pointer, `00dba7f0` destructor), `thiscall` on a one-word local.
const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
const DATA_CLASS_IS_NULL: u32 = 0x00db_a670;
const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
const DATA_OBJECT_DESTRUCT: u32 = 0x00db_a7f0;
/// `hkDataWorldDict` (`thiscall` on the world; the classes and objects are passed as the
/// addresses of one-word `hkDataClass` and `hkDataObject` locals): constructor (`00dc8030`) and
/// destructor (`00dc8080`); `findClass(name)` (`00dc7300`), `wrapClass(class)` (`00dc8270`),
/// `createClass(record)` (`00dc80c0`), `renameClass(class, name)` (`00dc7320`),
/// `setClassVersion(class, version)` (`00dc72e0`), `removeClass(class)` (`00dc8870`),
/// `setClassParent(class, parent)` (`00dc7cc0`), `addClassMember(class, four words)`
/// (`00dc7d20`), `removeClassMember(class, name)` (`00dc7600`), `renameClassMember(class, from,
/// to)` (`00dc73f0`), `findAllClasses(&array)` (`00dc7390`), `findObjectsByBaseClass(name,
/// &array)` (`00dc73c0`), `findObjectsByExactClass(name, &array)` (`00dc73a0`),
/// `castObject(object, class)` (`00dc7dc0`) and `copyClassFromWorld(name, world)` (`00dc85a0`).
const WORLD_CONSTRUCT: u32 = 0x00dc_8030;
const WORLD_DESTRUCT: u32 = 0x00dc_8080;
const WORLD_FIND_CLASS: u32 = 0x00dc_7300;
const WORLD_WRAP_CLASS: u32 = 0x00dc_8270;
const WORLD_CREATE_CLASS: u32 = 0x00dc_80c0;
const WORLD_RENAME_CLASS: u32 = 0x00dc_7320;
const WORLD_SET_CLASS_VERSION: u32 = 0x00dc_72e0;
const WORLD_REMOVE_CLASS: u32 = 0x00dc_8870;
const WORLD_SET_CLASS_PARENT: u32 = 0x00dc_7cc0;
const WORLD_ADD_MEMBER: u32 = 0x00dc_7d20;
const WORLD_REMOVE_MEMBER: u32 = 0x00dc_7600;
const WORLD_RENAME_MEMBER: u32 = 0x00dc_73f0;
const WORLD_FIND_ALL_CLASSES: u32 = 0x00dc_7390;
const WORLD_FIND_OBJECTS_BY_BASE_CLASS: u32 = 0x00dc_73c0;
const WORLD_FIND_OBJECTS_BY_EXACT_CLASS: u32 = 0x00dc_73a0;
const WORLD_CAST_OBJECT: u32 = 0x00dc_7dc0;
const WORLD_COPY_CLASS_FROM_WORLD: u32 = 0x00dc_85a0;
/// `hkClass::getDescribedVersion` (`00d47120`) and `hkClass::getName` (`00d46d40`), `thiscall`.
const CLASS_GET_DESCRIBED_VERSION: u32 = 0x00d4_7120;
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
/// The text builder of the error messages (`hkOstream`, identity of the class unconfirmed):
/// constructor (`00cb1600`, `thiscall`: buffer, size), append string (`00cc01c0`), append integer
/// (`00cc0280`) (both answer the stream) and destructor (`00cc00e0`).
const OSTREAM_CONSTRUCT: u32 = 0x00cb_1600;
const OSTREAM_APPEND_TEXT: u32 = 0x00cc_01c0;
const OSTREAM_APPEND_INT: u32 = 0x00cc_0280;
const OSTREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The constructor of a string buffer local (`00cbbd80`, `fastcall`, unnamed in the map): a
/// 128-byte inline buffer holding one NUL character, capacity word `0x80000080`.
const STRING_BUFFER_CONSTRUCT: u32 = 0x00cb_bd80;
/// `hkArrayUtil`: `_reserve(allocator, array, capacity, element size)` (`00c908c0`) and
/// `_reserveMore(allocator, array, element size)` (`00c90940`), both cdecl; `hkMemUtil::memMove(
/// destination, source, count)` (`00d2cc70`, cdecl).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `hkArray::swap(other)` (`00d9ec10`, `thiscall`, `RET 4`): exchanges the three words.
const ARRAY_SWAP: u32 = 0x00d9_ec10;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), `thiscall`
/// on the object, and `hkMemoryRouter::easyFree(allocator, block)` (`00c856c0`, cdecl).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkString::strDup(text)` (`00cb1410`) and `hkString::strCmp(a, b)` (`00cb0fa0`), cdecl.
const STRING_DUPLICATE: u32 = 0x00cb_1410;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
/// `hkMapBase<unsigned __int64, unsigned __int64>` (`thiscall` on the map): `findKey(key)`
/// (`00cb1c60`, index, `RET 8`), `insert(allocator, key, value)` (`00cb1bc0`, `RET 0x14`),
/// `getWithDefault(key, default)` (`00cb1cb0`, `RET 0x10`), `clearAndDeallocate(allocator)`
/// (`00cb1e40`), `reserve(allocator, capacity)` (`00cb2030`) and `clear()` (`00cb1e00`).
const U64_MAP_FIND_KEY: u32 = 0x00cb_1c60;
const U64_MAP_INSERT: u32 = 0x00cb_1bc0;
const U64_MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1cb0;
const U64_MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1e40;
const U64_MAP_RESERVE: u32 = 0x00cb_2030;
const U64_MAP_CLEAR: u32 = 0x00cb_1e00;
/// `hkMapBase<unsigned long, unsigned long>` (`thiscall` on the map): `findKey(key)` (`00cb18f0`),
/// `insert(allocator, key, value)` (`00cb1870`), `getWithDefault(key, default)` (`00cb1930`),
/// `clearAndDeallocate(allocator)` (`00cb1a90`) and `reserve(allocator, capacity)` (`00cb1fe0`);
/// the empty destructor both map kinds are followed by (`00a29680`).
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_DESTRUCT: u32 = 0x00a2_9680;
/// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>` (`thiscall` on the map):
/// constructor (`00d87790`, one dword it ignores), destructor (`00d877d0`), `getIterator`
/// (`00d87410`), `getValue(iterator)` (`00d87440`), `getNext(iterator)` (`00d87480`),
/// `isValid(&out byte, iterator)` (`00d874b0`), `insert(key, value)` (`00d874d0`), `remove(key)`
/// (`00d87820`), `getWithDefault(key, default)` (`00d87800`) and the lookup that inserts the
/// default when the key is absent (`00d87600`, unnamed in the map).
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
const STRING_MAP_GET_ITERATOR: u32 = 0x00d8_7410;
const STRING_MAP_GET_VALUE: u32 = 0x00d8_7440;
const STRING_MAP_GET_NEXT: u32 = 0x00d8_7480;
const STRING_MAP_IS_VALID: u32 = 0x00d8_74b0;
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
const STRING_MAP_REMOVE: u32 = 0x00d8_7820;
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
const STRING_MAP_GET_OR_INSERT: u32 = 0x00d8_7600;
/// The multimap's `getWithDefault` on its map at `+0xc` (`00dc8e30`, `thiscall`, `RET 4`: key):
/// the first entry of the key's chain, `-1` for none.
const MULTI_MAP_FIRST: u32 = 0x00dc_8e30;
/// The two small functions of the unit `hkVersionPatchManager::UidFromClassVersion` forwards to
/// (unnamed in the map, `thiscall`, `RET 8`): the string at an index of the array at `+0x10`
/// (`00db7cf0`) and the version word of a uid (`00db7c70`, answers its second argument).
const UID_CLASS_NAME: u32 = 0x00db_7cf0;
const UID_VERSION: u32 = 0x00db_7c70;

/// The word array the exe's inline code keeps as `(data, size, capacity word)`; a capacity word
/// with bit 31 set owns no block. Returns the allocator the memory router (`00c85750`) keeps at
/// `+0x10`.
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The inline `hkArray::clearAndDeallocate` the exe repeats: size zero, the block (capacity
/// without the flag bits, times `entry_size` bytes) freed through the allocator's slot `0x10`
/// unless the capacity word has bit 31 set, then data zero and capacity word `0x80000000`.
fn clear_array(e: &mut Engine, array: u32, entry_size: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(entry_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The inline `hkArray<int>::pushBack`: when the size equals the capacity (the flag bits
/// masked off) `_reserveMore(allocator, array, 4)` makes room, then the word is stored.
fn push_back_word(e: &mut Engine, array: u32, value: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Releases a map the way the exe does: `clearAndDeallocate(allocator)` of the 64-bit key kind,
/// then the empty destructor.
fn release_u64_map(e: &mut Engine, map: u32) {
    let allocator = router_allocator(e);
    e.call(U64_MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_DESTRUCT, &args![map]);
}

// Translated from 00db7c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): forwards `(first, second)` to the version
/// word function (`00db7c70`) with the uid cache object at `+8` as its `this`; answers its result.
pub fn fn_00db7c90(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let cache = e.mem.u32(this.addr() + 8);
    e.call(UID_VERSION, &args![cache, first, second]).u32()
}

// Translated from 00db7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one stack word): whether the patch record (`+0` old name,
/// `+4` new name, `+8` old version, `+0xc` new version, `-1` for none) names what its kind
/// needs. Without an old version (a new class) it needs no old name, a new version and a
/// new name; with an old version and no new version (a removed class) it needs the old name
/// and no new name; with both versions it needs the old name.
pub fn fn_00db7cb0(e: &mut Engine, patch: Ptr) -> u32 {
    let base = patch.addr();
    let old_name = e.mem.u32(base);
    let new_name = e.mem.u32(base + 4);
    let old_version = e.mem.u32(base + 8);
    let new_version = e.mem.u32(base + 0xc);
    if old_version == 0xffff_ffff {
        u32::from(old_name == 0 && new_version != 0xffff_ffff && new_name != 0)
    } else if new_version == 0xffff_ffff {
        u32::from(old_name != 0 && new_name == 0)
    } else {
        u32::from(old_name != 0)
    }
}

// Translated from 00db7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `text`): the interned copy of the string. A
/// null `text` answers null; otherwise the string map at `+0x1c` is asked (`getWithDefault`,
/// default null) and a miss duplicates the text (`hkString::strDup`) and inserts the copy as
/// both key and value.
pub fn fn_00db7d00(e: &mut Engine, this: Ptr, text: u32) -> u32 {
    if text == 0 {
        return 0;
    }
    let map = this.addr() + 0x1c;
    let mut interned = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, text, 0u32])
        .u32();
    if interned == 0 {
        interned = e.call(STRING_DUPLICATE, &args![text]).u32();
        e.call(STRING_MAP_INSERT, &args![map, interned, interned]);
    }
    interned
}

// Translated from 00db7d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): forwards `(first, second)` to the class name
/// function (`00db7cf0`) with the uid cache object at `+8` as its `this`; answers its result.
pub fn fn_00db7d40(e: &mut Engine, this: Ptr, first: u32, second: u32) -> u32 {
    let cache = e.mem.u32(this.addr() + 8);
    e.call(UID_CLASS_NAME, &args![cache, first, second]).u32()
}

// Translated from 00db7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::UidFromClassVersion::get` (Xbox PDB), `thiscall`, `RET 8`: the uid of
/// a class name and version, `(version << 32) | index`, where the index is that of the interned
/// name (`00db7d00`) in the array at `+0x10`: the string map is asked (`00d87600`, default the
/// array's size, which inserts it) and an answer equal to the size appends the name to the array.
pub fn hk_version_patch_manager_uid_from_class_version_get(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    version: u32,
) -> u64 {
    let base = this.addr();
    let interned = fn_00db7d00(e, this, name);
    let size = e.mem.u32(base + 0x14);
    let index = e
        .call(STRING_MAP_GET_OR_INSERT, &args![this, interned, size])
        .u32();
    if index == e.mem.u32(base + 0x14) {
        push_back_word(e, base + 0x10, interned);
    }
    (u64::from(version) << 32) | u64::from(index)
}

// Translated from 00db7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::getUid` (Xbox PDB), `thiscall`, `RET 8`: the uid cache object at `+8`
/// answers the uid of `(name, version)` (`00db7d60`, a tail jump).
pub fn hk_version_patch_manager_get_uid(e: &mut Engine, this: Ptr, name: u32, version: u32) -> u64 {
    let cache = e.mem.u32(this.addr() + 8);
    hk_version_patch_manager_uid_from_class_version_get(e, Ptr::new(cache), name, version)
}

// Translated from 00db7de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::findLastPatchIndexForUid` (Xbox PDB), `thiscall`, `RET 0xc`: the index
/// of the patch (the array at `+0xc`) the map at `+0x18` has for `uid`, following the chain of
/// patches that continue from one another: it stops at a patch without a new version, and, when
/// `flag` is zero, at one whose old and new names differ; otherwise the uid of
/// `(new name, new version)` is looked up (default `-1`) and the walk goes on. `-1` when the map
/// has no entry.
pub fn hk_version_patch_manager_find_last_patch_index_for_uid(
    e: &mut Engine,
    this: Ptr,
    uid: u64,
    flag: u32,
) -> i32 {
    let base = this.addr();
    let mut index = e
        .call(U64_MAP_GET_WITH_DEFAULT, &args![base + 0x18, uid, u64::MAX])
        .i32();
    while index != -1 {
        let patches = e.mem.u32(base + 0xc);
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        let new_version = e.mem.u32(patch + 0xc);
        if new_version == 0xffff_ffff {
            return index;
        }
        if flag == 0 {
            let new_name = e.mem.u32(patch + 4);
            if new_name != 0 {
                let old_name = e.mem.u32(patch);
                if e.call(STRING_COMPARE, &args![old_name, new_name]).i32() != 0 {
                    return index;
                }
            }
        }
        let new_name = e.mem.u32(patch + 4);
        let cache = e.mem.u32(base + 8);
        let next = hk_version_patch_manager_uid_from_class_version_get(
            e,
            Ptr::new(cache),
            new_name,
            new_version,
        );
        index = e
            .call(
                U64_MAP_GET_WITH_DEFAULT,
                &args![base + 0x18, next, u64::MAX],
            )
            .i32();
    }
    -1
}

// Translated from 00db7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four stack words `(target, node, multimap, visited)`):
/// whether `target` can be reached from `node` along the multimap's chains. A `node` already in
/// the pointer map `visited` (`findKey` not past the map's mask word at `+8`) answers 0; else it
/// is inserted (value 0) and each entry of its chain (the multimap's `getWithDefault`, entries of
/// `(value, next)` pairs at the multimap's first word) is tried: the target itself, or a
/// recursive reach, answers 1. 0 when none does.
pub fn fn_00db7e70(e: &mut Engine, target: u32, node: u32, multimap: Ptr, visited: Ptr) -> u32 {
    let map = visited.addr();
    let key_index = e.call(MAP_FIND_KEY, &args![map, node]).i32();
    if key_index <= e.mem.i32(map + 8) {
        return 0;
    }
    let allocator = router_allocator(e);
    e.call(MAP_INSERT, &args![map, allocator, node, 0u32]);
    let mut link = e.call(MULTI_MAP_FIRST, &args![multimap, node]).i32();
    while link != -1 {
        let entries = e.mem.u32(multimap.addr());
        let entry = entries.wrapping_add((link as u32).wrapping_mul(8));
        let child = e.mem.u32(entry);
        if target == child {
            return 1;
        }
        if fn_00db7e70(e, target, child, multimap, visited) != 0 {
            return 1;
        }
        let entries = e.mem.u32(multimap.addr());
        link = e
            .mem
            .i32(entries.wrapping_add((link as u32).wrapping_mul(8)) + 4);
    }
    0
}

// Translated from 00db7f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::UidFromClassVersion::~UidFromClassVersion` (Xbox PDB), `fastcall`: the
/// interned strings (the values of the string map at `+0x1c`) are freed through the memory
/// router's allocator (`easyFree`), that map and the array at `+0x10` (words) are destroyed, and
/// the string map at `+0` too.
pub fn hk_version_patch_manager_uid_from_class_version_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let map = base + 0x1c;
    e.with_stack(4, |e, valid| {
        let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![map]).u32();
        e.call(STRING_MAP_IS_VALID, &args![map, valid, iterator]);
        while e.mem.u8(valid.addr()) != 0 {
            let value = e.call(STRING_MAP_GET_VALUE, &args![map, iterator]).u32();
            let allocator = router_allocator(e);
            e.call(EASY_FREE, &args![allocator, value]);
            iterator = e.call(STRING_MAP_GET_NEXT, &args![map, iterator]).u32();
            e.call(STRING_MAP_IS_VALID, &args![map, valid, iterator]);
        }
    });
    e.call(STRING_MAP_DESTRUCT, &args![map]);
    clear_array(e, base + 0x10, 4);
    e.call(STRING_MAP_DESTRUCT, &args![this]);
}

// Translated from 00db85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::~hkVersionPatchManager` (Xbox PDB), `fastcall`: stores the vtable
/// `010d6138`; the uid cache object at `+8`, when there is one, is destroyed
/// (`UidFromClassVersion::~UidFromClassVersion`) and returned to the router's allocator (slot
/// `8`, `0x2c` bytes); the map at `+0x18` is cleared and deallocated; the patch array at `+0xc` is
/// released; the base class vtable `0102e388` is stored.
pub fn hk_version_patch_manager_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let cache = e.mem.u32(base + 8);
    e.mem.set_u32(base, 0x010d_6138);
    if cache != 0 {
        hk_version_patch_manager_uid_from_class_version_destructor(e, Ptr::new(cache));
        let allocator = router_allocator(e);
        e.vcall(allocator, 8, &args![cache, 0x2cu32]);
    }
    release_u64_map(e, base + 0x18);
    clear_array(e, base + 0xc, 4);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00dba210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a `hkSerializeMultiMap<unsigned __int64,int>`, `RET
/// 8`): the first entry of the chain for `uid`, the `getWithDefault` of the map at `+0xc` with
/// default `-1`.
pub fn fn_00dba210(e: &mut Engine, this: Ptr, uid: u64) -> i32 {
    e.call(
        U64_MAP_GET_WITH_DEFAULT,
        &args![this.addr() + 0xc, uid, u64::MAX],
    )
    .i32()
}

// Translated from 00dba230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the array of 16-byte entries (data
/// word, size at `+4`) holds one whose first two words equal those of the record at `key`.
pub fn fn_00dba230(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
    let base = this.addr();
    let size = e.mem.i32(base + 4);
    if size > 0 {
        let low = e.mem.u32(key.addr());
        let high = e.mem.u32(key.addr() + 4);
        let mut entry = e.mem.u32(base);
        let mut index = 0;
        while index < size {
            if low == e.mem.u32(entry) && high == e.mem.u32(entry + 4) {
                return true;
            }
            index += 1;
            entry = entry.wrapping_add(16);
        }
    }
    false
}

// Translated from 00dba270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x14`: `allocator`, `index`, `delete_count`,
/// `source`, `insert_count`): `hkArray::spliceInto` for 16-byte entries (data word, size at `+4`,
/// capacity word at `+8`). The new size is the size less `delete_count` plus `insert_count`; when
/// the capacity (flag bits masked off) is below it, `_reserve(allocator, array, max(2 *
/// capacity, new size), 16)` grows the block; the tail after the removed range is moved
/// (`memMove`) to follow the inserted range, and the `insert_count` records at `source` are
/// copied to the gap, four words each (an entry address of zero is skipped).
pub fn fn_00dba270(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    delete_count: i32,
    source: u32,
    insert_count: i32,
) {
    let base = this.addr();
    let size = e.mem.i32(base + 4);
    let tail = size.wrapping_sub(index).wrapping_sub(delete_count);
    let new_size = size.wrapping_sub(delete_count).wrapping_add(insert_count);
    let capacity = (e.mem.u32(base + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, this, grown, 16u32]);
    }
    let data = e.mem.u32(base);
    let destination = data.wrapping_add(index.wrapping_add(insert_count).wrapping_mul(16) as u32);
    let moved_from = data.wrapping_add(index.wrapping_add(delete_count).wrapping_mul(16) as u32);
    e.call(
        MEM_MOVE,
        &args![destination, moved_from, tail.wrapping_mul(16)],
    );
    let mut entry = e.mem.u32(base).wrapping_add(index.wrapping_mul(16) as u32);
    let mut from = source;
    let mut remaining = insert_count;
    while remaining > 0 {
        if entry != 0 {
            for word in 0..4 {
                let value = e.mem.u32(from.wrapping_add(word * 4));
                e.mem.set_u32(entry.wrapping_add(word * 4), value);
            }
        }
        entry = entry.wrapping_add(16);
        from = from.wrapping_add(16);
        remaining -= 1;
    }
    e.mem.set_i32(base + 4, new_size);
}

// Translated from 00dba320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`: `allocator`, `index`, `source`, `count`):
/// the splice (`00dba270`) with nothing deleted: `hkArray::insertAt` of `count` records.
pub fn fn_00dba320(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32, count: i32) {
    fn_00dba270(e, this, allocator, index, 0, source, count);
}

// Translated from 00dba340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `allocator`, `index`, `source`): inserts one
/// record at `index` (`00dba320` with a count of 1).
pub fn fn_00dba340(e: &mut Engine, this: Ptr, allocator: u32, index: i32, source: u32) {
    fn_00dba320(e, this, allocator, index, source, 1);
}

// Translated from 00dbe710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<unsigned __int64,int>::getFreeIndex` (Xbox PDB), `fastcall`: the entry
/// array is `(data, size, capacity word)` with the chain head of free entries at `+0x18`. A free
/// entry (head not `-1`) is taken: the head becomes its `next` word (at `+4` of the 8-byte
/// entry) and its index is answered. Otherwise the array grows by one: a full array
/// (size equal to the capacity, flag bits masked off) first calls `_reserveMore(allocator, array,
/// 8)`, and the old size is the new index.
pub fn hk_serialize_multi_map_u64_get_free_index(e: &mut Engine, this: Ptr) -> i32 {
    let base = this.addr();
    let head = e.mem.i32(base + 0x18);
    if head != -1 {
        let data = e.mem.u32(base);
        let next = e
            .mem
            .u32(data.wrapping_add((head as u32).wrapping_mul(8)) + 4);
        e.mem.set_u32(base + 0x18, next);
        return head;
    }
    let index = e.mem.i32(base + 4);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(base + 8) & 0x3fff_ffff;
    if e.mem.u32(base + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, this, 8u32]);
    }
    let size = e.mem.u32(base + 4);
    e.mem.set_u32(base + 4, size.wrapping_add(1));
    index
}

// Translated from 00dba360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<unsigned __int64,int>::insert` (Xbox PDB), `thiscall`, `RET 0xc` (`key`
/// as two words, `value`: the address of the word to store): the head of the key's chain
/// (`getWithDefault` of the map at `+0xc`, default `-1`) is read, a free entry is taken
/// (`getFreeIndex`), and it is made `(value, old head)`; then the key's chain head becomes the
/// new index (`insert(allocator, key, index)`).
pub fn hk_serialize_multi_map_u64_insert(e: &mut Engine, this: Ptr, key: u64, value: Ptr) {
    let base = this.addr();
    let head = e
        .call(U64_MAP_GET_WITH_DEFAULT, &args![base + 0xc, key, u64::MAX])
        .u32();
    let index = hk_serialize_multi_map_u64_get_free_index(e, this);
    let entry = e.mem.u32(base).wrapping_add((index as u32).wrapping_mul(8));
    let item = e.mem.u32(value.addr());
    e.mem.set_u32(entry, item);
    e.mem.set_u32(entry + 4, head);
    let allocator = router_allocator(e);
    e.call(
        U64_MAP_INSERT,
        &args![base + 0xc, allocator, key, i64::from(index) as u64],
    );
}

// Translated from 00dc9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<char const *,int>::insert` (Xbox PDB), `thiscall`, `RET 8` (`key`,
/// `value`: the address of the word to store): as the 64-bit key kind (`00dba360`) with a
/// one-word key and the pointer map (`getWithDefault` `00cb1930`, `insert` `00cb1870`).
pub fn hk_serialize_multi_map_char_ptr_insert(e: &mut Engine, this: Ptr, key: u32, value: Ptr) {
    let base = this.addr();
    let head = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0xc, key, 0xffff_ffffu32],
        )
        .u32();
    let index = hk_serialize_multi_map_u64_get_free_index(e, this);
    let entry = e.mem.u32(base).wrapping_add((index as u32).wrapping_mul(8));
    let item = e.mem.u32(value.addr());
    e.mem.set_u32(entry, item);
    e.mem.set_u32(entry + 4, head);
    let allocator = router_allocator(e);
    e.call(MAP_INSERT, &args![base + 0xc, allocator, key, index]);
}

// Translated from 00dba3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<unsigned __int64,int>::~hkSerializeMultiMap` (Xbox PDB), `fastcall`: the
/// map at `+0xc` is cleared and deallocated and the entry array (8-byte entries) released.
pub fn hk_serialize_multi_map_u64_destructor(e: &mut Engine, this: Ptr) {
    release_u64_map(e, this.addr() + 0xc);
    clear_array(e, this.addr(), 8);
}

// Translated from 00dba430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeMultiMap<char const *,int>::~hkSerializeMultiMap` (Xbox PDB), `fastcall`: as the
/// 64-bit key kind (`00dba3d0`) with the pointer map's `clearAndDeallocate` (`00cb1a90`).
pub fn hk_serialize_multi_map_char_ptr_destructor(e: &mut Engine, this: Ptr) {
    let map = this.addr() + 0xc;
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_DESTRUCT, &args![map]);
    clear_array(e, this.addr(), 8);
}

// Translated from 00dba490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`): releases the array of 16-byte entries at `+0xc` (data
/// `+0xc`, size `+0x10`, capacity word `+0x14`).
pub fn fn_00dba490(e: &mut Engine, this: Ptr) {
    clear_array(e, this.addr() + 0xc, 16);
}

// Translated from 00dba4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `item`, a 16-byte record whose word at `+8`
/// is its sort key): inserts the record into the array of 16-byte entries sorted by that word
/// (signed): before the first entry whose key is greater (`00dba340` with the router's
/// allocator), else at the end (`_reserveMore(allocator, array, 16)` when full, then the four
/// words copied; an address of zero is skipped).
pub fn fn_00dba4d0(e: &mut Engine, this: Ptr, item: Ptr) {
    let base = this.addr();
    let key = e.mem.i32(item.addr() + 8);
    let mut index = 0;
    if e.mem.i32(base + 4) > 0 {
        let mut entry = e.mem.u32(base);
        loop {
            if key < e.mem.i32(entry + 8) {
                let allocator = router_allocator(e);
                fn_00dba340(e, this, allocator, index, item.addr());
                return;
            }
            index += 1;
            entry = entry.wrapping_add(16);
            if index >= e.mem.i32(base + 4) {
                break;
            }
        }
    }
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(base + 8) & 0x3fff_ffff;
    if e.mem.u32(base + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, this, 16u32]);
    }
    let size = e.mem.u32(base + 4);
    let slot = e.mem.u32(base).wrapping_add(size.wrapping_mul(16));
    if slot != 0 {
        for word in 0..4 {
            let value = e.mem.u32(item.addr() + word * 4);
            e.mem.set_u32(slot.wrapping_add(word * 4), value);
        }
    }
    e.mem.set_u32(base + 4, size.wrapping_add(1));
}

// Translated from 00dc8d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::UidFromClassVersion::cache` (Xbox PDB), `thiscall`, `RET 8` (`out`,
/// `text`): stores at `out` the interned copy of `text`, using the string map at `+0x4c` of
/// `this`; null `text` stores null; a miss duplicates the text (`hkString::strDup`) and
/// inserts the copy as key and value.
/// Answers `out` (`EAX` at both `RET`s; the caller reads the uid through it, 00dc8dee).
pub fn hk_version_patch_manager_uid_from_class_version_cache(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    text: u32,
) -> Ptr {
    if text == 0 {
        e.mem.set_u32(out.addr(), 0);
        return out;
    }
    let map = this.addr() + 0x4c;
    let mut interned = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![map, text, 0u32])
        .u32();
    if interned == 0 {
        interned = e.call(STRING_DUPLICATE, &args![text]).u32();
        e.call(STRING_MAP_INSERT, &args![map, interned, interned]);
    }
    e.mem.set_u32(out.addr(), interned);
    out
}

// Translated from 00db83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, six stack words `(node, state, multimap, count, stack,
/// carried)`): the depth-first step of the topological sort of the patches. `state` is the
/// array of words (`data` first) holding for each node `-1` (not seen), `-2` (on the walk) or its
/// place. A node already seen answers `count`. Otherwise it is marked `-2` and pushed on the word
/// array `stack`, and each node in its chain of the multimap is taken: one marked `-2` (a cycle)
/// is pushed, a string buffer is made (`00cbbd80`, on a 0x8c-byte local) and dropped, the
/// pushed word is popped, and the buffer's block, when it owns one, is freed through the memory
/// router's second allocator (slot `0x10`, size the capacity without flag bits); one marked `-1`
/// is walked recursively (`count` becomes the answer; `carried` is passed on unchanged). At the
/// end the node is popped, its state set to `count` and `count + 1` is answered.
pub fn fn_00db83c0(
    e: &mut Engine,
    node: i32,
    state: Ptr,
    multimap: Ptr,
    count: i32,
    stack: Ptr,
    carried: Ptr,
) -> i32 {
    let node_slot = |e: &Engine, node: i32| {
        e.mem
            .u32(state.addr())
            .wrapping_add((node as u32).wrapping_mul(4))
    };
    let slot = node_slot(e, node);
    if e.mem.i32(slot) != -1 {
        return count;
    }
    e.mem.set_i32(slot, -2);
    push_back_word(e, stack.addr(), node as u32);
    let mut count = count;
    let mut link = e.call(MULTI_MAP_FIRST, &args![multimap, node]).i32();
    while link != -1 {
        let entries = e.mem.u32(multimap.addr());
        let child = e
            .mem
            .i32(entries.wrapping_add((link as u32).wrapping_mul(8)));
        let child_state = e.mem.i32(node_slot(e, child));
        if child_state == -2 {
            push_back_word(e, stack.addr(), child as u32);
            e.with_stack(0x8c, |e, buffer| {
                e.call(STRING_BUFFER_CONSTRUCT, &args![buffer]);
                let size = e.mem.u32(stack.addr() + 4);
                e.mem.set_u32(stack.addr() + 4, size.wrapping_sub(1));
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let other = e.mem.u32(router + 0xc);
                let capacity = e.mem.u32(buffer.addr() + 8);
                e.mem.set_u32(buffer.addr() + 4, 0);
                if capacity & 0x8000_0000 == 0 {
                    let data = e.mem.u32(buffer.addr());
                    e.vcall(other, 0x10, &args![data, capacity & 0x3fff_ffff]);
                }
            });
        } else if child_state == -1 {
            count = fn_00db83c0(e, child, state, multimap, count, stack, carried);
        }
        let entries = e.mem.u32(multimap.addr());
        link = e
            .mem
            .i32(entries.wrapping_add((link as u32).wrapping_mul(8)) + 4);
    }
    let size = e.mem.u32(stack.addr() + 4);
    e.mem.set_u32(stack.addr() + 4, size.wrapping_sub(1));
    let slot = node_slot(e, node);
    e.mem.set_i32(slot, count);
    count.wrapping_add(1)
}

// Translated from 00db94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, seven stack words `(collected, name, version, world,
/// manager, new_classes, states)`): collects the classes `name`/`version` depends on. The uid of
/// the pair (`getUid` of `manager`) is looked up: when the world (`world`, slot `0x24` asked for
/// the class of `name`, whose slot `0xc` is its version) does not already hold that version, the
/// patch index is that of the map `new_classes` (default `-1`); `states` (default 0) says how the
/// uid has been seen: when that index is `-1` or the state is 1 the index comes from the
/// manager's own uid map at `+0x18` (default `-1`). Unless the state is 2 the record `(uid, index)`
/// (16 bytes, the fourth word left zero where the exe leaves stack garbage) is added to the sorted
/// array `collected` when its first two words are not in it (`00dba230`, `00dba4d0`). For an
/// index of zero or more, the patch's entries (8 bytes: tag, pointer) are walked from the last:
/// each of tag 6 (a dependency on another class: name and version) is collected the same way,
/// recursively.
pub fn fn_00db94b0(
    e: &mut Engine,
    collected: Ptr,
    name: u32,
    version: u32,
    world: Ptr,
    manager: Ptr,
    new_classes: Ptr,
    states: Ptr,
) {
    let uid = hk_version_patch_manager_get_uid(e, manager, name, version);
    let class = e.vcall(world.addr(), 0x24, &args![name]).u32();
    let mut index = -1;
    if class == 0 || e.vcall(class, 0xc, &args![]).u32() != version {
        index = e
            .call(U64_MAP_GET_WITH_DEFAULT, &args![new_classes, uid, u64::MAX])
            .i32();
    }
    let state = e
        .call(U64_MAP_GET_WITH_DEFAULT, &args![states, uid, 0u64])
        .i32();
    if index == -1 || state == 1 {
        index = e
            .call(
                U64_MAP_GET_WITH_DEFAULT,
                &args![manager.addr() + 0x18, uid, u64::MAX],
            )
            .i32();
    }
    if state != 2 {
        e.with_stack(0x10, |e, record| {
            e.mem.set_u32(record.addr(), uid as u32);
            e.mem.set_u32(record.addr() + 4, (uid >> 32) as u32);
            e.mem.set_i32(record.addr() + 8, index);
            if !fn_00dba230(e, collected, record) {
                fn_00dba4d0(e, collected, record);
            }
        });
    }
    if index >= 0 {
        let patches = e.mem.u32(manager.addr() + 0xc);
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        let mut entry_index = e.mem.i32(patch + 0x14) - 1;
        while entry_index >= 0 {
            let entries = e.mem.u32(patch + 0x10);
            let entry = entries.wrapping_add((entry_index as u32).wrapping_mul(8));
            if e.mem.u32(entry) == 6 {
                let dependency = e.mem.u32(entry + 4);
                let dependency_name = e.mem.u32(dependency);
                let dependency_version = e.mem.u32(dependency + 4);
                fn_00db94b0(
                    e,
                    collected,
                    dependency_name,
                    dependency_version,
                    world,
                    manager,
                    new_classes,
                    states,
                );
            }
            entry_index -= 1;
        }
    }
}

// Translated from 00db8000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `applyPatchesFinally` (Xbox PDB), cdecl with one stack word and the world in `ESI` (the Rust
/// signature takes `world` first): applies the patches in the array `patches` (data word, size
/// at `+4`; each patch `+0` old name, `+4` new name, `+8` old version, `+0xc` new version, `+0x10`
/// the entries array of `(tag, pointer)` pairs, size at `+0x14`) to the data world (slot `0x24`
/// finds a class by name, slot `0xc` creates one from a record). The patch's name is the old one,
/// else the new. A patch without an old version creates its class (a record of the name, the new
/// version, zero and an empty array of 16-byte members); a class that already exists ends
/// everything with 1. The class is then wrapped (`hkDataClass`, a one-word local) and renamed
/// (`00dc7320`) when both names are present and differ; each entry is applied by tag: 1 adds a
/// member (`00dc7d20`, four words), 2 removes one (`00dc7600`), 3 renames one (`00dc73f0`), 4 runs
/// a function (its second word's second word, called with a wrapped object) on every object of
/// the base class (`00dc73c0`), 5 casts every object of the exact class (`00dc73a0`) to the
/// class named by its first word (`00dc7dc0`), 7 sets the class's parent to the class named by
/// the second word of its record (`00dc7cc0`; no name gives a null class); other tags do
/// nothing. A patch without a new version removes the class (`00dc8870`); one whose new version
/// differs from its old (both present) sets the version (`00dc72e0`). Answers 0.
pub fn apply_patches_finally(e: &mut Engine, world: Ptr, patches: Ptr) -> u32 {
    let world_address = world.addr();
    e.with_stack(0x68, |e, frame| {
        let s = frame.addr();
        let class = s + 0xc;
        let mut patch_index = 0;
        while patch_index < e.mem.i32(patches.addr() + 4) {
            let data = e.mem.u32(patches.addr());
            let patch = e
                .mem
                .u32(data.wrapping_add((patch_index as u32).wrapping_mul(4)));
            let old_name = e.mem.u32(patch);
            let name = if old_name != 0 {
                old_name
            } else {
                e.mem.u32(patch + 4)
            };
            e.mem.set_u32(s + 0x10, name);
            if e.mem.u32(patch + 8) == 0xffff_ffff {
                let info = s + 0x48;
                e.mem.set_u32(info, name);
                e.mem.set_u32(info + 4, e.mem.u32(patch + 0xc));
                e.mem.set_u32(info + 8, 0);
                e.mem.set_u32(info + 0xc, 0);
                e.mem.set_u32(info + 0x10, 0);
                e.mem.set_u32(info + 0x14, 0x8000_0000);
                let existing = e.vcall(world_address, 0x24, &args![name]).u32();
                if existing != 0 {
                    clear_array(e, info + 0xc, 16);
                    return 1;
                }
                e.vcall(world_address, 0xc, &args![info]);
                clear_array(e, info + 0xc, 16);
            }
            let found = e.vcall(world_address, 0x24, &args![name]).u32();
            e.call(DATA_CLASS_CONSTRUCT, &args![class, found]);
            let new_name = e.mem.u32(patch + 4);
            let old_name = e.mem.u32(patch);
            if old_name != 0
                && new_name != 0
                && e.call(STRING_COMPARE, &args![old_name, new_name]).i32() != 0
            {
                e.call(WORLD_RENAME_CLASS, &args![world, class, new_name]);
            }
            let mut entry_index = 0;
            while entry_index < e.mem.i32(patch + 0x14) {
                let entries = e.mem.u32(patch + 0x10);
                let entry = entries.wrapping_add((entry_index as u32).wrapping_mul(8));
                let tag = e.mem.u32(entry);
                let record = e.mem.u32(entry + 4);
                match tag {
                    3 => {
                        let from = e.mem.u32(record);
                        let to = e.mem.u32(record + 4);
                        e.call(WORLD_RENAME_MEMBER, &args![world, class, from, to]);
                    }
                    1 => {
                        let words: [u32; 4] =
                            std::array::from_fn(|i| e.mem.u32(record + 4 * i as u32));
                        e.call(
                            WORLD_ADD_MEMBER,
                            &args![world, class, words[0], words[1], words[2], words[3]],
                        );
                    }
                    2 => {
                        let member = e.mem.u32(record);
                        e.call(WORLD_REMOVE_MEMBER, &args![world, class, member]);
                    }
                    4 => {
                        let function = e.mem.u32(record + 4);
                        let objects = s + 0x30;
                        e.mem.set_u32(objects, 0);
                        e.mem.set_u32(objects + 4, 0);
                        e.mem.set_u32(objects + 8, 0x8000_0000);
                        e.call(
                            WORLD_FIND_OBJECTS_BY_BASE_CLASS,
                            &args![world, name, objects],
                        );
                        let mut index = 0;
                        while index < e.mem.i32(objects + 4) {
                            let list = e.mem.u32(objects);
                            let object =
                                e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
                            e.call(DATA_OBJECT_CONSTRUCT, &args![s + 0x18, object]);
                            e.call(function, &args![s + 0x18]);
                            e.call(DATA_OBJECT_DESTRUCT, &args![s + 0x18]);
                            index += 1;
                        }
                        clear_array(e, objects, 4);
                    }
                    5 => {
                        let target_name = e.mem.u32(record);
                        let target = e.vcall(world_address, 0x24, &args![target_name]).u32();
                        e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x24, target]);
                        let objects = s + 0x3c;
                        e.mem.set_u32(objects, 0);
                        e.mem.set_u32(objects + 4, 0);
                        e.mem.set_u32(objects + 8, 0x8000_0000);
                        e.call(
                            WORLD_FIND_OBJECTS_BY_EXACT_CLASS,
                            &args![world, name, objects],
                        );
                        let mut index = 0;
                        while index < e.mem.i32(objects + 4) {
                            let list = e.mem.u32(objects);
                            let object =
                                e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
                            e.call(DATA_OBJECT_CONSTRUCT, &args![s + 0x20, object]);
                            e.call(WORLD_CAST_OBJECT, &args![world, s + 0x20, s + 0x24]);
                            e.call(DATA_OBJECT_DESTRUCT, &args![s + 0x20]);
                            index += 1;
                        }
                        clear_array(e, objects, 4);
                        e.call(DATA_CLASS_DESTRUCT, &args![s + 0x24]);
                    }
                    7 => {
                        let parent_name = e.mem.u32(record + 4);
                        let parent = if parent_name != 0 {
                            e.vcall(world_address, 0x24, &args![parent_name]).u32()
                        } else {
                            0
                        };
                        e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x1c, parent]);
                        e.call(WORLD_SET_CLASS_PARENT, &args![world, class, s + 0x1c]);
                        e.call(DATA_CLASS_DESTRUCT, &args![s + 0x1c]);
                    }
                    _ => {}
                }
                entry_index += 1;
            }
            let new_version = e.mem.u32(patch + 0xc);
            let old_version = e.mem.u32(patch + 8);
            if new_version == 0xffff_ffff {
                e.call(WORLD_REMOVE_CLASS, &args![world, class]);
            } else if old_version != 0xffff_ffff && old_version != new_version {
                e.call(WORLD_SET_CLASS_VERSION, &args![world, class, new_version]);
            }
            e.call(DATA_CLASS_DESTRUCT, &args![class]);
            patch_index += 1;
        }
        0
    })
}

// Translated from 00dba0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::applyPatches` (Xbox PDB), `thiscall`, `RET 8` (`world`, `registry`):
/// a null `registry` is replaced by the answer of slot `0x10` of the object at `01268b08`; the
/// registry gets a reference (`00c90510`). When the manager has patches (word at `+0x10`) but
/// its uid map (count at `+0x1c`, flag bit masked) is empty, the dependencies are recomputed
/// (`00db8670`) and an answer of 1 returns 1 at once (the reference is not released). The
/// patches to apply are prepared (`00db95c0`, into an empty local array of words) and, when that
/// answers 0, applied (`00db8000`); the registry loses its reference (`00c905b0`), the local
/// array is released and the answer is returned.
pub fn hk_version_patch_manager_apply_patches(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    registry: Ptr,
) -> u32 {
    let base = this.addr();
    let mut registry = registry.addr();
    if registry == 0 {
        let object = e.mem.u32(0x0126_8b08);
        registry = e.vcall(object, 0x10, &args![]).u32();
    }
    e.call(ADD_REFERENCE, &args![registry]);
    if e.mem.u32(base + 0x1c) & 0x7fff_ffff == 0 && e.mem.u32(base + 0x10) != 0 {
        let result = hk_version_patch_manager_recompute_patch_dependencies(e, this);
        if result == 1 {
            return 1;
        }
    }
    e.with_stack(0xc, |e, patches| {
        e.mem.set_u32(patches.addr(), 0);
        e.mem.set_u32(patches.addr() + 4, 0);
        e.mem.set_u32(patches.addr() + 8, 0x8000_0000);
        let mut result =
            hk_version_patch_manager_prepare_patches(e, this, world, Ptr::new(registry), patches);
        if result == 0 {
            result = apply_patches_finally(e, world, patches);
        }
        e.call(REMOVE_REFERENCE, &args![registry]);
        clear_array(e, patches.addr(), 4);
        result
    })
}

// Translated from 00dba1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::applyPatchesDebug` (Xbox PDB), `fastcall` with one stack word
/// (`world`, which the exe puts in `ESI`): as `applyPatches` (`00dba0d0`) without a registry:
/// the dependencies are recomputed when needed (an answer of 1 returns 1) and the manager's own
/// patch array (at `+0xc`) is applied directly (`00db8000`), answering its result.
pub fn hk_version_patch_manager_apply_patches_debug(e: &mut Engine, this: Ptr, world: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x1c) & 0x7fff_ffff == 0 && e.mem.u32(base + 0x10) != 0 {
        let result = hk_version_patch_manager_recompute_patch_dependencies(e, this);
        if result == 1 {
            return 1;
        }
    }
    apply_patches_finally(e, world, Ptr::new(base + 0xc))
}

/// `UidFromClassVersion::get` on the cache object `cache`.
fn uid_get(e: &mut Engine, cache: u32, name: u32, version: u32) -> u64 {
    hk_version_patch_manager_uid_from_class_version_get(e, Ptr::new(cache), name, version)
}

/// A map local as the exe initialises it: `(0, 0, -1)` (data, count, mask word) and its
/// `reserve(allocator, 0)` through `reserve` (`00cb2030` for the 64-bit key kind, `00cb1fe0` for
/// the pointer kind).
fn init_map(e: &mut Engine, map: u32, reserve: u32) {
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(reserve, &args![map, allocator, 0u32]);
}

/// A `hkSerializeMultiMap` local as the exe initialises it: the entry array `(0, 0, 0x80000000)`,
/// its map at `+0xc` (see [`init_map`]) and the free-entry chain head at `+0x18` set to `-1`.
fn init_multimap(e: &mut Engine, multimap: u32, reserve: u32) {
    e.mem.set_u32(multimap, 0);
    e.mem.set_u32(multimap + 4, 0);
    e.mem.set_u32(multimap + 8, 0x8000_0000);
    init_map(e, multimap + 0xc, reserve);
    e.mem.set_u32(multimap + 0x18, 0xffff_ffff);
}

/// The release of the locals of `recomputePatchDependencies` (frame base `frame`): from the
/// second pass on the two pointer multimaps (at `+0x74` and `+0x90`) and the string map (at
/// `+0xd8`); always the pointer multimap at `+0x4c`, the 64-bit one at `+0x18` and the 64-bit
/// map at `+0x40`.
fn release_recompute_locals(e: &mut Engine, frame: u32, second_pass: bool) {
    if second_pass {
        hk_serialize_multi_map_char_ptr_destructor(e, Ptr::new(frame + 0x74));
        hk_serialize_multi_map_char_ptr_destructor(e, Ptr::new(frame + 0x90));
        e.call(STRING_MAP_DESTRUCT, &args![frame + 0xd8]);
    }
    hk_serialize_multi_map_char_ptr_destructor(e, Ptr::new(frame + 0x4c));
    hk_serialize_multi_map_u64_destructor(e, Ptr::new(frame + 0x18));
    release_u64_map(e, frame + 0x40);
}

/// The first pass of `recomputePatchDependencies` (the loops before the string map exists):
/// answers `None` when everything went well, `Some(1)` after releasing the locals on an invalid
/// patch or a uid that is already known.
fn recompute_first_pass(e: &mut Engine, manager: u32, frame: u32, cache: u32) -> Option<u32> {
    let map_a = frame + 0x40;
    let multimap = frame + 0x18;
    let uid_map = manager + 0x18;
    let pass_two_index = frame + 0x10;
    let mut index: i32 = 0;
    if e.mem.i32(manager + 0x10) <= 0 {
        return None;
    }
    loop {
        e.mem.set_i32(pass_two_index, index);
        let patches = e.mem.u32(manager + 0xc);
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        if fn_00db7cb0(e, Ptr::new(patch)) == 0 {
            release_recompute_locals(e, frame, false);
            return Some(1);
        }
        let old_version = e.mem.u32(patch + 8);
        if old_version == 0xffff_ffff {
            let current = e.mem.u32(manager + 8);
            let new_name = e.mem.u32(patch + 4);
            let new_version = e.mem.u32(patch + 0xc);
            let uid = uid_get(e, current, new_name, new_version);
            let found = e.call(U64_MAP_FIND_KEY, &args![map_a, uid]).i32();
            if found <= e.mem.i32(map_a + 8) {
                release_recompute_locals(e, frame, false);
                return Some(1);
            }
            let allocator = router_allocator(e);
            let value = i64::from(index) as u64;
            e.call(U64_MAP_INSERT, &args![map_a, allocator, uid, value]);
        } else {
            let current = e.mem.u32(manager + 8);
            let old_name = e.mem.u32(patch);
            let uid = uid_get(e, current, old_name, old_version);
            let found = e.call(U64_MAP_FIND_KEY, &args![uid_map, uid]).i32();
            if found <= e.mem.i32(uid_map + 8) {
                release_recompute_locals(e, frame, false);
                return Some(1);
            }
            let allocator = router_allocator(e);
            let value = i64::from(index) as u64;
            e.call(U64_MAP_INSERT, &args![uid_map, allocator, uid, value]);
            let new_version = e.mem.u32(patch + 0xc);
            let new_name = e.mem.u32(patch + 4);
            if new_version == 0xffff_ffff
                || (new_name != 0 && e.call(STRING_COMPARE, &args![old_name, new_name]).i32() != 0)
            {
                hk_serialize_multi_map_char_ptr_insert(
                    e,
                    Ptr::new(frame + 0x4c),
                    old_name,
                    Ptr::new(pass_two_index),
                );
            }
        }
        let new_version = e.mem.u32(patch + 0xc);
        if new_version != 0xffff_ffff {
            let new_name = e.mem.u32(patch + 4);
            let name = if new_name != 0 {
                new_name
            } else {
                e.mem.u32(patch)
            };
            let uid = uid_get(e, cache, name, new_version);
            hk_serialize_multi_map_u64_insert(e, Ptr::new(multimap), uid, Ptr::new(pass_two_index));
        }
        index += 1;
        if index >= e.mem.i32(manager + 0x10) {
            return None;
        }
    }
}

// Translated from 00db8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::recomputePatchDependencies` (Xbox PDB), `fastcall`: orders the
/// manager's patches (array at `+0xc`) so that each follows the patches it depends on and
/// rebuilds the uid map at `+0x18`; answers 0, or 1 after releasing its locals on an error. The
/// exe keeps a `0xe8`-byte frame; the offsets below are its own: a 64-bit map `A` (`+0x40`), a
/// 64-bit multimap (`+0x18`, uids to patch indices) and a pointer multimap (`+0x4c`, names to
/// patch indices), and from the second pass a string map (`+0xd8`) and two pointer multimaps
/// (`+0x90` and `+0x74`).
///
/// First pass (`recompute_first_pass`): the uid map is cleared; every patch must be valid
/// (`00db7cb0`); a patch without an old version registers the uid of its new name and version in
/// `A`, a patch with one registers the uid of its old name and version in the manager's map,
/// and in both cases a uid already present is an error; the old name goes into the pointer
/// multimap when the patch has no new version or a new name that differs; a patch with a new
/// version adds the uid of its (new, else old) name and that version to the 64-bit multimap.
///
/// Second pass (`recompute_second_pass`): for each patch the first multimap's patches for the uid
/// and those of each dependency entry (tag 6) are added to the first pointer multimap (`+0x90`)
/// under the patch's own index; a patch that creates its class also adds the patches the name
/// multimap holds for its name to that multimap, or, when the uid chain has a last patch, to
/// the other one (`+0x74`), recording the name in the string map (a name seen before is an
/// error). Each key of the second multimap is then checked with the reachability walk
/// (`00db7e70`) and added to the first when it is not reachable. Third pass
/// (`recompute_third_pass`): the patch array is swapped into a local copy, the depth-first sort
/// (`00db83c0`) gives each patch its place and the patches are written in those places (a place
/// of `-1` swaps the array back and answers 1); the uid map is rebuilt from the old names and
/// versions.
pub fn hk_version_patch_manager_recompute_patch_dependencies(e: &mut Engine, this: Ptr) -> u32 {
    let manager = this.addr();
    e.with_stack(0xe8, |e, frame| {
        let s = frame.addr();
        let map_a = s + 0x40;
        let multimap_u64 = s + 0x18;
        let multimap_names = s + 0x4c;
        let uid_map = manager + 0x18;
        init_map(e, map_a, U64_MAP_RESERVE);
        init_multimap(e, multimap_u64, U64_MAP_RESERVE);
        let cache = e.mem.u32(manager + 8);
        init_multimap(e, multimap_names, MAP_RESERVE);
        e.call(U64_MAP_CLEAR, &args![uid_map]);
        if let Some(result) = recompute_first_pass(e, manager, s, cache) {
            return result;
        }
        recompute_second_pass(e, this, s, cache)
    })
}

/// Reads the word at the entry `link` of a multimap's entry array (`(value, next)` pairs).
fn multimap_entry(e: &Engine, multimap: u32, link: i32, field: u32) -> u32 {
    let entries = e.mem.u32(multimap);
    e.mem
        .u32(entries.wrapping_add((link as u32).wrapping_mul(8)) + field)
}

/// The second and third pass of `recomputePatchDependencies` (see there): the frame `s` and the
/// cache object `cache` the first pass used. Answers 0 or 1.
fn recompute_second_pass(e: &mut Engine, this: Ptr, s: u32, cache: u32) -> u32 {
    let manager = this.addr();
    let map_a = s + 0x40;
    let multimap_u64 = s + 0x18;
    let multimap_names = s + 0x4c;
    let multimap_all = s + 0x90;
    let multimap_late = s + 0x74;
    let string_map = s + 0xd8;
    let uid_map = manager + 0x18;
    let own_index = s + 0x10;
    let value_slot = s + 0xac;
    let late_value_slot = s + 0xb0;
    e.call(STRING_MAP_CONSTRUCT, &args![string_map, 0u32]);
    init_multimap(e, multimap_all, MAP_RESERVE);
    init_multimap(e, multimap_late, MAP_RESERVE);
    let mut k: i32 = 0;
    if e.mem.i32(manager + 0x10) > 0 {
        loop {
            let patches = e.mem.u32(manager + 0xc);
            let patch = e.mem.u32(patches.wrapping_add((k as u32).wrapping_mul(4)));
            let mut name = e.mem.u32(patch);
            let mut version = e.mem.u32(patch + 8);
            if version == 0xffff_ffff {
                name = e.mem.u32(patch + 4);
                version = e.mem.u32(patch + 0xc);
                if name == 0 || version == 0xffff_ffff {
                    release_recompute_locals(e, s, true);
                    return 1;
                }
            }
            let uid = uid_get(e, cache, name, version);
            e.mem.set_u32(s + 0xcc, uid as u32);
            e.mem.set_u32(s + 0xd0, (uid >> 32) as u32);
            if e.mem.u32(patch + 8) == 0xffff_ffff {
                let found = e
                    .call(U64_MAP_GET_WITH_DEFAULT, &args![map_a, uid, u64::MAX])
                    .i32();
                e.mem.set_i32(own_index, found);
            } else {
                let found = e
                    .call(U64_MAP_GET_WITH_DEFAULT, &args![uid_map, uid, u64::MAX])
                    .i32();
                e.mem.set_i32(own_index, found);
                let mut link = fn_00dba210(e, Ptr::new(multimap_u64), uid);
                while link != -1 {
                    let value = multimap_entry(e, multimap_u64, link, 0);
                    e.mem.set_u32(value_slot, value);
                    let key = e.mem.u32(own_index);
                    hk_serialize_multi_map_char_ptr_insert(
                        e,
                        Ptr::new(multimap_all),
                        key,
                        Ptr::new(value_slot),
                    );
                    link = multimap_entry(e, multimap_u64, link, 4) as i32;
                }
            }
            if e.mem.i32(patch + 0x14) > 0 {
                let mut entry_index: i32 = 0;
                loop {
                    let entries = e.mem.u32(patch + 0x10);
                    let entry = entries.wrapping_add((entry_index as u32).wrapping_mul(8));
                    if e.mem.u32(entry) == 6 {
                        let dependency = e.mem.u32(entry + 4);
                        let dependency_name = e.mem.u32(dependency);
                        let dependency_version = e.mem.u32(dependency + 4);
                        let depended = uid_get(e, cache, dependency_name, dependency_version);
                        let depended_index = e
                            .call(
                                U64_MAP_GET_WITH_DEFAULT,
                                &args![uid_map, depended, u64::MAX],
                            )
                            .i32();
                        if depended_index != -1 {
                            hk_serialize_multi_map_char_ptr_insert(
                                e,
                                Ptr::new(multimap_all),
                                depended_index as u32,
                                Ptr::new(own_index),
                            );
                        }
                        let mut link = fn_00dba210(e, Ptr::new(multimap_u64), depended);
                        while link != -1 {
                            let value = multimap_entry(e, multimap_u64, link, 0);
                            e.mem.set_u32(late_value_slot, value);
                            let key = e.mem.u32(own_index);
                            hk_serialize_multi_map_char_ptr_insert(
                                e,
                                Ptr::new(multimap_all),
                                key,
                                Ptr::new(late_value_slot),
                            );
                            link = multimap_entry(e, multimap_u64, link, 4) as i32;
                        }
                    }
                    entry_index += 1;
                    if entry_index >= e.mem.i32(patch + 0x14) {
                        break;
                    }
                }
            }
            if e.mem.u32(patch + 8) == 0xffff_ffff
                && e.call(MULTI_MAP_FIRST, &args![multimap_names, name]).i32() != -1
            {
                let first = e.call(MULTI_MAP_FIRST, &args![multimap_names, name]).i32();
                if first != -1 {
                    let last =
                        hk_version_patch_manager_find_last_patch_index_for_uid(e, this, uid, 0);
                    let target = if last == -1 {
                        let known = e
                            .call(
                                STRING_MAP_GET_WITH_DEFAULT,
                                &args![string_map, name, last as u32],
                            )
                            .i32();
                        if known != last {
                            release_recompute_locals(e, s, true);
                            return 1;
                        }
                        e.call(STRING_MAP_INSERT, &args![string_map, name, k as u32]);
                        multimap_all
                    } else {
                        multimap_late
                    };
                    let mut link = first;
                    loop {
                        let value = multimap_entry(e, multimap_names, link, 0);
                        e.mem.set_u32(late_value_slot, value);
                        if last as u32 != value {
                            let key = e.mem.u32(own_index);
                            hk_serialize_multi_map_char_ptr_insert(
                                e,
                                Ptr::new(target),
                                key,
                                Ptr::new(late_value_slot),
                            );
                        }
                        link = multimap_entry(e, multimap_names, link, 4) as i32;
                        if link == -1 {
                            break;
                        }
                    }
                }
            }
            k += 1;
            if k >= e.mem.i32(manager + 0x10) {
                break;
            }
        }
    }
    // Every key of the second pointer multimap's map (an occupied slot is one whose key is not -1)
    // is related to the patches reached from it that the first multimap does not yet hold.
    let late_map = s + 0x80;
    let mut slot = first_occupied_slot(e, late_map, 0);
    while slot <= e.mem.i32(late_map + 8) {
        let data = e.mem.u32(late_map);
        let key = e.mem.u32(data.wrapping_add((slot as u32).wrapping_mul(8)));
        let mut link = e.call(MULTI_MAP_FIRST, &args![multimap_late, key]).i32();
        while link != -1 {
            let visited = s + 0xcc;
            e.mem.set_u32(visited, 0);
            e.mem.set_u32(visited + 4, 0);
            e.mem.set_u32(visited + 8, 0xffff_ffff);
            let allocator = router_allocator(e);
            e.call(MAP_RESERVE, &args![visited, allocator, 0u32]);
            let value = multimap_entry(e, multimap_late, link, 0);
            e.mem.set_u32(late_value_slot, value);
            let reached = fn_00db7e70(e, key, value, Ptr::new(multimap_all), Ptr::new(visited));
            if reached == 0 {
                hk_serialize_multi_map_char_ptr_insert(
                    e,
                    Ptr::new(multimap_all),
                    key,
                    Ptr::new(late_value_slot),
                );
            }
            let allocator = router_allocator(e);
            e.call(MAP_CLEAR_AND_DEALLOCATE, &args![visited, allocator]);
            e.call(MAP_DESTRUCT, &args![visited]);
            link = multimap_entry(e, multimap_late, link, 4) as i32;
        }
        slot = first_occupied_slot(e, late_map, slot + 1);
    }
    recompute_third_pass(e, this, s)
}

/// The scan the exe repeats over a pointer map's slots (`(key, value)` pairs, the mask word at
/// `+8`): the first slot from `from` on whose key is not `-1`, or one past the mask when there
/// is none (the mask word, signed, is below zero for an empty map: the scan then ends at once).
fn first_occupied_slot(e: &Engine, map: u32, from: i32) -> i32 {
    let mask = e.mem.i32(map + 8);
    let data = e.mem.u32(map);
    let mut slot = from;
    if mask >= 0 && slot <= mask {
        loop {
            if e.mem.u32(data.wrapping_add((slot as u32).wrapping_mul(8))) != 0xffff_ffff {
                break;
            }
            slot += 1;
            if slot > mask {
                break;
            }
        }
    }
    slot
}

/// The third pass of `recomputePatchDependencies`: see there.
fn recompute_third_pass(e: &mut Engine, this: Ptr, s: u32) -> u32 {
    let manager = this.addr();
    let patches_array = manager + 0xc;
    let uid_map = manager + 0x18;
    let multimap_all = s + 0x90;
    let copy = s + 0x34;
    let places = s + 0x68;
    let walk = s + 0xbc;
    e.mem.set_u32(copy, 0);
    e.mem.set_u32(copy + 4, 0);
    e.mem.set_u32(copy + 8, 0x8000_0000);
    e.call(ARRAY_SWAP, &args![copy, patches_array]);
    let count = e.mem.i32(copy + 4);
    e.mem.set_u32(places, 0);
    e.mem.set_u32(places + 4, 0);
    e.mem.set_u32(places + 8, 0x8000_0000);
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(places + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, places, grown, 4u32]);
    }
    let size = e.mem.i32(places + 4);
    let data = e.mem.u32(places);
    for step in 0..count.wrapping_sub(size).max(0) {
        let at = data.wrapping_add((size.wrapping_add(step) as u32).wrapping_mul(4));
        e.mem.set_u32(at, 0xffff_ffff);
    }
    e.mem.set_i32(places + 4, count);
    e.mem.set_u32(walk, 0);
    e.mem.set_u32(walk + 4, 0);
    e.mem.set_u32(walk + 8, 0x8000_0000);
    let mut counter = 0;
    if e.mem.i32(copy + 4) > 0 {
        let mut node = 0;
        loop {
            counter = fn_00db83c0(
                e,
                node,
                Ptr::new(places),
                Ptr::new(multimap_all),
                counter,
                Ptr::new(walk),
                Ptr::new(copy),
            );
            node += 1;
            if node >= e.mem.i32(copy + 4) {
                break;
            }
        }
    }
    let count = e.mem.i32(copy + 4);
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(patches_array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let grown = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, patches_array, grown, 4u32]);
    }
    e.mem.set_i32(patches_array + 4, count);
    if e.mem.i32(copy + 4) > 0 {
        let mut index: i32 = 0;
        loop {
            let places_data = e.mem.u32(places);
            let place = e
                .mem
                .i32(places_data.wrapping_add((index as u32).wrapping_mul(4)));
            if place < 0 {
                e.call(ARRAY_SWAP, &args![patches_array, copy]);
                clear_array(e, walk, 4);
                clear_array(e, places, 4);
                clear_array(e, copy, 4);
                release_recompute_locals(e, s, true);
                return 1;
            }
            let copy_data = e.mem.u32(copy);
            let patch = e
                .mem
                .u32(copy_data.wrapping_add((index as u32).wrapping_mul(4)));
            let target = e.mem.u32(patches_array);
            e.mem
                .set_u32(target.wrapping_add((place as u32).wrapping_mul(4)), patch);
            index += 1;
            if index >= e.mem.i32(copy + 4) {
                break;
            }
        }
    }
    e.call(U64_MAP_CLEAR, &args![uid_map]);
    if e.mem.i32(manager + 0x10) > 0 {
        let mut index: i32 = 0;
        loop {
            let data = e.mem.u32(patches_array);
            let patch = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let name = e.mem.u32(patch);
            if name != 0 {
                let version = e.mem.u32(patch + 8);
                let current = e.mem.u32(manager + 8);
                let uid = uid_get(e, current, name, version);
                let allocator = router_allocator(e);
                let value = i64::from(index) as u64;
                e.call(U64_MAP_INSERT, &args![uid_map, allocator, uid, value]);
            }
            index += 1;
            if index >= e.mem.i32(manager + 0x10) {
                break;
            }
        }
    }
    clear_array(e, walk, 4);
    clear_array(e, places, 4);
    clear_array(e, copy, 4);
    release_recompute_locals(e, s, true);
    0
}

/// The release of the locals of `preparePatches` (frame base `frame`): the array of collected
/// 16-byte records at `+0x24` (when `with_collected`), the two 64-bit maps at `+0x48` and `+0x58`,
/// the scratch data world at `+0x3c`, the array of `(name, version)` pairs at `+0x14` (8-byte
/// entries) and the array of classes at `+0x30` (words).
fn release_prepare_locals(e: &mut Engine, frame: u32, with_collected: bool) {
    if with_collected {
        clear_array(e, frame + 0x24, 16);
    }
    release_u64_map(e, frame + 0x48);
    release_u64_map(e, frame + 0x58);
    e.call(WORLD_DESTRUCT, &args![frame + 0x3c]);
    clear_array(e, frame + 0x14, 8);
    clear_array(e, frame + 0x30, 4);
}

/// `hkOstream` appends the exe's error text builds: a string (`00cc01c0`) or a 32-bit integer
/// (`00cc0280`); each answers the stream.
fn append_text(e: &mut Engine, stream: u32, text: u32) -> u32 {
    e.call(OSTREAM_APPEND_TEXT, &args![stream, text]).u32()
}

fn append_int(e: &mut Engine, stream: u32, value: u32) -> u32 {
    e.call(OSTREAM_APPEND_INT, &args![stream, value]).u32()
}

/// The error `preparePatches` reports (severity 1, id `0x3f79ddb1`, `.\Version\
/// hkVersionPatchManager.cpp` line `0x2fe`) when the registry's class `class` named `name` has a
/// version other than the `version` the patches need: the stream on the frame (object at `+0x6c`,
/// 0x200-byte buffer at `+0xa0`) is made (`00cb1600`); the class's described version
/// (`00d47120`) is asked twice, the first answer `to` and the second `found`; the stream gets
/// "Found class " name " of version " found ", but expected version " version ".\nMake sure
/// required patches are registered to update the class " name " from version " version " to
/// version " to ",\nor update the class definition with the //+version(" version ")
/// annotation."; the error handler at `01268360` (slot `0xc`) is given the text; the stream is
/// destroyed.
fn report_version_mismatch(e: &mut Engine, frame: u32, class: u32, name: u32, version: u32) {
    let stream = frame + 0x6c;
    let buffer = frame + 0xa0;
    e.call(OSTREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    let to = e.call(CLASS_GET_DESCRIBED_VERSION, &args![class]).u32();
    let found = e.call(CLASS_GET_DESCRIBED_VERSION, &args![class]).u32();
    let mut out = append_text(e, stream, 0x010d_6168);
    out = append_text(e, out, name);
    out = append_text(e, out, 0x010d_6178);
    out = append_int(e, out, found);
    out = append_text(e, out, 0x010d_6188);
    out = append_int(e, out, version);
    out = append_text(e, out, 0x010d_61a0);
    out = append_text(e, out, name);
    out = append_text(e, out, 0x010d_61e4);
    out = append_int(e, out, version);
    out = append_text(e, out, 0x010d_61f4);
    out = append_int(e, out, to);
    out = append_text(e, out, 0x010d_61fc);
    out = append_int(e, out, version);
    append_text(e, out, 0x010d_6234);
    let handler = e.mem.u32(0x0126_8360);
    e.vcall(
        handler,
        0xc,
        &args![1u32, 0x3f79_ddb1u32, buffer, 0x010d_6144u32, 0x2feu32],
    );
    e.call(OSTREAM_DESTRUCT, &args![stream]);
}

/// The collecting step of `preparePatches` for the patch the sorted record array's first record
/// names (see there); answers `Some(1)` when the locals have been released after an error.
fn prepare_collected_record(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    registry: u32,
    out: Ptr,
    s: u32,
) -> Option<u32> {
    let manager = this.addr();
    let collected = s + 0x24;
    let states = s + 0x58;
    let new_classes = s + 0x48;
    let data = e.mem.u32(collected);
    let uid = u64::from(e.mem.u32(data)) | (u64::from(e.mem.u32(data + 4)) << 32);
    let index = e.mem.i32(data + 8);
    let size = e.mem.i32(collected + 4).wrapping_sub(1);
    e.mem.set_i32(collected + 4, size);
    let bytes = size.wrapping_mul(16);
    if bytes > 0 {
        let mut destination = data;
        let mut source = data + 16;
        for _ in 0..((bytes - 1) >> 3) + 1 {
            for word in 0..2 {
                let value = e.mem.u32(source + word * 4);
                e.mem.set_u32(destination + word * 4, value);
            }
            source += 8;
            destination += 8;
        }
    }
    let patches = e.mem.u32(manager + 0xc);
    let mut state = 2u32;
    if index != -1 {
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        if e.mem.u32(patch + 8) == 0xffff_ffff {
            state = 1;
        }
    }
    let allocator = router_allocator(e);
    e.call(
        U64_MAP_INSERT,
        &args![states, allocator, uid, u64::from(state)],
    );
    if index >= 0 {
        let patches = e.mem.u32(manager + 0xc);
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        push_back_word(e, out.addr(), patch);
        let patches = e.mem.u32(manager + 0xc);
        let patch = e
            .mem
            .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
        let new_name = e.mem.u32(patch + 4);
        let name = if new_name != 0 {
            new_name
        } else {
            e.mem.u32(patch)
        };
        let version = e.mem.u32(patch + 0xc);
        fn_00db94b0(
            e,
            Ptr::new(collected),
            name,
            version,
            world,
            this,
            Ptr::new(new_classes),
            Ptr::new(states),
        );
        return None;
    }
    let name = fn_00db7d40(e, this, uid as u32, (uid >> 32) as u32);
    let version = fn_00db7c90(e, this, uid as u32, (uid >> 32) as u32);
    if version == 0xffff_ffff {
        return None;
    }
    let class = e.vcall(registry, 0x10, &args![name]).u32();
    if class == 0 {
        release_prepare_locals(e, s, true);
        return Some(1);
    }
    let described = e.call(CLASS_GET_DESCRIBED_VERSION, &args![class]).u32();
    if described == version {
        let class_name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let known = e.call(WORLD_FIND_CLASS, &args![s + 0x3c, class_name]).u32();
        if known == 0 {
            e.call(WORLD_WRAP_CLASS, &args![s + 0x3c, class]);
        }
        return None;
    }
    report_version_mismatch(e, s, class, name, version);
    release_prepare_locals(e, s, true);
    Some(1)
}

/// The undoing step of `preparePatches` for `patch` (the data world at `+0x3c` of the frame `s`
/// is made to hold the class the way the patch found it): see `preparePatches`.
fn prepare_undo_patch(e: &mut Engine, patch: u32, s: u32) {
    let class = s + 0x20;
    let scratch = s + 0x3c;
    let old_name = e.mem.u32(patch);
    let new_name = e.mem.u32(patch + 4);
    let name = if new_name != 0 { new_name } else { old_name };
    let found = e.call(WORLD_FIND_CLASS, &args![scratch, name]).u32();
    e.call(DATA_CLASS_CONSTRUCT, &args![class, found]);
    if e.call(DATA_CLASS_IS_NULL, &args![class]).u32() != 0 {
        let info = s + 0x88;
        e.mem.set_u32(info + 0xc, 0);
        e.mem.set_u32(info + 0x10, 0);
        e.mem.set_u32(info + 0x14, 0x8000_0000);
        e.mem.set_u32(info, name);
        let version = e.mem.u32(patch + 8);
        e.mem.set_u32(info + 4, version);
        e.mem.set_u32(info + 8, 0);
        let created = e.call(WORLD_CREATE_CLASS, &args![scratch, info]).u32();
        e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x84, created]);
        let impl_word = e.mem.u32(s + 0x84);
        e.mem.set_u32(class, impl_word);
        e.call(DATA_CLASS_DESTRUCT, &args![s + 0x84]);
        clear_array(e, info + 0xc, 16);
    }
    let mut entry_index = e.mem.i32(patch + 0x14).wrapping_sub(1);
    while entry_index >= 0 {
        let entries = e.mem.u32(patch + 0x10);
        let entry = entries.wrapping_add((entry_index as u32).wrapping_mul(8));
        let tag = e.mem.u32(entry);
        let record = e.mem.u32(entry + 4);
        match tag {
            3 => {
                let from = e.mem.u32(record);
                let to = e.mem.u32(record + 4);
                e.call(WORLD_RENAME_MEMBER, &args![scratch, class, to, from]);
            }
            2 => {
                let words: [u32; 4] = std::array::from_fn(|i| e.mem.u32(record + 4 * i as u32));
                e.call(
                    WORLD_ADD_MEMBER,
                    &args![scratch, class, words[0], words[1], words[2], words[3]],
                );
            }
            1 => {
                let member = e.mem.u32(record);
                e.call(WORLD_REMOVE_MEMBER, &args![scratch, class, member]);
            }
            6 => {
                let depended = e.mem.u32(record);
                let known = e.call(WORLD_FIND_CLASS, &args![scratch, depended]).u32();
                e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x54, known]);
                e.call(DATA_CLASS_DESTRUCT, &args![s + 0x54]);
            }
            7 => {
                let parent_name = e.mem.u32(record);
                if parent_name != 0 {
                    let known = e.call(WORLD_FIND_CLASS, &args![scratch, parent_name]).u32();
                    e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x7c, known]);
                    e.call(WORLD_SET_CLASS_PARENT, &args![scratch, class, s + 0x7c]);
                    e.call(DATA_CLASS_DESTRUCT, &args![s + 0x7c]);
                } else {
                    e.call(DATA_CLASS_CONSTRUCT, &args![s + 0x80, 0u32]);
                    e.call(WORLD_SET_CLASS_PARENT, &args![scratch, class, s + 0x80]);
                    e.call(DATA_CLASS_DESTRUCT, &args![s + 0x80]);
                }
            }
            _ => {}
        }
        entry_index -= 1;
    }
    if e.mem.u32(patch + 8) == 0xffff_ffff {
        e.call(WORLD_REMOVE_CLASS, &args![scratch, class]);
    } else {
        if name != old_name {
            e.call(WORLD_RENAME_CLASS, &args![scratch, class, old_name]);
        }
        let version = e.mem.u32(patch + 8);
        e.call(WORLD_SET_CLASS_VERSION, &args![scratch, class, version]);
    }
    e.call(DATA_CLASS_DESTRUCT, &args![class]);
}

/// The last step of `preparePatches`: the classes of the scratch data world (found into the
/// array at `+0x24`) that the world's own classes (`classes`, an array of words at `+0x30`) do not
/// have are copied into `world` (`00dc85a0`); `world`'s class names are looked up in a string
/// map of the scratch ones and removed from it first. A second string map is made and destroyed
/// unused.
fn prepare_copy_missing_classes(e: &mut Engine, world: Ptr, s: u32) {
    let found = s + 0x24;
    let classes = s + 0x30;
    let by_name = s + 0x6c;
    let unused = s + 0x88;
    let valid = s + 0x13;
    e.mem.set_u32(found, 0);
    e.mem.set_u32(found + 4, 0);
    e.mem.set_u32(found + 8, 0x8000_0000);
    e.call(WORLD_FIND_ALL_CLASSES, &args![s + 0x3c, found]);
    e.call(STRING_MAP_CONSTRUCT, &args![by_name, 0u32]);
    e.call(STRING_MAP_CONSTRUCT, &args![unused, 0u32]);
    let mut index: i32 = 0;
    while index < e.mem.i32(found + 4) {
        let data = e.mem.u32(found);
        let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        let name = e.vcall(class, 8, &args![]).u32();
        e.call(STRING_MAP_INSERT, &args![by_name, name, class]);
        index += 1;
    }
    let mut index: i32 = 0;
    while index < e.mem.i32(classes + 4) {
        let data = e.mem.u32(classes);
        let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        let name = e.vcall(class, 8, &args![]).u32();
        let present = e
            .call(STRING_MAP_GET_WITH_DEFAULT, &args![by_name, name, 0u32])
            .u32();
        if present != 0 {
            let data = e.mem.u32(classes);
            let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let name = e.vcall(class, 8, &args![]).u32();
            e.call(STRING_MAP_REMOVE, &args![by_name, name]);
        }
        index += 1;
    }
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![by_name]).u32();
    e.call(STRING_MAP_IS_VALID, &args![by_name, valid, iterator]);
    while e.mem.u8(valid) != 0 {
        let class = e
            .call(STRING_MAP_GET_VALUE, &args![by_name, iterator])
            .u32();
        let name = e.vcall(class, 8, &args![]).u32();
        e.call(WORLD_COPY_CLASS_FROM_WORLD, &args![world, name, s + 0x3c]);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![by_name, iterator]).u32();
        e.call(STRING_MAP_IS_VALID, &args![by_name, valid, iterator]);
    }
    e.call(STRING_MAP_DESTRUCT, &args![unused]);
    e.call(STRING_MAP_DESTRUCT, &args![by_name]);
    clear_array(e, found, 4);
}

// Translated from 00db95c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVersionPatchManager::preparePatches` (Xbox PDB), `thiscall`, `RET 0xc` (`world`, `registry`,
/// `out`): works out which of the manager's patches have to run on `world` and leaves them, in
/// the order they must be applied, in the word array `out`, answering 0 (1 after an error).
///
/// The world's classes (slot `0x20` fills a word array) give `(name, version)` pairs (slots `8`
/// and `0xc` of each class). A scratch data world (`00dc8030`) and two 64-bit maps are made.
/// Each patch must be valid (`00db7cb0`); a patch without an old version registers its new uid
/// in the second map (a uid already there is an error). For each pair the dependencies are
/// collected (`00db94b0`) into a 16-byte-record array kept sorted by its word at `+8`; then its
/// first record is taken off the front repeatedly: it is entered in the first map (value 1 when
/// its index is that of a patch that creates its class, else 2), and a record with a patch
/// index adds that patch to `out` and collects what that patch depends on; a record without one
/// (`-1`) needs the registry (slot `0x10` finds a class by name): the class named by the uid
/// and the version the uid carries must match the class's described version (`00d47120`),
/// otherwise the error text is built and reported (`report_version_mismatch`) and everything
/// ends with 1; a class that matches but is not in the scratch world is wrapped into it
/// (`00dc8270`). Then the patches in `out` are undone on the scratch world from the last to the
/// first (the class found or created at the patch's old version, each entry reversed, the class
/// removed, renamed back and set to the old version). At the end the classes of the scratch world
/// that `world` lacks are copied into it. Frame offsets are those of the exe (`0x2a0` bytes).
pub fn hk_version_patch_manager_prepare_patches(
    e: &mut Engine,
    this: Ptr,
    world: Ptr,
    registry: Ptr,
    out: Ptr,
) -> u32 {
    let manager = this.addr();
    e.with_stack(0x2a0, |e, frame| {
        let s = frame.addr();
        let classes = s + 0x30;
        let pairs = s + 0x14;
        let collected = s + 0x24;
        let new_classes = s + 0x48;
        let states = s + 0x58;
        e.mem.set_u32(classes, 0);
        e.mem.set_u32(classes + 4, 0);
        e.mem.set_u32(classes + 8, 0x8000_0000);
        e.vcall(world.addr(), 0x20, &args![classes]);
        let count = e.mem.i32(classes + 4);
        e.mem.set_u32(pairs, 0);
        e.mem.set_u32(pairs + 4, 0);
        e.mem.set_u32(pairs + 8, 0x8000_0000);
        let allocator = router_allocator(e);
        let capacity = (e.mem.u32(pairs + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            e.call(ARRAY_RESERVE, &args![allocator, pairs, count, 8u32]);
        }
        let mut index: i32 = 0;
        if e.mem.i32(classes + 4) > 0 {
            loop {
                let data = e.mem.u32(classes);
                let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                let name = e.vcall(class, 8, &args![]).u32();
                let data = e.mem.u32(classes);
                let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                let version = e.vcall(class, 0xc, &args![]).u32();
                let pair_data = e.mem.u32(pairs);
                let size = e.mem.u32(pairs + 4);
                let slot = pair_data.wrapping_add(size.wrapping_mul(8));
                if slot != 0 {
                    e.mem.set_u32(slot, name);
                    e.mem.set_u32(slot + 4, version);
                }
                e.mem.set_u32(pairs + 4, size.wrapping_add(1));
                index += 1;
                if index >= e.mem.i32(classes + 4) {
                    break;
                }
            }
        }
        e.call(WORLD_CONSTRUCT, &args![s + 0x3c]);
        init_map(e, states, U64_MAP_RESERVE);
        init_map(e, new_classes, U64_MAP_RESERVE);
        let mut index: i32 = 0;
        if e.mem.i32(manager + 0x10) > 0 {
            loop {
                let patches = e.mem.u32(manager + 0xc);
                let patch = e
                    .mem
                    .u32(patches.wrapping_add((index as u32).wrapping_mul(4)));
                if fn_00db7cb0(e, Ptr::new(patch)) == 0 {
                    release_prepare_locals(e, s, false);
                    return 1;
                }
                if e.mem.u32(patch + 8) == 0xffff_ffff {
                    let cache = e.mem.u32(manager + 8);
                    let new_name = e.mem.u32(patch + 4);
                    let new_version = e.mem.u32(patch + 0xc);
                    let uid = uid_get(e, cache, new_name, new_version);
                    let found = e.call(U64_MAP_FIND_KEY, &args![new_classes, uid]).i32();
                    if found <= e.mem.i32(new_classes + 8) {
                        release_prepare_locals(e, s, false);
                        return 1;
                    }
                    let allocator = router_allocator(e);
                    let value = i64::from(index) as u64;
                    e.call(U64_MAP_INSERT, &args![new_classes, allocator, uid, value]);
                }
                index += 1;
                if index >= e.mem.i32(manager + 0x10) {
                    break;
                }
            }
        }
        e.mem.set_u32(out.addr() + 4, 0);
        e.mem.set_u32(collected, 0);
        e.mem.set_u32(collected + 4, 0);
        e.mem.set_u32(collected + 8, 0x8000_0000);
        if e.mem.i32(pairs + 4) > 0 {
            let mut index: i32 = 0;
            loop {
                let data = e.mem.u32(pairs);
                let entry = data.wrapping_add((index as u32).wrapping_mul(8));
                let name = e.mem.u32(entry);
                let version = e.mem.u32(entry + 4);
                fn_00db94b0(
                    e,
                    Ptr::new(collected),
                    name,
                    version,
                    world,
                    this,
                    Ptr::new(new_classes),
                    Ptr::new(states),
                );
                index += 1;
                if index >= e.mem.i32(pairs + 4) {
                    break;
                }
            }
            while e.mem.u32(collected + 4) != 0 {
                if let Some(result) =
                    prepare_collected_record(e, this, world, registry.addr(), out, s)
                {
                    return result;
                }
            }
        }
        clear_array(e, collected, 16);
        let mut undo = e.mem.i32(out.addr() + 4).wrapping_sub(1);
        while undo >= 0 {
            let data = e.mem.u32(out.addr());
            let patch = e.mem.u32(data.wrapping_add((undo as u32).wrapping_mul(4)));
            prepare_undo_patch(e, patch, s);
            undo -= 1;
        }
        prepare_copy_missing_classes(e, world, s);
        release_prepare_locals(e, s, false);
        0
    })
}

// Translated from 00db8540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words, plain `RET`; descriptions, not
/// confirmed; `00dba1e0` jumps to it): sets up a 0x24-byte object. The vtable `010d6138` and the
/// 16-bit word 1 at `+6` are stored, the words at `+0xc` and `+0x10` cleared and the one at
/// `+0x14` set to `0x80000000`; the `hkArray` at `+0x18` is made empty (data 0, size 0, capacity
/// word `0xffffffff`) and `00cb2030` runs on it (`thiscall`, `(allocator, 0)`, the memory
/// router's allocator and 0). Then a 0x2c-byte member comes from the same allocator (virtual
/// slot 4, no size word set): when it is not null the constructor `00d87790` runs on it and on
/// its sub-object at `+0x1c` (`thiscall`, one stack word the callee never reads, `RET 4`: at
/// `00db859b`/`00db85ab` the exe clears the low byte of its `PUSH ECX` scratch slot, which
/// still holds `this`, and pushes that slot, so the word is `this & 0xffffff00`), the words at
/// `+0x10` and `+0x14` are cleared and the one at `+0x18` set to `0x80000000` (between the two
/// constructor runs); the member is stored at `+8` (0 when the allocator gave none). Answers
/// `this`.
pub fn fn_00db8540(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, 0x010d_6138);
    e.mem.set_u16(me.wrapping_add(6), 1);
    e.mem.set_u32(me.wrapping_add(0xc), 0);
    e.mem.set_u32(me.wrapping_add(0x10), 0);
    e.mem.set_u32(me.wrapping_add(0x14), 0x8000_0000);
    let array = me.wrapping_add(0x18);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), 0xffff_ffff);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    e.call(0x00cb_2030, &args![array, allocator, 0u32]);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let member = e.vcall(allocator, 4, &args![0x2cu32]).u32();
    if member == 0 {
        e.mem.set_u32(me.wrapping_add(8), 0);
        return this;
    }
    // 00db859b-00db85c4: the ignored word is the scratch slot (`this`) with its low byte 0.
    let ignored = me & 0xffff_ff00;
    e.call(0x00d8_7790, &args![member, ignored]);
    e.mem.set_u32(member.wrapping_add(0x10), 0);
    e.mem.set_u32(member.wrapping_add(0x14), 0);
    e.mem.set_u32(member.wrapping_add(0x18), 0x8000_0000);
    e.call(0x00d8_7790, &args![member.wrapping_add(0x1c), ignored]);
    e.mem.set_u32(me.wrapping_add(8), member);
    this
}

// Translated from 00dba1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, no words; no direct caller; descriptions, not confirmed):
/// allocates 0x24 bytes from the memory router's allocator (virtual slot 4), stores the size in
/// the 16-bit word at `+4` and jumps to `00db8540` on the block. Answers the block.
pub fn fn_00dba1e0(e: &mut Engine) -> Ptr {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![0x24u32]).u32();
    e.mem.set_u16(block.wrapping_add(4), 0x24);
    fn_00db8540(e, Ptr::new(block))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db8540, fn_00db8540(Ptr) -> Ptr),
        entry!(0x00dba1e0, fn_00dba1e0() -> Ptr),
        entry!(0x00db_a580, deleting_destructor_00dba580(Ptr, u8) -> Ptr),
        entry!(0x00db_7c90, fn_00db7c90(Ptr, u32, u32) -> u32),
        entry!(0x00db_7cb0, fn_00db7cb0(Ptr) -> u32),
        entry!(0x00db_7d00, fn_00db7d00(Ptr, u32) -> u32),
        entry!(0x00db_7d40, fn_00db7d40(Ptr, u32, u32) -> u32),
        entry!(
            0x00db_7d60,
            hk_version_patch_manager_uid_from_class_version_get(Ptr, u32, u32) -> u64
        ),
        entry!(0x00db_7dd0, hk_version_patch_manager_get_uid(Ptr, u32, u32) -> u64),
        entry!(
            0x00db_7de0,
            hk_version_patch_manager_find_last_patch_index_for_uid(Ptr, u64, u32) -> i32
        ),
        entry!(0x00db_7e70, fn_00db7e70(u32, u32, Ptr, Ptr) -> u32),
        entry!(
            0x00db_7f50,
            hk_version_patch_manager_uid_from_class_version_destructor(Ptr)
        ),
        entry!(0x00db_8000, apply_patches_finally(Ptr, Ptr) -> u32),
        entry!(0x00db_83c0, fn_00db83c0(i32, Ptr, Ptr, i32, Ptr, Ptr) -> i32),
        entry!(0x00db_85e0, hk_version_patch_manager_destructor(Ptr)),
        entry!(
            0x00db_8670,
            hk_version_patch_manager_recompute_patch_dependencies(Ptr) -> u32
        ),
        entry!(0x00db_94b0, fn_00db94b0(Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00db_95c0,
            hk_version_patch_manager_prepare_patches(Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00db_a0d0, hk_version_patch_manager_apply_patches(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00db_a1a0, hk_version_patch_manager_apply_patches_debug(Ptr, Ptr) -> u32),
        entry!(0x00db_a210, fn_00dba210(Ptr, u64) -> i32),
        entry!(0x00db_a230, fn_00dba230(Ptr, Ptr) -> bool),
        entry!(0x00db_a270, fn_00dba270(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00db_a320, fn_00dba320(Ptr, u32, i32, u32, i32)),
        entry!(0x00db_a340, fn_00dba340(Ptr, u32, i32, u32)),
        entry!(
            0x00db_a360,
            hk_serialize_multi_map_u64_insert(Ptr, u64, Ptr)
        ),
        entry!(0x00db_a3d0, hk_serialize_multi_map_u64_destructor(Ptr)),
        entry!(0x00db_a430, hk_serialize_multi_map_char_ptr_destructor(Ptr)),
        entry!(0x00db_a490, fn_00dba490(Ptr)),
        entry!(0x00db_a4d0, fn_00dba4d0(Ptr, Ptr)),
        entry!(0x00db_e710, hk_serialize_multi_map_u64_get_free_index(Ptr) -> i32),
        entry!(
            0x00dc_8d20,
            hk_version_patch_manager_uid_from_class_version_cache(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00dc_9ec0,
            hk_serialize_multi_map_char_ptr_insert(Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out the block that goes with the size it is asked for
    /// (`blocks`: (size, block)), 0 for any other size. Returns the allocator.
    fn install_allocator_by_size(e: &mut Engine, blocks: &'static [(u32, u32)]) -> u32 {
        for (_, block) in blocks {
            if *block != 0 {
                e.map(*block, 0x100);
            }
        }
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| Ret {
            eax: blocks
                .iter()
                .find(|(size, _)| *size == a[1])
                .map_or(0, |(_, block)| *block),
            ..Ret::default()
        });
        allocator
    }

    fn calls_of_00db8540(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00db8540_sets_up_the_object_and_its_member() {
        let mut e = Engine::new();
        let allocator = install_allocator_by_size(&mut e, &[(0x2c, 0x0801_0000)]);
        e.register(0x00cb_2030, |_, _| Ret::default());
        e.register(0x00d8_7790, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        let answer = e.call(0x00db_8540, &args![this]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), 0x010d_6138);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0x0801_0000);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0xffff_ffff);
        assert_eq!(
            calls_of_00db8540(&e, 0x00cb_2030),
            vec![vec![this + 0x18, allocator, 0]]
        );
        // The ignored stack word is the exe's scratch slot: `this` with its low byte 0.
        let ignored = this & 0xffff_ff00;
        assert_eq!(
            calls_of_00db8540(&e, 0x00d8_7790),
            vec![vec![0x0801_0000, ignored], vec![0x0801_001c, ignored]]
        );
        assert_eq!(e.mem.u32(0x0801_0010), 0);
        assert_eq!(e.mem.u32(0x0801_0014), 0);
        assert_eq!(e.mem.u32(0x0801_0018), 0x8000_0000);
    }

    #[test]
    fn fn_00db8540_stores_zero_when_the_allocator_gives_no_member() {
        let mut e = Engine::new();
        install_allocator_by_size(&mut e, &[]);
        e.register(0x00cb_2030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + 8, 0x1234);
        let answer = e.call(0x00db_8540, &args![this]).u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(calls_of_00db8540(&e, 0x00d8_7790).is_empty());
    }

    #[test]
    fn fn_00dba1e0_allocates_the_object_and_sets_it_up() {
        let mut e = Engine::new();
        install_allocator_by_size(&mut e, &[(0x24, 0x0800_0000), (0x2c, 0x0801_0000)]);
        e.register(0x00cb_2030, |_, _| Ret::default());
        e.register(0x00d8_7790, |_, _| Ret::default());
        let object = e.call(0x00db_a1e0, &args![]).u32();
        assert_eq!(object, 0x0800_0000);
        assert_eq!(e.mem.u16(object + 4), 0x24);
        assert_eq!(e.mem.u32(object), 0x010d_6138);
        assert_eq!(e.mem.u32(object + 8), 0x0801_0000);
    }

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dba580_frees_only_when_bit_zero_is_set() {
        check(0x00db_a580, 0x00db_85e0);
    }
}

#[cfg(test)]
mod patch_tests {
    use super::*;

    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;
    type MapStore = Rc<RefCell<HashMap<(u32, u64), u64>>>;
    type WordStore = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    const BLOCK_FREE: u32 = 0x0900_0008;
    const BUFFER_FREE: u32 = 0x0900_0010;
    const OTHER_BUFFER_FREE: u32 = 0x0900_0020;
    const NONE: u32 = 0xffff_ffff;
    /// Doubles for the slots of a class object made by [`Rig::class_object`]: the name word at
    /// `+4` and the version word at `+8`.
    const CLASS_NAME_FN: u32 = 0x0900_0300;
    const CLASS_VERSION_FN: u32 = 0x0900_0304;

    /// The engine with the memory router and the callees outside this file replaced by doubles:
    /// the allocators log their frees; the array growth, move and swap functions work on memory;
    /// the 64-bit and pointer maps and the string map keep their entries in stores keyed by the
    /// map's address; every double notes its call (tag and argument words) in `calls`.
    struct Rig {
        e: Engine,
        calls: Calls,
        maps: MapStore,
    }

    fn note(calls: &Calls, tag: &'static str, a: &[u32]) {
        calls.borrow_mut().push((tag, a.to_vec()));
    }

    fn grow(e: &mut Engine, array: u32, capacity: u32, entry: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let block = e.mem.alloc(capacity * entry);
        if size > 0 {
            let bytes = e.mem.bytes(old, size * entry);
            e.mem.write(block, &bytes);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 8, capacity);
    }

    fn key64(a: &[u32]) -> u64 {
        u64::from(a[0]) | (u64::from(a[1]) << 32)
    }

    impl Rig {
        fn new() -> Rig {
            let mut e = Engine::new();
            e.map(0x0100_0000, 0x0030_0000);
            let calls: Calls = Rc::default();
            let maps: MapStore = Rc::default();
            let words: WordStore = Rc::default();
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 8, BLOCK_FREE);
            e.mem.set_u32(vtable + 0x10, BUFFER_FREE);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let other_vtable = e.mem.alloc(0x20);
            e.mem.set_u32(other_vtable + 0x10, OTHER_BUFFER_FREE);
            let other = e.mem.alloc(0x10);
            e.mem.set_u32(other, other_vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0xc, other);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            for (addr, tag) in [
                (BLOCK_FREE, "block_free"),
                (BUFFER_FREE, "free"),
                (OTHER_BUFFER_FREE, "other_free"),
            ] {
                let c = Rc::clone(&calls);
                e.register_double(addr, move |_, a| {
                    note(&c, tag, &a[1..]);
                    Ret::default()
                });
            }
            let c = Rc::clone(&calls);
            e.register_double(ARRAY_RESERVE, move |e, a| {
                note(&c, "reserve", &a[1..]);
                grow(e, a[1], a[2], a[3]);
                Ret::default()
            });
            let c = Rc::clone(&calls);
            e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
                note(&c, "reserve_more", &a[1..]);
                let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff).max(2) * 2;
                grow(e, a[1], capacity, a[2]);
                Ret::default()
            });
            e.register_double(MEM_MOVE, |e, a| {
                if a[2] != 0 {
                    let bytes = e.mem.bytes(a[1], a[2]);
                    e.mem.write(a[0], &bytes);
                }
                Ret::default()
            });
            e.register_double(ARRAY_SWAP, |e, a| {
                for word in 0..3 {
                    let mine = e.mem.u32(a[0] + word * 4);
                    let theirs = e.mem.u32(a[1] + word * 4);
                    e.mem.set_u32(a[0] + word * 4, theirs);
                    e.mem.set_u32(a[1] + word * 4, mine);
                }
                Ret::default()
            });
            let m = Rc::clone(&maps);
            e.register_double(U64_MAP_GET_WITH_DEFAULT, move |_, a| {
                let found = m.borrow().get(&(a[0], key64(&a[1..]))).copied();
                Ret {
                    eax: found.map_or(a[3], |v| v as u32),
                    ..Ret::default()
                }
            });
            let m = Rc::clone(&maps);
            e.register_double(U64_MAP_FIND_KEY, move |e, a| {
                let present = m.borrow().contains_key(&(a[0], key64(&a[1..])));
                let mask = e.mem.i32(a[0] + 8);
                Ret {
                    eax: if present { 0 } else { (mask + 1) as u32 },
                    ..Ret::default()
                }
            });
            let m = Rc::clone(&maps);
            e.register_double(U64_MAP_INSERT, move |_, a| {
                let value = u64::from(a[4]) | (u64::from(a[5]) << 32);
                m.borrow_mut().insert((a[0], key64(&a[2..])), value);
                Ret::default()
            });
            let m = Rc::clone(&maps);
            let c = Rc::clone(&calls);
            e.register_double(U64_MAP_CLEAR_AND_DEALLOCATE, move |_, a| {
                note(&c, "map_release", &a[..1]);
                m.borrow_mut().retain(|k, _| k.0 != a[0]);
                Ret::default()
            });
            let m = Rc::clone(&maps);
            let c = Rc::clone(&calls);
            e.register_double(U64_MAP_CLEAR, move |_, a| {
                note(&c, "map_clear", &a[..1]);
                m.borrow_mut().retain(|k, _| k.0 != a[0]);
                Ret::default()
            });
            e.register_double(U64_MAP_RESERVE, |e, a| {
                e.mem.set_u32(a[0] + 8, 3);
                Ret::default()
            });
            e.register_double(MAP_RESERVE, |e, a| {
                let table = e.mem.alloc(4 * 8);
                for slot in 0..4 {
                    e.mem.set_u32(table + slot * 8, NONE);
                }
                e.mem.set_u32(a[0], table);
                e.mem.set_u32(a[0] + 8, 3);
                Ret::default()
            });
            let m = Rc::clone(&maps);
            e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
                let found = m.borrow().get(&(a[0], u64::from(a[1]))).copied();
                Ret {
                    eax: found.map_or(a[2], |v| v as u32),
                    ..Ret::default()
                }
            });
            let m = Rc::clone(&maps);
            e.register_double(MAP_FIND_KEY, move |e, a| {
                let present = m.borrow().contains_key(&(a[0], u64::from(a[1])));
                let mask = e.mem.i32(a[0] + 8);
                Ret {
                    eax: if present { 0 } else { (mask + 1) as u32 },
                    ..Ret::default()
                }
            });
            let m = Rc::clone(&maps);
            e.register_double(MAP_INSERT, move |e, a| {
                m.borrow_mut()
                    .insert((a[0], u64::from(a[2])), u64::from(a[3]));
                let table = e.mem.u32(a[0]);
                if table != 0 {
                    for slot in 0..4 {
                        let key = e.mem.u32(table + slot * 8);
                        if key == NONE || key == a[2] {
                            e.mem.set_u32(table + slot * 8, a[2]);
                            e.mem.set_u32(table + slot * 8 + 4, a[3]);
                            break;
                        }
                    }
                }
                Ret::default()
            });
            let m = Rc::clone(&maps);
            let c = Rc::clone(&calls);
            e.register_double(MAP_CLEAR_AND_DEALLOCATE, move |_, a| {
                note(&c, "ptr_map_release", &a[..1]);
                m.borrow_mut().retain(|k, _| k.0 != a[0]);
                Ret::default()
            });
            e.register_double(MAP_DESTRUCT, |_, _| Ret::default());
            let m = Rc::clone(&maps);
            e.register_double(MULTI_MAP_FIRST, move |_, a| {
                let found = m.borrow().get(&(a[0] + 0xc, u64::from(a[1]))).copied();
                Ret {
                    eax: found.map_or(NONE, |v| v as u32),
                    ..Ret::default()
                }
            });
            for (addr, tag) in [
                (STRING_MAP_CONSTRUCT, "string_map_construct"),
                (STRING_MAP_DESTRUCT, "string_map_destruct"),
            ] {
                let c = Rc::clone(&calls);
                e.register_double(addr, move |_, a| {
                    note(&c, tag, &a[..1]);
                    Ret::default()
                });
            }
            let w = Rc::clone(&words);
            let c = Rc::clone(&calls);
            e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |_, a| {
                note(&c, "string_get", a);
                let found = w.borrow().get(&(a[0], a[1])).copied();
                Ret {
                    eax: found.unwrap_or(a[2]),
                    ..Ret::default()
                }
            });
            let w = Rc::clone(&words);
            let c = Rc::clone(&calls);
            e.register_double(STRING_MAP_INSERT, move |_, a| {
                note(&c, "string_insert", a);
                w.borrow_mut().insert((a[0], a[1]), a[2]);
                Ret::default()
            });
            let w = Rc::clone(&words);
            e.register_double(STRING_MAP_REMOVE, move |_, a| {
                w.borrow_mut().remove(&(a[0], a[1]));
                Ret::default()
            });
            let w = Rc::clone(&words);
            e.register_double(STRING_MAP_GET_OR_INSERT, move |_, a| {
                let value = *w.borrow_mut().entry((a[0], a[1])).or_insert(a[2]);
                Ret {
                    eax: value,
                    ..Ret::default()
                }
            });
            let c = Rc::clone(&calls);
            e.register_double(STRING_DUPLICATE, move |_, a| {
                note(&c, "string_dup", a);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
            e.register_double(STRING_COMPARE, |_, a| Ret {
                eax: u32::from(a[0] != a[1]),
                ..Ret::default()
            });
            e.register_double(UID_CLASS_NAME, |e, a| {
                let data = e.mem.u32(a[0] + 0x10);
                Ret {
                    eax: e.mem.u32(data + a[1] * 4),
                    ..Ret::default()
                }
            });
            e.register_double(UID_VERSION, |_, a| Ret {
                eax: a[2],
                ..Ret::default()
            });
            e.register_double(CLASS_NAME_FN, |e, a| Ret {
                eax: e.mem.u32(a[0] + 4),
                ..Ret::default()
            });
            e.register_double(CLASS_VERSION_FN, |e, a| Ret {
                eax: e.mem.u32(a[0] + 8),
                ..Ret::default()
            });
            Rig { e, calls, maps }
        }

        /// The calls noted so far with the given tag.
        fn calls_tagged(&self, tag: &str) -> Vec<Vec<u32>> {
            self.calls
                .borrow()
                .iter()
                .filter(|(t, _)| *t == tag)
                .map(|(_, a)| a.clone())
                .collect()
        }

        fn tags(&self) -> Vec<&'static str> {
            self.calls.borrow().iter().map(|(t, _)| *t).collect()
        }

        fn clear_calls(&self) {
            self.calls.borrow_mut().clear();
        }

        /// A distinct address standing for a string.
        fn name(&mut self) -> u32 {
            self.e.mem.alloc(8)
        }

        /// An array `(data, size, capacity word)` of `words`, with room for `capacity`.
        fn word_array(&mut self, words: &[u32], capacity: u32) -> u32 {
            let array = self.e.mem.alloc(0xc);
            let data = self.e.mem.alloc(capacity.max(1) * 4);
            for (i, word) in words.iter().enumerate() {
                self.e.mem.set_u32(data + 4 * i as u32, *word);
            }
            self.e.mem.set_u32(array, data);
            self.e.mem.set_u32(array + 4, words.len() as u32);
            self.e.mem.set_u32(array + 8, capacity);
            array
        }

        /// The uid cache object (`+0x10` the interned names array) with room for 16 names.
        fn cache(&mut self) -> u32 {
            let cache = self.e.mem.alloc(0x2c);
            let data = self.e.mem.alloc(16 * 4);
            self.e.mem.set_u32(cache + 0x10, data);
            self.e.mem.set_u32(cache + 0x18, 16);
            cache
        }

        /// A patch manager: `cache` at `+8`, the patches array at `+0xc` and the uid map at
        /// `+0x18` (mask word 3).
        fn manager(&mut self, cache: u32, patches: &[u32]) -> u32 {
            let manager = self.e.mem.alloc(0x30);
            let array = self.word_array(patches, 8);
            for word in 0..3 {
                let value = self.e.mem.u32(array + word * 4);
                self.e.mem.set_u32(manager + 0xc + word * 4, value);
            }
            self.e.mem.set_u32(manager + 8, cache);
            self.e.mem.set_u32(manager + 0x20, 3);
            manager
        }

        /// A patch record with its entries `(tag, record)`.
        fn patch(
            &mut self,
            old_name: u32,
            new_name: u32,
            old_version: u32,
            new_version: u32,
            entries: &[(u32, u32)],
        ) -> u32 {
            let patch = self.e.mem.alloc(0x18);
            let data = self.e.mem.alloc(8 * entries.len().max(1) as u32);
            for (i, (tag, record)) in entries.iter().enumerate() {
                self.e.mem.set_u32(data + 8 * i as u32, *tag);
                self.e.mem.set_u32(data + 8 * i as u32 + 4, *record);
            }
            for (i, word) in [old_name, new_name, old_version, new_version]
                .iter()
                .enumerate()
            {
                self.e.mem.set_u32(patch + 4 * i as u32, *word);
            }
            self.e.mem.set_u32(patch + 0x10, data);
            self.e.mem.set_u32(patch + 0x14, entries.len() as u32);
            patch
        }

        /// A record of `words` words.
        fn record(&mut self, words: &[u32]) -> u32 {
            let record = self.e.mem.alloc(4 * words.len() as u32);
            for (i, word) in words.iter().enumerate() {
                self.e.mem.set_u32(record + 4 * i as u32, *word);
            }
            record
        }

        /// The uid the cache gives `(name, version)`.
        fn uid(&mut self, cache: u32, name: u32, version: u32) -> u64 {
            hk_version_patch_manager_uid_from_class_version_get(
                &mut self.e,
                Ptr::new(cache),
                name,
                version,
            )
        }

        fn map_value(&self, map: u32, key: u64) -> Option<u64> {
            self.maps.borrow().get(&(map, key)).copied()
        }

        fn set_map_value(&self, map: u32, key: u64, value: u64) {
            self.maps.borrow_mut().insert((map, key), value);
        }

        /// A class object whose slot `8` answers `name` and slot `0xc` `version`.
        fn class_object(&mut self, name: u32, version: u32) -> u32 {
            let class = self.object_with(&[(8, CLASS_NAME_FN), (0xc, CLASS_VERSION_FN)]);
            self.e.mem.set_u32(class + 4, name);
            self.e.mem.set_u32(class + 8, version);
            class
        }

        /// A multimap of the pointer kind with room for 16 entries.
        fn multimap(&mut self) -> u32 {
            let multimap = self.e.mem.alloc(0x1c);
            let data = self.e.mem.alloc(16 * 8);
            self.e.mem.set_u32(multimap, data);
            self.e.mem.set_u32(multimap + 8, 16);
            self.e.mem.set_u32(multimap + 0x14, 3);
            self.e.mem.set_u32(multimap + 0x18, NONE);
            multimap
        }

        /// Adds `value` to the chain of `key` in a pointer multimap.
        fn link(&mut self, multimap: u32, key: u32, value: u32) {
            let slot = self.record(&[value]);
            hk_serialize_multi_map_char_ptr_insert(
                &mut self.e,
                Ptr::new(multimap),
                key,
                Ptr::new(slot),
            );
        }
    }

    impl Rig {
        /// An array `(data, size, capacity word)` of 16-byte records.
        fn record_array(&mut self, records: &[[u32; 4]], capacity: u32) -> u32 {
            let array = self.e.mem.alloc(0xc);
            let data = self.e.mem.alloc(capacity.max(1) * 16);
            for (i, record) in records.iter().enumerate() {
                for (j, word) in record.iter().enumerate() {
                    self.e
                        .mem
                        .set_u32(data + 16 * i as u32 + 4 * j as u32, *word);
                }
            }
            self.e.mem.set_u32(array, data);
            self.e.mem.set_u32(array + 4, records.len() as u32);
            self.e.mem.set_u32(array + 8, capacity);
            array
        }

        /// The first word of each record of an array of 16-byte records.
        fn first_words(&self, array: u32) -> Vec<u32> {
            let data = self.e.mem.u32(array);
            (0..self.e.mem.u32(array + 4))
                .map(|i| self.e.mem.u32(data + 16 * i))
                .collect()
        }

        /// The words of an array `(data, size, capacity word)` of words.
        fn words_of(&self, array: u32) -> Vec<u32> {
            let data = self.e.mem.u32(array);
            (0..self.e.mem.u32(array + 4))
                .map(|i| self.e.mem.u32(data + 4 * i))
                .collect()
        }

        /// An object whose vtable holds the `(slot offset, function address)` pairs.
        fn object_with(&mut self, slots: &[(u32, u32)]) -> u32 {
            let vtable = self.e.mem.alloc(0x40);
            for (slot, function) in slots {
                self.e.mem.set_u32(vtable + slot, *function);
            }
            let object = self.e.mem.alloc(0x10);
            self.e.mem.set_u32(object, vtable);
            object
        }

        /// Doubles for the string map iteration over `values`: the iterators count from 1 and
        /// `easyFree` notes the blocks it is given.
        fn install_iteration(&mut self, values: &[u32]) {
            let count = values.len() as u32;
            self.e.register_double(STRING_MAP_GET_ITERATOR, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
            self.e.register_double(STRING_MAP_IS_VALID, move |e, a| {
                e.mem.set_u8(a[1], u8::from(a[2] <= count));
                Ret::default()
            });
            let values = values.to_vec();
            self.e
                .register_double(STRING_MAP_GET_VALUE, move |_, a| Ret {
                    eax: values[a[1] as usize - 1],
                    ..Ret::default()
                });
            self.e.register_double(STRING_MAP_GET_NEXT, |_, a| Ret {
                eax: a[1] + 1,
                ..Ret::default()
            });
            let calls = Rc::clone(&self.calls);
            self.e.register_double(EASY_FREE, move |_, a| {
                note(&calls, "easy_free", a);
                Ret::default()
            });
        }
    }

    #[test]
    fn fn_00db7c90_asks_the_version_word_function_of_the_cache() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let calls = Rc::clone(&rig.calls);
        rig.e.register_double(UID_VERSION, move |_, a| {
            note(&calls, "version", a);
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
        let got = rig.e.call(0x00db_7c90, &args![manager, 7u32, 9u32]).u32();
        assert_eq!(got, 0x55);
        assert_eq!(rig.calls_tagged("version"), vec![vec![cache, 7, 9]]);
    }

    #[test]
    fn fn_00db7cb0_checks_the_names_each_kind_of_patch_needs() {
        let mut rig = Rig::new();
        let n = rig.name();
        // (old name, new name, old version, new version, valid)
        let table = [
            (0, n, NONE, 2, 1),
            (n, n, NONE, 2, 0),
            (0, 0, NONE, 2, 0),
            (0, n, NONE, NONE, 0),
            (n, 0, 1, NONE, 1),
            (n, n, 1, NONE, 0),
            (0, 0, 1, NONE, 0),
            (n, n, 1, 2, 1),
            (n, 0, 1, 2, 1),
            (0, n, 1, 2, 0),
        ];
        for (old_name, new_name, old_version, new_version, valid) in table {
            let patch = rig.patch(old_name, new_name, old_version, new_version, &[]);
            let got = rig.e.call(0x00db_7cb0, &args![patch]).u32();
            assert_eq!(
                got, valid,
                "{old_name:x} {new_name:x} {old_version:x} {new_version:x}"
            );
        }
    }

    #[test]
    fn fn_00db7d00_interns_a_string_once() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let text = rig.name();
        assert_eq!(rig.e.call(0x00db_7d00, &args![cache, 0u32]).u32(), 0);
        assert!(rig.calls.borrow().is_empty());
        let first = rig.e.call(0x00db_7d00, &args![cache, text]).u32();
        assert_eq!(first, text);
        assert_eq!(
            rig.tags(),
            vec!["string_get", "string_dup", "string_insert"]
        );
        rig.clear_calls();
        let second = rig.e.call(0x00db_7d00, &args![cache, text]).u32();
        assert_eq!(second, text);
        assert_eq!(rig.tags(), vec!["string_get"]);
        assert_eq!(
            rig.calls_tagged("string_get"),
            vec![vec![cache + 0x1c, text, 0]]
        );
    }

    #[test]
    fn fn_00db7d40_asks_the_class_name_function_of_the_cache() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let calls = Rc::clone(&rig.calls);
        rig.e.register_double(UID_CLASS_NAME, move |_, a| {
            note(&calls, "class_name", a);
            Ret {
                eax: 0x66,
                ..Ret::default()
            }
        });
        let got = rig.e.call(0x00db_7d40, &args![manager, 4u32, 8u32]).u32();
        assert_eq!(got, 0x66);
        assert_eq!(rig.calls_tagged("class_name"), vec![vec![cache, 4, 8]]);
    }

    #[test]
    fn uid_from_class_version_get_numbers_the_interned_names() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        rig.e.mem.set_u32(cache + 0x18, 1);
        let (a, b) = (rig.name(), rig.name());
        let first = rig.e.call(0x00db_7d60, &args![cache, a, 3u32]).u64();
        assert_eq!(first, 3u64 << 32);
        let second = rig.e.call(0x00db_7d60, &args![cache, b, 5u32]).u64();
        assert_eq!(second, (5u64 << 32) | 1);
        let again = rig.e.call(0x00db_7d60, &args![cache, a, 9u32]).u64();
        assert_eq!(again, 9u64 << 32);
        assert_eq!(rig.words_of(cache + 0x10), vec![a, b]);
        assert_eq!(
            rig.calls_tagged("reserve_more"),
            vec![vec![cache + 0x10, 4]]
        );
    }

    #[test]
    fn hk_version_patch_manager_get_uid_asks_the_cache() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let (a, b) = (rig.name(), rig.name());
        let first = rig.e.call(0x00db_7dd0, &args![manager, a, 3u32]).u64();
        assert_eq!(first, 3u64 << 32);
        let second = rig.e.call(0x00db_7dd0, &args![manager, b, 4u32]).u64();
        assert_eq!(second, (4u64 << 32) | 1);
        assert_eq!(rig.words_of(cache + 0x10), vec![a, b]);
    }

    #[test]
    fn find_last_patch_index_for_uid_follows_the_chain() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let (a, b, z) = (rig.name(), rig.name(), rig.name());
        let p0 = rig.patch(a, a, 1, 2, &[]);
        let p1 = rig.patch(a, b, 2, 3, &[]);
        let p2 = rig.patch(b, b, 3, NONE, &[]);
        let manager = rig.manager(cache, &[p0, p1, p2]);
        let u0 = rig.uid(cache, a, 1);
        let u1 = rig.uid(cache, a, 2);
        let u2 = rig.uid(cache, b, 3);
        let unknown = rig.uid(cache, z, 1);
        rig.set_map_value(manager + 0x18, u0, 0);
        rig.set_map_value(manager + 0x18, u1, 1);
        rig.set_map_value(manager + 0x18, u2, 2);
        let find = |rig: &mut Rig, uid: u64, flag: u32| {
            rig.e.call(0x00db_7de0, &args![manager, uid, flag]).i32()
        };
        assert_eq!(find(&mut rig, u0, 0), 1);
        assert_eq!(find(&mut rig, u0, 1), 2);
        assert_eq!(find(&mut rig, u2, 0), 2);
        assert_eq!(find(&mut rig, unknown, 0), -1);
    }

    #[test]
    fn fn_00db7e70_walks_the_multimap() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        rig.link(multimap, 1, 2);
        rig.link(multimap, 2, 3);
        rig.link(multimap, 1, 4);
        let reach = |rig: &mut Rig, target: u32, node: u32, visited: u32| {
            rig.e
                .call(0x00db_7e70, &args![target, node, multimap, visited])
                .u32()
        };
        let visited = rig.record(&[0, 0, 3]);
        assert_eq!(reach(&mut rig, 3, 1, visited), 1);
        assert_eq!(rig.map_value(visited, 1), Some(0));
        assert_eq!(rig.map_value(visited, 2), Some(0));
        let fresh = rig.record(&[0, 0, 3]);
        assert_eq!(reach(&mut rig, 9, 1, fresh), 0);
        let seen = rig.record(&[0, 0, 3]);
        rig.set_map_value(seen, 1, 0);
        assert_eq!(reach(&mut rig, 3, 1, seen), 0);
        assert_eq!(rig.map_value(seen, 2), None);
    }

    #[test]
    fn uid_from_class_version_destructor_frees_the_interned_strings() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        rig.e.mem.set_u32(cache + 0x14, 2);
        rig.install_iteration(&[0x100, 0x200]);
        let data = rig.e.mem.u32(cache + 0x10);
        rig.e.call(0x00db_7f50, &args![cache]);
        assert_eq!(
            rig.tags(),
            vec![
                "easy_free",
                "easy_free",
                "string_map_destruct",
                "free",
                "string_map_destruct"
            ]
        );
        let freed: Vec<u32> = rig.calls_tagged("easy_free").iter().map(|a| a[1]).collect();
        assert_eq!(freed, vec![0x100, 0x200]);
        assert_eq!(
            rig.calls_tagged("string_map_destruct"),
            vec![vec![cache + 0x1c], vec![cache]]
        );
        assert_eq!(rig.calls_tagged("free"), vec![vec![data, 64]]);
        assert_eq!(rig.e.mem.u32(cache + 0x14), 0);
        assert_eq!(rig.e.mem.u32(cache + 0x10), 0);
        assert_eq!(rig.e.mem.u32(cache + 0x18), 0x8000_0000);
    }

    #[test]
    fn hk_version_patch_manager_destructor_releases_the_members() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        rig.install_iteration(&[]);
        let manager = rig.manager(cache, &[]);
        let patch_data = rig.e.mem.u32(manager + 0xc);
        rig.e.mem.set_u32(manager, 0x1111);
        rig.e.call(0x00db_85e0, &args![manager]);
        assert_eq!(rig.e.mem.u32(manager), 0x0102_e388);
        assert_eq!(
            rig.tags(),
            vec![
                "string_map_destruct",
                "free",
                "string_map_destruct",
                "block_free",
                "map_release",
                "free"
            ]
        );
        assert_eq!(rig.calls_tagged("block_free"), vec![vec![cache, 0x2c]]);
        assert_eq!(rig.calls_tagged("map_release"), vec![vec![manager + 0x18]]);
        assert_eq!(rig.calls_tagged("free").last(), Some(&vec![patch_data, 32]));
        assert_eq!(rig.e.mem.u32(manager + 0xc), 0);
        assert_eq!(rig.e.mem.u32(manager + 0x14), 0x8000_0000);

        let mut rig = Rig::new();
        let manager = rig.manager(0, &[]);
        rig.e.call(0x00db_85e0, &args![manager]);
        assert_eq!(rig.tags(), vec!["map_release", "free"]);
    }

    #[test]
    fn fn_00dba210_reads_the_chain_head() {
        let mut rig = Rig::new();
        let multimap = rig.e.mem.alloc(0x1c);
        rig.set_map_value(multimap + 0xc, 0x1_0000_0002, 5);
        let head = rig
            .e
            .call(0x00db_a210, &args![multimap, 0x1_0000_0002u64])
            .i32();
        assert_eq!(head, 5);
        let none = rig.e.call(0x00db_a210, &args![multimap, 7u64]).i32();
        assert_eq!(none, -1);
    }

    #[test]
    fn fn_00dba230_finds_a_record_by_its_first_two_words() {
        let mut rig = Rig::new();
        let array = rig.record_array(&[[1, 2, 3, 0], [4, 5, 6, 0]], 4);
        let probe = |rig: &mut Rig, a: u32, b: u32| {
            let key = rig.record(&[a, b, 0, 0]);
            rig.e.call(0x00db_a230, &args![array, key]).bool()
        };
        assert!(probe(&mut rig, 4, 5));
        assert!(!probe(&mut rig, 4, 6));
        assert!(!probe(&mut rig, 9, 2));
        let empty = rig.record_array(&[], 4);
        let key = rig.record(&[1, 2, 0, 0]);
        assert!(!rig.e.call(0x00db_a230, &args![empty, key]).bool());
    }

    #[test]
    fn fn_00dba270_splices_records() {
        let mut rig = Rig::new();
        let array = rig.record_array(&[[1, 0, 0, 0], [2, 0, 0, 0], [3, 0, 0, 0], [4, 0, 0, 0]], 8);
        let source = rig.record_array(&[[0x10, 1, 1, 1], [0x11, 2, 2, 2]], 2);
        let source_data = rig.e.mem.u32(source);
        rig.e.call(
            0x00db_a270,
            &args![array, 0x1234u32, 1u32, 1u32, source_data, 2u32],
        );
        assert_eq!(rig.first_words(array), vec![1, 0x10, 0x11, 3, 4]);
        let data = rig.e.mem.u32(array);
        assert_eq!(rig.e.mem.u32(data + 16 + 4), 1);
        assert_eq!(rig.e.mem.u32(data + 32 + 8), 2);
        assert!(rig.calls_tagged("reserve").is_empty());
        rig.e.mem.set_u32(array + 8, 5);
        rig.e.call(
            0x00db_a270,
            &args![array, 0x1234u32, 5u32, 0u32, source_data, 2u32],
        );
        assert_eq!(
            rig.first_words(array),
            vec![1, 0x10, 0x11, 3, 4, 0x10, 0x11]
        );
        assert_eq!(rig.calls_tagged("reserve"), vec![vec![array, 10, 16]]);
    }

    #[test]
    fn fn_00dba320_inserts_records_without_deleting() {
        let mut rig = Rig::new();
        let array = rig.record_array(&[[1, 0, 0, 0], [2, 0, 0, 0]], 4);
        let source = rig.record_array(&[[9, 0, 0, 0], [8, 0, 0, 0]], 2);
        let source_data = rig.e.mem.u32(source);
        rig.e.call(
            0x00db_a320,
            &args![array, 0x1234u32, 1u32, source_data, 2u32],
        );
        assert_eq!(rig.first_words(array), vec![1, 9, 8, 2]);
    }

    #[test]
    fn fn_00dba340_inserts_one_record() {
        let mut rig = Rig::new();
        let array = rig.record_array(&[[1, 0, 0, 0], [2, 0, 0, 0]], 4);
        let source = rig.record_array(&[[9, 0, 0, 0], [8, 0, 0, 0]], 2);
        let source_data = rig.e.mem.u32(source);
        rig.e
            .call(0x00db_a340, &args![array, 0x1234u32, 0u32, source_data]);
        assert_eq!(rig.first_words(array), vec![9, 1, 2]);
    }

    #[test]
    fn hk_serialize_multi_map_u64_insert_chains_the_entries() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        let key = 0x7_0000_0001u64;
        let (first, second) = (rig.record(&[7]), rig.record(&[9]));
        rig.e.call(0x00db_a360, &args![multimap, key, first]);
        rig.e.call(0x00db_a360, &args![multimap, key, second]);
        let data = rig.e.mem.u32(multimap);
        let entries: Vec<u32> = (0..4).map(|i| rig.e.mem.u32(data + 4 * i)).collect();
        assert_eq!(entries, vec![7, NONE, 9, 0]);
        assert_eq!(rig.map_value(multimap + 0xc, key), Some(1));
        assert_eq!(rig.e.mem.u32(multimap + 4), 2);
    }

    #[test]
    fn hk_serialize_multi_map_u64_destructor_releases_the_map_and_the_entries() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        let data = rig.e.mem.u32(multimap);
        rig.e.mem.set_u32(multimap + 4, 3);
        rig.set_map_value(multimap + 0xc, 5, 1);
        rig.e.call(0x00db_a3d0, &args![multimap]);
        assert_eq!(rig.tags(), vec!["map_release", "free"]);
        assert_eq!(rig.calls_tagged("free"), vec![vec![data, 128]]);
        assert_eq!(rig.map_value(multimap + 0xc, 5), None);
        assert_eq!(rig.e.mem.u32(multimap), 0);
        assert_eq!(rig.e.mem.u32(multimap + 4), 0);
        assert_eq!(rig.e.mem.u32(multimap + 8), 0x8000_0000);
    }

    #[test]
    fn hk_serialize_multi_map_char_ptr_destructor_releases_the_map_and_the_entries() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        let data = rig.e.mem.u32(multimap);
        rig.set_map_value(multimap + 0xc, 5, 1);
        rig.e.call(0x00db_a430, &args![multimap]);
        assert_eq!(rig.tags(), vec!["ptr_map_release", "free"]);
        assert_eq!(rig.calls_tagged("free"), vec![vec![data, 128]]);
        assert_eq!(rig.map_value(multimap + 0xc, 5), None);
        assert_eq!(rig.e.mem.u32(multimap + 8), 0x8000_0000);
    }

    #[test]
    fn fn_00dba490_releases_the_record_array() {
        let mut rig = Rig::new();
        let object = rig.e.mem.alloc(0x18);
        let data = rig.e.mem.alloc(64);
        rig.e.mem.set_u32(object + 0xc, data);
        rig.e.mem.set_u32(object + 0x10, 2);
        rig.e.mem.set_u32(object + 0x14, 4);
        rig.e.call(0x00db_a490, &args![object]);
        assert_eq!(rig.calls_tagged("free"), vec![vec![data, 64]]);
        assert_eq!(rig.e.mem.u32(object + 0xc), 0);
        assert_eq!(rig.e.mem.u32(object + 0x10), 0);
        assert_eq!(rig.e.mem.u32(object + 0x14), 0x8000_0000);
        rig.clear_calls();
        rig.e.mem.set_u32(object + 0x14, 0x8000_0004);
        rig.e.call(0x00db_a490, &args![object]);
        assert!(rig.calls_tagged("free").is_empty());
    }

    #[test]
    fn fn_00dba4d0_inserts_a_record_in_order() {
        let mut rig = Rig::new();
        let array = rig.record_array(&[], 2);
        for key in [5u32, 3, 9, 4] {
            let item = rig.record(&[key * 10, 0, key, 0]);
            rig.e.call(0x00db_a4d0, &args![array, item]);
        }
        assert_eq!(rig.first_words(array), vec![30, 40, 50, 90]);
        assert_eq!(rig.calls_tagged("reserve_more"), vec![vec![array, 16]]);
    }

    #[test]
    fn hk_serialize_multi_map_u64_get_free_index_reuses_freed_entries() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        let data = rig.e.mem.u32(multimap);
        rig.e.mem.set_u32(multimap + 0x18, 1);
        rig.e.mem.set_u32(data + 8 + 4, 5);
        let index = rig.e.call(0x00db_e710, &args![multimap]).i32();
        assert_eq!(index, 1);
        assert_eq!(rig.e.mem.i32(multimap + 0x18), 5);
        assert_eq!(rig.e.mem.u32(multimap + 4), 0);
        rig.e.mem.set_u32(multimap + 0x18, NONE);
        assert_eq!(rig.e.call(0x00db_e710, &args![multimap]).i32(), 0);
        assert_eq!(rig.e.mem.u32(multimap + 4), 1);
        assert!(rig.calls_tagged("reserve_more").is_empty());
        rig.e.mem.set_u32(multimap + 4, 16);
        assert_eq!(rig.e.call(0x00db_e710, &args![multimap]).i32(), 16);
        assert_eq!(rig.e.mem.u32(multimap + 4), 17);
        assert_eq!(rig.calls_tagged("reserve_more"), vec![vec![multimap, 8]]);
    }

    #[test]
    fn uid_from_class_version_cache_interns_into_the_owners_map() {
        let mut rig = Rig::new();
        let owner = rig.e.mem.alloc(0x60);
        let out = rig.record(&[0xdead]);
        let text = rig.name();
        rig.e.call(0x00dc_8d20, &args![owner, out, 0u32]);
        assert_eq!(rig.e.mem.u32(out), 0);
        assert!(rig.calls.borrow().is_empty());
        rig.e.call(0x00dc_8d20, &args![owner, out, text]);
        assert_eq!(rig.e.mem.u32(out), text);
        assert_eq!(
            rig.tags(),
            vec!["string_get", "string_dup", "string_insert"]
        );
        assert_eq!(
            rig.calls_tagged("string_get"),
            vec![vec![owner + 0x4c, text, 0]]
        );
        rig.clear_calls();
        rig.e.mem.set_u32(out, 0);
        rig.e.call(0x00dc_8d20, &args![owner, out, text]);
        assert_eq!(rig.e.mem.u32(out), text);
        assert_eq!(rig.tags(), vec!["string_get"]);
    }

    #[test]
    fn uid_from_class_version_cache_answers_its_out_pointer() {
        let mut rig = Rig::new();
        let owner = rig.e.mem.alloc(0x60);
        let out = rig.record(&[0xdead]);
        // EAX is `out` at both RETs; the caller reads the uid through it (00dc8dee).
        assert_eq!(rig.e.call(0x00dc_8d20, &args![owner, out, 0u32]).u32(), out);
    }

    #[test]
    fn hk_serialize_multi_map_char_ptr_insert_chains_the_entries() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        let (first, second) = (rig.record(&[7]), rig.record(&[9]));
        rig.e.call(0x00dc_9ec0, &args![multimap, 0x77u32, first]);
        rig.e.call(0x00dc_9ec0, &args![multimap, 0x77u32, second]);
        let data = rig.e.mem.u32(multimap);
        let entries: Vec<u32> = (0..4).map(|i| rig.e.mem.u32(data + 4 * i)).collect();
        assert_eq!(entries, vec![7, NONE, 9, 0]);
        assert_eq!(rig.map_value(multimap + 0xc, 0x77), Some(1));
        assert_eq!(rig.e.mem.u32(multimap + 4), 2);
    }

    type Table = Rc<RefCell<HashMap<u32, u32>>>;
    type Listing = Rc<RefCell<Vec<u32>>>;

    /// The class pointer the data world's `createClass` double hands out, and the scratch world's.
    const CREATED_CLASS: u32 = 0x00c1_a550;
    const SCRATCH_CLASS: u32 = 0x00c1_a5a0;
    /// The function a tag 4 entry runs on every object.
    const OBJECT_FUNCTION: u32 = 0x0900_0200;
    const GET_CLASS: u32 = 0x0900_0100;
    const CREATE_CLASS: u32 = 0x0900_0104;
    const LIST_CLASSES: u32 = 0x0900_0108;

    /// The doubles of the data world functions. `world` is the object the exe's world pointer
    /// stands for (slot `0x24` finds a class in `classes`, slot `0xc` creates one there, slot `0x20`
    /// lists `listing`); `scratch` holds the classes of the scratch world; `objects` and `all`
    /// are what the object and class listings return.
    struct Worlds {
        world: u32,
        classes: Table,
        scratch: Table,
        listing: Listing,
        objects: Listing,
        all: Listing,
    }

    fn fill_words(e: &mut Engine, array: u32, words: &[u32]) {
        if words.is_empty() {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
            return;
        }
        let data = e.mem.alloc(words.len().max(1) as u32 * 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *word);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, words.len() as u32);
    }

    fn install_worlds(rig: &mut Rig) -> Worlds {
        let classes: Table = Rc::default();
        let scratch: Table = Rc::default();
        let listing: Listing = Rc::default();
        let objects: Listing = Rc::default();
        let all: Listing = Rc::default();
        let world =
            rig.object_with(&[(0x24, GET_CLASS), (0xc, CREATE_CLASS), (0x20, LIST_CLASSES)]);
        let (c, t) = (Rc::clone(&rig.calls), Rc::clone(&classes));
        rig.e.register_double(GET_CLASS, move |_, a| {
            note(&c, "get_class", &a[1..]);
            Ret {
                eax: t.borrow().get(&a[1]).copied().unwrap_or(0),
                ..Ret::default()
            }
        });
        let (c, t) = (Rc::clone(&rig.calls), Rc::clone(&classes));
        rig.e.register_double(CREATE_CLASS, move |e, a| {
            let (name, version) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            note(
                &c,
                "create_class",
                &[name, version, e.mem.u32(a[1] + 8), e.mem.u32(a[1] + 0x10)],
            );
            t.borrow_mut().insert(name, CREATED_CLASS);
            Ret::default()
        });
        let l = Rc::clone(&listing);
        rig.e.register_double(LIST_CLASSES, move |e, a| {
            fill_words(e, a[1], &l.borrow());
            Ret::default()
        });
        let (c, t) = (Rc::clone(&rig.calls), Rc::clone(&scratch));
        rig.e.register_double(WORLD_FIND_CLASS, move |_, a| {
            note(&c, "find_class", &a[1..]);
            Ret {
                eax: t.borrow().get(&a[1]).copied().unwrap_or(0),
                ..Ret::default()
            }
        });
        let (c, t) = (Rc::clone(&rig.calls), Rc::clone(&scratch));
        rig.e.register_double(WORLD_CREATE_CLASS, move |e, a| {
            let (name, version) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            note(&c, "scratch_create", &[name, version]);
            t.borrow_mut().insert(name, SCRATCH_CLASS);
            Ret {
                eax: SCRATCH_CLASS,
                ..Ret::default()
            }
        });
        for (address, tag, source) in [
            (WORLD_FIND_OBJECTS_BY_BASE_CLASS, "find_base", &objects),
            (WORLD_FIND_OBJECTS_BY_EXACT_CLASS, "find_exact", &objects),
        ] {
            let (c, l) = (Rc::clone(&rig.calls), Rc::clone(source));
            rig.e.register_double(address, move |e, a| {
                note(&c, tag, &[a[1]]);
                fill_words(e, a[2], &l.borrow());
                Ret::default()
            });
        }
        let l = Rc::clone(&all);
        rig.e.register_double(WORLD_FIND_ALL_CLASSES, move |e, a| {
            fill_words(e, a[1], &l.borrow());
            Ret::default()
        });
        for (address, tag) in [
            (DATA_CLASS_CONSTRUCT, "class_construct"),
            (DATA_OBJECT_CONSTRUCT, "object_construct"),
        ] {
            let c = Rc::clone(&rig.calls);
            rig.e.register_double(address, move |e, a| {
                note(&c, tag, a);
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        rig.e.register_double(DATA_CLASS_IS_NULL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == 0),
            ..Ret::default()
        });
        let c = Rc::clone(&rig.calls);
        rig.e.register_double(OBJECT_FUNCTION, move |_, a| {
            note(&c, "object_function", a);
            Ret::default()
        });
        for (address, tag) in [
            (WORLD_RENAME_CLASS, "rename_class"),
            (WORLD_RENAME_MEMBER, "rename_member"),
            (WORLD_ADD_MEMBER, "add_member"),
            (WORLD_REMOVE_MEMBER, "remove_member"),
            (WORLD_SET_CLASS_PARENT, "set_parent"),
            (WORLD_REMOVE_CLASS, "remove_class"),
            (WORLD_SET_CLASS_VERSION, "set_version"),
            (WORLD_CAST_OBJECT, "cast_object"),
            (WORLD_WRAP_CLASS, "wrap_class"),
            (WORLD_COPY_CLASS_FROM_WORLD, "copy_class"),
            (WORLD_CONSTRUCT, "world_construct"),
            (WORLD_DESTRUCT, "world_destruct"),
            (DATA_CLASS_DESTRUCT, "class_destruct"),
            (DATA_OBJECT_DESTRUCT, "object_destruct"),
        ] {
            let c = Rc::clone(&rig.calls);
            rig.e.register_double(address, move |_, a| {
                note(&c, tag, a);
                Ret::default()
            });
        }
        Worlds {
            world,
            classes,
            scratch,
            listing,
            objects,
            all,
        }
    }

    #[test]
    fn collecting_a_class_reaches_what_it_depends_on() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let cache = rig.cache();
        let (a, b) = (rig.name(), rig.name());
        let dependency = rig.record(&[b, 2]);
        let pa = rig.patch(0, a, NONE, 1, &[(6, dependency)]);
        let pb = rig.patch(0, b, NONE, 2, &[]);
        let manager = rig.manager(cache, &[pa, pb]);
        let (ua, ub) = (rig.uid(cache, a, 1), rig.uid(cache, b, 2));
        let new_classes = rig.record(&[0, 0, 3]);
        rig.set_map_value(new_classes, ua, 0);
        rig.set_map_value(new_classes, ub, 1);
        let states = rig.record(&[0, 0, 3]);
        let collected = rig.record_array(&[], 4);
        rig.e.call(
            0x00db_94b0,
            &args![
                collected,
                a,
                1u32,
                worlds.world,
                manager,
                new_classes,
                states
            ],
        );
        let data = rig.e.mem.u32(collected);
        let record = |i: u32| -> Vec<u32> {
            (0..3)
                .map(|j| rig.e.mem.u32(data + 16 * i + 4 * j))
                .collect()
        };
        assert_eq!(rig.e.mem.u32(collected + 4), 2);
        assert_eq!(record(0), vec![ua as u32, (ua >> 32) as u32, 0]);
        assert_eq!(record(1), vec![ub as u32, (ub >> 32) as u32, 1]);
        assert_eq!(rig.calls_tagged("get_class"), vec![vec![a], vec![b]]);

        // A class the world has at that version gets its index from the manager's own map.
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let cache = rig.cache();
        let a = rig.name();
        let known = rig.class_object(a, 1);
        worlds.classes.borrow_mut().insert(a, known);
        let pa = rig.patch(0, a, NONE, 1, &[]);
        let pb = rig.patch(0, a, NONE, 2, &[]);
        let manager = rig.manager(cache, &[pa, pb]);
        let ua = rig.uid(cache, a, 1);
        rig.set_map_value(manager + 0x18, ua, 1);
        let new_classes = rig.record(&[0, 0, 3]);
        rig.set_map_value(new_classes, ua, 0);
        let states = rig.record(&[0, 0, 3]);
        let collected = rig.record_array(&[], 4);
        rig.e.call(
            0x00db_94b0,
            &args![
                collected,
                a,
                1u32,
                worlds.world,
                manager,
                new_classes,
                states
            ],
        );
        let data = rig.e.mem.u32(collected);
        assert_eq!(rig.e.mem.u32(collected + 4), 1);
        assert_eq!(rig.e.mem.u32(data + 8), 1);

        // A uid already in state 2 is not collected again; the same record is not added twice.
        rig.set_map_value(states, ua, 2);
        rig.e.call(
            0x00db_94b0,
            &args![
                collected,
                a,
                1u32,
                worlds.world,
                manager,
                new_classes,
                states
            ],
        );
        assert_eq!(rig.e.mem.u32(collected + 4), 1);
        rig.set_map_value(states, ua, 0);
        rig.e.call(
            0x00db_94b0,
            &args![
                collected,
                a,
                1u32,
                worlds.world,
                manager,
                new_classes,
                states
            ],
        );
        assert_eq!(rig.e.mem.u32(collected + 4), 1);
    }

    #[test]
    fn the_sort_step_places_each_node_after_what_it_leads_to() {
        let mut rig = Rig::new();
        let multimap = rig.multimap();
        rig.link(multimap, 0, 1);
        rig.link(multimap, 1, 2);
        let state = rig.word_array(&[NONE, NONE, NONE], 3);
        let stack = rig.word_array(&[], 4);
        let carried = rig.record(&[0, 0, 0]);
        let got = rig
            .e
            .call(
                0x00db_83c0,
                &args![0u32, state, multimap, 0u32, stack, carried],
            )
            .i32();
        assert_eq!(got, 3);
        assert_eq!(rig.words_of(state), vec![2, 1, 0]);
        assert_eq!(rig.e.mem.u32(stack + 4), 0);
        let again = rig
            .e
            .call(
                0x00db_83c0,
                &args![1u32, state, multimap, 7u32, stack, carried],
            )
            .i32();
        assert_eq!(again, 7);
    }

    #[test]
    fn the_sort_step_notes_a_cycle_with_a_string_buffer() {
        for owns_block in [false, true] {
            let mut rig = Rig::new();
            let multimap = rig.multimap();
            rig.link(multimap, 0, 1);
            rig.link(multimap, 1, 0);
            let calls = Rc::clone(&rig.calls);
            rig.e.register_double(STRING_BUFFER_CONSTRUCT, move |e, a| {
                note(&calls, "string_buffer", a);
                if owns_block {
                    e.mem.set_u32(a[0], 0x0abc);
                    e.mem.set_u32(a[0] + 8, 0x40);
                } else {
                    e.mem.set_u32(a[0], a[0] + 0xc);
                    e.mem.set_u32(a[0] + 8, 0x8000_0080);
                }
                Ret::default()
            });
            let state = rig.word_array(&[NONE, NONE], 2);
            let stack = rig.word_array(&[], 4);
            let carried = rig.record(&[0, 0, 0]);
            let got = rig
                .e
                .call(
                    0x00db_83c0,
                    &args![0u32, state, multimap, 0u32, stack, carried],
                )
                .i32();
            assert_eq!(got, 2);
            assert_eq!(rig.words_of(state), vec![1, 0]);
            assert_eq!(rig.e.mem.u32(stack + 4), 0);
            assert_eq!(rig.calls_tagged("string_buffer").len(), 1);
            let freed = rig.calls_tagged("other_free");
            if owns_block {
                assert_eq!(freed, vec![vec![0x0abc, 0x40]]);
            } else {
                assert!(freed.is_empty());
            }
        }
    }

    #[test]
    fn apply_patches_finally_creates_a_class_and_applies_its_entries() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let (n, parent, x, y) = (rig.name(), rig.name(), rig.name(), rig.name());
        worlds.classes.borrow_mut().insert(parent, 0x2222);
        let add = rig.record(&[x, y, 3, 4]);
        let rename = rig.record(&[x, y]);
        let set_parent = rig.record(&[x, parent]);
        let patch = rig.patch(
            0,
            n,
            NONE,
            3,
            &[(1, add), (3, rename), (7, set_parent), (6, add)],
        );
        let patches = rig.word_array(&[patch], 1);
        let got = rig.e.call(0x00db_8000, &args![worlds.world, patches]).u32();
        assert_eq!(got, 0);
        assert_eq!(
            rig.tags(),
            vec![
                "get_class",
                "create_class",
                "get_class",
                "class_construct",
                "add_member",
                "rename_member",
                "get_class",
                "class_construct",
                "set_parent",
                "class_destruct",
                "class_destruct"
            ]
        );
        assert_eq!(rig.calls_tagged("create_class"), vec![vec![n, 3, 0, 0]]);
        let class = rig.calls_tagged("class_construct")[0][0];
        assert_eq!(rig.calls_tagged("class_construct")[0][1], CREATED_CLASS);
        let world = worlds.world;
        assert_eq!(
            rig.calls_tagged("add_member"),
            vec![vec![world, class, x, y, 3, 4]]
        );
        assert_eq!(
            rig.calls_tagged("rename_member"),
            vec![vec![world, class, x, y]]
        );
        let parent_class = rig.calls_tagged("class_construct")[1][0];
        assert_eq!(rig.calls_tagged("class_construct")[1][1], 0x2222);
        assert_eq!(
            rig.calls_tagged("set_parent"),
            vec![vec![world, class, parent_class]]
        );
    }

    #[test]
    fn apply_patches_finally_runs_the_entries_on_an_existing_class() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let (old, new, member, target) = (rig.name(), rig.name(), rig.name(), rig.name());
        worlds.classes.borrow_mut().insert(old, 0x1111);
        worlds.classes.borrow_mut().insert(target, 0x3333);
        *worlds.objects.borrow_mut() = vec![0x71, 0x72];
        let remove = rig.record(&[member]);
        let run = rig.record(&[0, OBJECT_FUNCTION]);
        let cast = rig.record(&[target]);
        let patch = rig.patch(old, new, 1, 2, &[(2, remove), (4, run), (5, cast)]);
        let patches = rig.word_array(&[patch], 1);
        let got = rig.e.call(0x00db_8000, &args![worlds.world, patches]).u32();
        assert_eq!(got, 0);
        assert_eq!(
            rig.tags(),
            vec![
                "get_class",
                "class_construct",
                "rename_class",
                "remove_member",
                "find_base",
                "object_construct",
                "object_function",
                "object_destruct",
                "object_construct",
                "object_function",
                "object_destruct",
                "free",
                "get_class",
                "class_construct",
                "find_exact",
                "object_construct",
                "cast_object",
                "object_destruct",
                "object_construct",
                "cast_object",
                "object_destruct",
                "free",
                "class_destruct",
                "set_version",
                "class_destruct"
            ]
        );
        let world = worlds.world;
        let class = rig.calls_tagged("class_construct")[0][0];
        assert_eq!(
            rig.calls_tagged("rename_class"),
            vec![vec![world, class, new]]
        );
        assert_eq!(
            rig.calls_tagged("remove_member"),
            vec![vec![world, class, member]]
        );
        assert_eq!(rig.calls_tagged("find_base"), vec![vec![old]]);
        assert_eq!(rig.calls_tagged("set_version"), vec![vec![world, class, 2]]);
        assert_eq!(
            rig.calls_tagged("object_construct")
                .iter()
                .map(|a| a[1])
                .collect::<Vec<_>>(),
            vec![0x71, 0x72, 0x71, 0x72]
        );
    }

    #[test]
    fn apply_patches_finally_stops_on_an_existing_class_and_removes_classes() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let (n, old) = (rig.name(), rig.name());
        worlds.classes.borrow_mut().insert(n, 0x1111);
        let created = rig.patch(0, n, NONE, 1, &[]);
        let patches = rig.word_array(&[created], 1);
        let got = rig.e.call(0x00db_8000, &args![worlds.world, patches]).u32();
        assert_eq!(got, 1);
        assert_eq!(rig.tags(), vec!["get_class"]);
        rig.clear_calls();
        worlds.classes.borrow_mut().insert(old, 0x2222);
        let removed = rig.patch(old, 0, 1, NONE, &[]);
        let patches = rig.word_array(&[removed], 1);
        let got = rig.e.call(0x00db_8000, &args![worlds.world, patches]).u32();
        assert_eq!(got, 0);
        assert_eq!(
            rig.tags(),
            vec![
                "get_class",
                "class_construct",
                "remove_class",
                "class_destruct"
            ]
        );
    }

    #[test]
    fn recompute_orders_the_patches_after_their_dependencies() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let (a, b) = (rig.name(), rig.name());
        let dependency = rig.record(&[b, 2]);
        let p0 = rig.patch(a, a, 1, 2, &[(6, dependency)]);
        let p1 = rig.patch(b, b, 1, 2, &[]);
        let manager = rig.manager(cache, &[p0, p1]);
        let got = rig.e.call(0x00db_8670, &args![manager]).u32();
        assert_eq!(got, 0);
        assert_eq!(rig.words_of(manager + 0xc), vec![p1, p0]);
        let (ub, ua) = (rig.uid(cache, b, 1), rig.uid(cache, a, 1));
        assert_eq!(rig.map_value(manager + 0x18, ub), Some(0));
        assert_eq!(rig.map_value(manager + 0x18, ua), Some(1));
        assert_eq!(rig.calls_tagged("string_map_construct").len(), 1);
        assert_eq!(rig.calls_tagged("string_map_destruct").len(), 1);
    }

    #[test]
    fn recompute_reports_invalid_and_repeated_patches() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let bad = rig.patch(0, 0, NONE, NONE, &[]);
        let manager = rig.manager(cache, &[bad]);
        assert_eq!(rig.e.call(0x00db_8670, &args![manager]).u32(), 1);
        assert_eq!(rig.calls_tagged("map_release").len(), 2);
        assert_eq!(rig.calls_tagged("ptr_map_release").len(), 1);

        let mut rig = Rig::new();
        let cache = rig.cache();
        let a = rig.name();
        let first = rig.patch(0, a, NONE, 1, &[]);
        let second = rig.patch(0, a, NONE, 1, &[]);
        let manager = rig.manager(cache, &[first, second]);
        assert_eq!(rig.e.call(0x00db_8670, &args![manager]).u32(), 1);
        assert_eq!(rig.calls_tagged("map_release").len(), 2);
        assert_eq!(rig.calls_tagged("ptr_map_release").len(), 1);
        assert!(rig.calls_tagged("string_map_construct").is_empty());

        let mut rig = Rig::new();
        let cache = rig.cache();
        let a = rig.name();
        let first = rig.patch(a, a, 1, 2, &[]);
        let second = rig.patch(a, a, 1, 3, &[]);
        let manager = rig.manager(cache, &[first, second]);
        assert_eq!(rig.e.call(0x00db_8670, &args![manager]).u32(), 1);
        assert_eq!(rig.calls_tagged("map_release").len(), 2);
    }

    #[test]
    fn recompute_relates_a_creating_patch_to_the_patches_that_retire_its_name() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let a = rig.name();
        // A class created at version 2 after one retired at version 1 under the same name.
        let retire = rig.patch(a, 0, 1, NONE, &[]);
        let create = rig.patch(0, a, NONE, 2, &[]);
        let manager = rig.manager(cache, &[create, retire]);
        let got = rig.e.call(0x00db_8670, &args![manager]).u32();
        assert_eq!(got, 0);
        assert_eq!(rig.words_of(manager + 0xc), vec![retire, create]);
        let retired = rig.uid(cache, a, 1);
        assert_eq!(rig.map_value(manager + 0x18, retired), Some(0));
        let named: Vec<Vec<u32>> = rig
            .calls_tagged("string_insert")
            .into_iter()
            .filter(|call| call[0] != cache + 0x1c)
            .collect();
        assert_eq!(named.len(), 1);
        assert_eq!(named[0][1..], [a, 0]);
    }

    /// A manager with one patch of the class `name` (version 1 to 2), a data world that has the
    /// class at version 2, a scratch world that lacks it, and a registry whose class is
    /// `registry_class`.
    struct Scenario {
        worlds: Worlds,
        manager: u32,
        registry: u32,
        out: u32,
        name: u32,
    }

    const REGISTRY_FIND: u32 = 0x0900_0400;

    fn class_scenario(rig: &mut Rig, registry_class: u32) -> Scenario {
        let worlds = install_worlds(rig);
        let cache = rig.cache();
        let name = rig.name();
        let class = rig.class_object(name, 2);
        *worlds.listing.borrow_mut() = vec![class];
        worlds.classes.borrow_mut().insert(name, class);
        let member = rig.record(&[name, name, 0, 0]);
        let patch = rig.patch(name, name, 1, 2, &[(1, member)]);
        let manager = rig.manager(cache, &[patch]);
        let uid = rig.uid(cache, name, 1);
        rig.set_map_value(manager + 0x18, uid, 0);
        let calls = Rc::clone(&rig.calls);
        rig.e.register_double(REGISTRY_FIND, move |_, a| {
            note(&calls, "registry_find", a);
            Ret {
                eax: registry_class,
                ..Ret::default()
            }
        });
        let registry = rig.object_with(&[(0x10, REGISTRY_FIND)]);
        rig.e.register_double(CLASS_GET_NAME, move |_, _| Ret {
            eax: name,
            ..Ret::default()
        });
        let out = rig.word_array(&[], 4);
        Scenario {
            worlds,
            manager,
            registry,
            out,
            name,
        }
    }

    #[test]
    fn prepare_patches_with_nothing_to_do_leaves_the_output_empty() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        rig.install_iteration(&[]);
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let registry = rig.e.mem.alloc(8);
        let out = rig.word_array(&[], 4);
        let got = rig
            .e
            .call(0x00db_95c0, &args![manager, worlds.world, registry, out])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(rig.e.mem.u32(out + 4), 0);
        assert_eq!(
            rig.tags(),
            vec![
                "world_construct",
                "string_map_construct",
                "string_map_construct",
                "string_map_destruct",
                "string_map_destruct",
                "map_release",
                "map_release",
                "world_destruct"
            ]
        );
    }

    #[test]
    fn prepare_patches_wraps_a_class_of_the_right_version_and_copies_new_classes() {
        let mut rig = Rig::new();
        let registry_class = rig.e.mem.alloc(8);
        let scenario = class_scenario(&mut rig, registry_class);
        let other = rig.name();
        let other_class = rig.class_object(other, 1);
        *scenario.worlds.all.borrow_mut() = vec![other_class];
        rig.install_iteration(&[other_class]);
        let calls = Rc::clone(&rig.calls);
        rig.e
            .register_double(CLASS_GET_DESCRIBED_VERSION, move |_, a| {
                note(&calls, "described", a);
                Ret {
                    eax: 2,
                    ..Ret::default()
                }
            });
        let got = rig
            .e
            .call(
                0x00db_95c0,
                &args![
                    scenario.manager,
                    scenario.worlds.world,
                    scenario.registry,
                    scenario.out
                ],
            )
            .u32();
        assert_eq!(got, 0);
        assert_eq!(rig.e.mem.u32(scenario.out + 4), 0);
        let scratch = rig.calls_tagged("world_construct")[0][0];
        assert_eq!(
            rig.calls_tagged("registry_find"),
            vec![vec![scenario.registry, scenario.name]]
        );
        assert_eq!(
            rig.calls_tagged("wrap_class"),
            vec![vec![scratch, registry_class]]
        );
        assert_eq!(
            rig.calls_tagged("copy_class"),
            vec![vec![scenario.worlds.world, other, scratch]]
        );
        assert_eq!(rig.calls_tagged("described").len(), 1);
    }

    #[test]
    fn prepare_patches_reports_a_class_of_the_wrong_version() {
        let mut rig = Rig::new();
        let registry_class = rig.e.mem.alloc(8);
        let scenario = class_scenario(&mut rig, registry_class);
        rig.install_iteration(&[]);
        let mut asked = 0;
        rig.e
            .register_double(CLASS_GET_DESCRIBED_VERSION, move |_, _| {
                asked += 1;
                Ret {
                    eax: 4 + asked,
                    ..Ret::default()
                }
            });
        for (address, tag) in [
            (OSTREAM_CONSTRUCT, "stream_construct"),
            (OSTREAM_DESTRUCT, "stream_destruct"),
        ] {
            let calls = Rc::clone(&rig.calls);
            rig.e.register_double(address, move |_, a| {
                note(&calls, tag, a);
                Ret::default()
            });
        }
        for (address, tag) in [
            (OSTREAM_APPEND_TEXT, "append_text"),
            (OSTREAM_APPEND_INT, "append_int"),
        ] {
            let calls = Rc::clone(&rig.calls);
            rig.e.register_double(address, move |_, a| {
                note(&calls, tag, a);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let handler = rig.object_with(&[(0xc, 0x0900_0500)]);
        rig.e.mem.set_u32(0x0126_8360, handler);
        let calls = Rc::clone(&rig.calls);
        rig.e.register_double(0x0900_0500, move |_, a| {
            note(&calls, "report", a);
            Ret::default()
        });
        let got = rig
            .e
            .call(
                0x00db_95c0,
                &args![
                    scenario.manager,
                    scenario.worlds.world,
                    scenario.registry,
                    scenario.out
                ],
            )
            .u32();
        assert_eq!(got, 1);
        let name = scenario.name;
        let appended: Vec<(&str, u32)> = rig
            .calls
            .borrow()
            .iter()
            .filter(|(tag, _)| tag.starts_with("append_"))
            .map(|(tag, a)| (*tag, a[1]))
            .collect();
        assert_eq!(
            appended,
            vec![
                ("append_text", 0x010d_6168),
                ("append_text", name),
                ("append_text", 0x010d_6178),
                ("append_int", 7),
                ("append_text", 0x010d_6188),
                ("append_int", 2),
                ("append_text", 0x010d_61a0),
                ("append_text", name),
                ("append_text", 0x010d_61e4),
                ("append_int", 2),
                ("append_text", 0x010d_61f4),
                ("append_int", 6),
                ("append_text", 0x010d_61fc),
                ("append_int", 2),
                ("append_text", 0x010d_6234)
            ]
        );
        let stream = rig.calls_tagged("stream_construct")[0].clone();
        assert_eq!(stream[2], 0x200);
        assert_eq!(
            rig.calls_tagged("report"),
            vec![vec![handler, 1, 0x3f79_ddb1, stream[1], 0x010d_6144, 0x2fe]]
        );
        assert_eq!(rig.calls_tagged("stream_destruct"), vec![vec![stream[0]]]);
        assert!(rig.calls_tagged("wrap_class").is_empty());
        assert_eq!(rig.calls_tagged("world_destruct").len(), 1);
    }

    #[test]
    fn prepare_patches_stops_on_a_class_the_registry_lacks_and_on_bad_patches() {
        let mut rig = Rig::new();
        let scenario = class_scenario(&mut rig, 0);
        rig.install_iteration(&[]);
        let got = rig
            .e
            .call(
                0x00db_95c0,
                &args![
                    scenario.manager,
                    scenario.worlds.world,
                    scenario.registry,
                    scenario.out
                ],
            )
            .u32();
        assert_eq!(got, 1);
        assert!(rig.calls_tagged("wrap_class").is_empty());
        assert_eq!(rig.calls_tagged("world_destruct").len(), 1);

        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let cache = rig.cache();
        let bad = rig.patch(0, 0, NONE, NONE, &[]);
        let manager = rig.manager(cache, &[bad]);
        let out = rig.word_array(&[], 4);
        let got = rig
            .e
            .call(0x00db_95c0, &args![manager, worlds.world, 0u32, out])
            .u32();
        assert_eq!(got, 1);
        assert_eq!(
            rig.tags(),
            vec![
                "world_construct",
                "map_release",
                "map_release",
                "world_destruct"
            ]
        );

        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        let cache = rig.cache();
        let a = rig.name();
        let first = rig.patch(0, a, NONE, 1, &[]);
        let second = rig.patch(0, a, NONE, 1, &[]);
        let manager = rig.manager(cache, &[first, second]);
        let out = rig.word_array(&[], 4);
        let got = rig
            .e
            .call(0x00db_95c0, &args![manager, worlds.world, 0u32, out])
            .u32();
        assert_eq!(got, 1);
        assert_eq!(rig.calls_tagged("world_destruct").len(), 1);
    }

    #[test]
    fn prepare_patches_undoes_the_patches_it_collected_on_the_scratch_world() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        rig.install_iteration(&[]);
        let cache = rig.cache();
        let name = rig.name();
        let class = rig.class_object(name, 1);
        *worlds.listing.borrow_mut() = vec![class];
        let member = rig.record(&[name, name, 7, 8]);
        let patch = rig.patch(0, name, NONE, 1, &[(2, member)]);
        let manager = rig.manager(cache, &[patch]);
        let registry_class = rig.e.mem.alloc(8);
        let calls = Rc::clone(&rig.calls);
        rig.e.register_double(REGISTRY_FIND, move |_, a| {
            note(&calls, "registry_find", a);
            Ret {
                eax: registry_class,
                ..Ret::default()
            }
        });
        let registry = rig.object_with(&[(0x10, REGISTRY_FIND)]);
        rig.e
            .register_double(CLASS_GET_DESCRIBED_VERSION, |_, _| Ret {
                eax: 1,
                ..Ret::default()
            });
        rig.e.register_double(CLASS_GET_NAME, move |_, _| Ret {
            eax: name,
            ..Ret::default()
        });
        let out = rig.word_array(&[], 4);
        let got = rig
            .e
            .call(0x00db_95c0, &args![manager, worlds.world, registry, out])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(rig.words_of(out), vec![patch]);
        let scratch = rig.calls_tagged("world_construct")[0][0];
        assert_eq!(rig.calls_tagged("scratch_create"), vec![vec![name, NONE]]);
        assert_eq!(worlds.scratch.borrow().get(&name), Some(&SCRATCH_CLASS));
        let class_ptr = rig.calls_tagged("class_construct")[0][0];
        assert_eq!(
            rig.calls_tagged("add_member"),
            vec![vec![scratch, class_ptr, name, name, 7, 8]]
        );
        assert_eq!(
            rig.calls_tagged("remove_class"),
            vec![vec![scratch, class_ptr]]
        );
        assert_eq!(rig.calls_tagged("registry_find").len(), 1);
    }

    #[test]
    fn apply_patches_prepares_and_applies_then_releases_the_registry() {
        let mut rig = Rig::new();
        let worlds = install_worlds(&mut rig);
        rig.install_iteration(&[]);
        for (address, tag) in [(ADD_REFERENCE, "add_ref"), (REMOVE_REFERENCE, "remove_ref")] {
            let calls = Rc::clone(&rig.calls);
            rig.e.register_double(address, move |_, a| {
                note(&calls, tag, a);
                Ret::default()
            });
        }
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let registry = rig.e.mem.alloc(8);
        let got = rig
            .e
            .call(0x00db_a0d0, &args![manager, worlds.world, registry])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(rig.calls_tagged("add_ref"), vec![vec![registry]]);
        assert_eq!(rig.calls_tagged("remove_ref"), vec![vec![registry]]);

        // A null registry is fetched from the object at 01268b08.
        rig.clear_calls();
        let found = rig.e.mem.alloc(8);
        rig.e.register_double(0x0900_0600, move |_, _| Ret {
            eax: found,
            ..Ret::default()
        });
        let global = rig.object_with(&[(0x10, 0x0900_0600)]);
        rig.e.mem.set_u32(0x0126_8b08, global);
        let got = rig
            .e
            .call(0x00db_a0d0, &args![manager, worlds.world, 0u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(rig.calls_tagged("add_ref"), vec![vec![found]]);
        assert_eq!(rig.calls_tagged("remove_ref"), vec![vec![found]]);

        // Recomputing the dependencies of an invalid patch answers 1 and keeps the reference.
        rig.clear_calls();
        let bad = rig.patch(0, 0, NONE, NONE, &[]);
        let broken = rig.manager(cache, &[bad]);
        let got = rig
            .e
            .call(0x00db_a0d0, &args![broken, worlds.world, registry])
            .u32();
        assert_eq!(got, 1);
        assert_eq!(rig.calls_tagged("add_ref"), vec![vec![registry]]);
        assert!(rig.calls_tagged("remove_ref").is_empty());
    }

    #[test]
    fn apply_patches_debug_applies_the_managers_own_patches() {
        let mut rig = Rig::new();
        let cache = rig.cache();
        let manager = rig.manager(cache, &[]);
        let world = rig.e.mem.alloc(8);
        let got = rig.e.call(0x00db_a1a0, &args![manager, world]).u32();
        assert_eq!(got, 0);
        assert!(rig.calls.borrow().is_empty());

        let bad = rig.patch(0, 0, NONE, NONE, &[]);
        let broken = rig.manager(cache, &[bad]);
        let got = rig.e.call(0x00db_a1a0, &args![broken, world]).u32();
        assert_eq!(got, 1);
        assert_eq!(rig.calls_tagged("map_release").len(), 2);
    }

    #[test]
    fn the_patch_manager_creator_and_constructor_build_the_maps() {
        let mut e = Engine::new();
        e.map(0x0000_7000, 0x1000);
        e.mem.set_u32(0x7010, 0x7100);
        e.mem.set_u32(0x7100, 0x7200);
        e.mem.set_u32(0x7204, 0x00ab_0004);
        e.register(0x00c8_5750, |_, _| 0x7000u32.into_ret());
        // The first block is the manager (0x24 bytes), the second its 0x2C-byte part.
        e.register_double(0x00ab_0004, |_, a| {
            if a[1] == 0x24 {
                0x7800u32.into_ret()
            } else {
                assert_eq!(a[1], 0x2c);
                0x7900u32.into_ret()
            }
        });
        e.register(0x00cb_2030, |_, _| Ret::default());
        e.register(0x00d8_7790, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00db_a1e0, &args![]).u32(), 0x7800);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u16(0x7804), 0x24);
        assert_eq!(e.mem.u16(0x7806), 1);
        assert_eq!(e.mem.u32(0x7800), 0x010d_6138);
        assert_eq!(e.mem.u32(0x7814), 0x8000_0000);
        assert_eq!(e.mem.u32(0x7820), 0xffff_ffff);
        assert_eq!(e.mem.u32(0x7808), 0x7900);
        assert_eq!(e.mem.u32(0x7918), 0x8000_0000);
        let reserve: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00cb_2030).collect();
        assert_eq!(reserve[0].1, [0x7818, 0x7100, 0]);
        let maps: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x00d8_7790)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(maps, [0x7900, 0x791c]);
    }
}
