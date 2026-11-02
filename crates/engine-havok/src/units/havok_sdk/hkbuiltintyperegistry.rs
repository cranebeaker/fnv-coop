//! `Havok SDK/hkbuiltintyperegistry.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The three global registry pointers: the type info registry (`01269b18`), the class name
/// registry (`01269b14`) and the vtable class registry (`01269b10`).
const TYPE_INFO_REGISTRY: u32 = 0x0126_9b18;
const CLASS_NAME_REGISTRY: u32 = 0x0126_9b14;
const VTABLE_CLASS_REGISTRY: u32 = 0x0126_9b10;
/// `hkClass`-like functions (`this` = the class): name (`00d46d40`) and `getNumInterfaces`
/// (`00d46de0`, summed over the parents).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const CLASS_GET_NUM_INTERFACES: u32 = 0x00d4_6de0;
/// The string map functions (`this` = the string map): constructor (`00d87790`, one dword it
/// ignores), get with default (`00d87800`, `(key, default)`), insert (`00d874d0`, `(key,
/// value)`), destructor (`00d877d0`), and the iterator functions `getIterator` (`00d87410`),
/// `getKey` (`00d87430`), `getValue` (`00d87440`), `getNext` (`00d87480`) (each `(iterator)`
/// but the first) and `isValid` (`00d874b0`, `(&out byte, iterator)`).
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
const STRING_MAP_GET_ITERATOR: u32 = 0x00d8_7410;
const STRING_MAP_GET_KEY: u32 = 0x00d8_7430;
const STRING_MAP_GET_VALUE: u32 = 0x00d8_7440;
const STRING_MAP_GET_NEXT: u32 = 0x00d8_7480;
const STRING_MAP_IS_VALID: u32 = 0x00d8_74b0;
/// The pointer map functions (`this` = the map): get with default (`00cb1930`, `(key,
/// default)`), insert (`00cb1870`, `(allocator, key, value)`) and reserve (`00cb1fe0`,
/// `(allocator, capacity)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkArray` growth by one element (`00c90940`, cdecl `(allocator, array, element size)`).
const RESERVE_MORE: u32 = 0x00c9_0940;
/// `removeReferenceLockUnchecked` (`00c903b0`, `ECX` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_03b0;
/// `00db78a0` (`ECX` = the vtable class registry, `(list, list)`), which `init` calls with the
/// two exe lists; it registers the vtables of the listed type infos (unconfirmed).
const REGISTER_LISTED_VTABLES: u32 = 0x00db_78a0;
/// The exe list of type info pointers, ended by 0 (`010d5430`), and the second list `init`
/// hands to `00db78a0` (`010d59b8`).
const TYPE_INFO_LIST: u32 = 0x010d_5430;
const VTABLE_LIST: u32 = 0x010d_59b8;
/// The exe data object at `01268b0c` that `init` hands to slot `0x24` of the class name
/// registry (identity unconfirmed).
const INIT_CLASS_OBJECT: u32 = 0x0126_8b0c;
/// The vtables stored by the constructor and by `reinitialize` (the class name registry gets
/// `010cf7d4` for a moment and `010cf804` after its map is built), and the base vtable.
const VTABLE_DEFAULT_REGISTRY: u32 = 0x010c_f78c;
const VTABLE_CLASS_NAMES_FIRST: u32 = 0x010c_f7d4;
const VTABLE_CLASS_NAMES: u32 = 0x010c_f804;
const VTABLE_CLASS_NAMES_AUX: u32 = 0x010c_f830;
const VTABLE_TYPE_INFOS: u32 = 0x010c_f7b0;
const VTABLE_VTABLE_CLASSES: u32 = 0x010c_f844;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The capacity bits of an array's capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Whether the string map iterator is valid: `isValid` stores a byte through a pointer.
fn iterator_is_valid(e: &mut Engine, map: u32, iterator: u32) -> bool {
    e.with_stack(4, |e, flag| {
        e.call(STRING_MAP_IS_VALID, &args![map, flag.addr(), iterator]);
        e.mem.u8(flag.addr()) != 0
    })
}

/// Appends one word to the `hkArray` at `array` the way the exe does: when the size equals the
/// capacity (`& 0x3fffffff`) `00c90940` grows it first (the router is asked once, before the
/// test).
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_MASK {
        let allocator = e.mem.u32(router + 0x10);
        e.call(RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Replaces the registry pointer in the global `global` with `registry`, first releasing the
/// old one (`00c903b0`) when there is one.
fn replace_registry(e: &mut Engine, global: u32, registry: u32) {
    let old = e.mem.u32(global);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(global, registry);
}

// Translated from 00d477c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBuiltinTypeRegistry::addType` (Xbox PDB; two stack words, no `this`): registers `class`
/// in the class name registry (global `01269b14`, slot `0x1c` with its name from `00d46d40`),
/// `type_info` in the type info registry (global `01269b18`, slot `0xc`) and, when the class's
/// `getNumInterfaces` (`00d46de0`) is above 0 (signed), `(type_info word +0xc, class)` in the
/// vtable class registry (global `01269b10`, slot `0xc`).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hk_builtin_type_registry_add_type(e: &mut Engine, _this: u32, type_info: u32, class: u32) {
    let class_names = e.mem.u32(CLASS_NAME_REGISTRY);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    e.vcall(class_names, 0x1c, &args![class, name]);
    let type_infos = e.mem.u32(TYPE_INFO_REGISTRY);
    e.vcall(type_infos, 0xc, &args![type_info]);
    let interfaces = e.call(CLASS_GET_NUM_INTERFACES, &args![class]).i32();
    if interfaces > 0 {
        let vtable_classes = e.mem.u32(VTABLE_CLASS_REGISTRY);
        let vtable = e.mem.u32(type_info + 0xc);
        e.vcall(vtable_classes, 0xc, &args![vtable, class]);
    }
}

// Translated from 00d47860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultBuiltinTypeRegistry::init` (Xbox PDB): hands the exe object at `01268b0c` to slot
/// `0x24` of the class name registry (global `01269b14`), registers each type info of the
/// 0-ended pointer list at `010d5430` in the type info registry (global `01269b18`, slot
/// `0xc`) and calls `00db78a0` on the vtable class registry (global `01269b10`) with the list
/// `010d5430` and the list `010d59b8`.
pub fn hk_default_builtin_type_registry_init(e: &mut Engine) {
    let class_names = e.mem.u32(CLASS_NAME_REGISTRY);
    e.vcall(class_names, 0x24, &args![INIT_CLASS_OBJECT]);
    let type_infos = e.mem.u32(TYPE_INFO_REGISTRY);
    let mut cursor = TYPE_INFO_LIST;
    loop {
        let type_info = e.mem.u32(cursor);
        if type_info == 0 {
            break;
        }
        e.vcall(type_infos, 0xc, &args![type_info]);
        cursor += 4;
    }
    let vtable_classes = e.mem.u32(VTABLE_CLASS_REGISTRY);
    e.call(
        REGISTER_LISTED_VTABLES,
        &args![vtable_classes, TYPE_INFO_LIST, VTABLE_LIST],
    );
}

// Translated from 00d478c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the default builtin type registry (unnamed in the exe; `ECX` = `this`):
/// stores the 16-bit word 1 at `+6` and the vtable `010cf78c`, runs
/// [`hk_default_builtin_type_registry_init`] and returns `this`.
pub fn construct_00d478c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), VTABLE_DEFAULT_REGISTRY);
    hk_default_builtin_type_registry_init(e);
    this
}

// Translated from 00d478e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::registerTypeInfo` (Xbox PDB): inserts `type_info` into the string map
/// at `+8` under the first word of the type info.
pub fn hk_type_info_registry_register_type_info(e: &mut Engine, this: Ptr, type_info: u32) {
    let name = e.mem.u32(type_info);
    e.call(STRING_MAP_INSERT, &args![this.addr() + 8, name, type_info]);
}

// Translated from 00d47900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::finishLoadedObject` (Xbox PDB): looks `name` up in the string map at
/// `+8` (default 0); without an entry the answer is 0. Otherwise a non-null function pointer
/// at entry `+4` is called (cdecl) with `(object, the word at this + 0x18)`. Answers the entry.
pub fn hk_type_info_registry_finish_loaded_object(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    name: u32,
) -> u32 {
    let t = this.addr();
    let entry = e
        .call(STRING_MAP_GET_WITH_DEFAULT, &args![t + 8, name, 0u32])
        .u32();
    if entry == 0 {
        return 0;
    }
    let finish = e.mem.u32(entry + 4);
    if finish != 0 {
        let extra = e.mem.u32(t + 0x18);
        e.call(finish, &args![object, extra]);
    }
    entry
}

// Translated from 00d47940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::cleanupLoadedObject` (Xbox PDB): looks `name` up in the string map at
/// `+8` (default 0); without an entry the answer is 0. Otherwise a non-null function pointer
/// at entry `+8` is called (cdecl) with `(object)`. Answers the entry.
pub fn hk_type_info_registry_cleanup_loaded_object(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    name: u32,
) -> u32 {
    let entry = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![this.addr() + 8, name, 0u32],
        )
        .u32();
    if entry == 0 {
        return 0;
    }
    let cleanup = e.mem.u32(entry + 8);
    if cleanup != 0 {
        e.call(cleanup, &args![object]);
    }
    entry
}

// Translated from 00d47970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::merge` (Xbox PDB): walks the string map at `other + 8` with its
/// iterator and inserts every key and value into the string map at `this + 8` (the value is
/// fetched first, then the key, as in the exe).
pub fn hk_type_info_registry_merge(e: &mut Engine, this: Ptr, other: u32) {
    let source = other + 8;
    let target = this.addr() + 8;
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![source]).u32();
    if !iterator_is_valid(e, source, iterator) {
        return;
    }
    loop {
        let value = e.call(STRING_MAP_GET_VALUE, &args![source, iterator]).u32();
        let key = e.call(STRING_MAP_GET_KEY, &args![source, iterator]).u32();
        e.call(STRING_MAP_INSERT, &args![target, key, value]);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![source, iterator]).u32();
        if !iterator_is_valid(e, source, iterator) {
            break;
        }
    }
}

// Translated from 00d479e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::getTypeInfo` (Xbox PDB): the entry of `name` in the string map at
/// `+8`, 0 when there is none.
pub fn hk_type_info_registry_get_type_info(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    e.call(
        STRING_MAP_GET_WITH_DEFAULT,
        &args![this.addr() + 8, name, 0u32],
    )
    .u32()
}

// Translated from 00d47a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultBuiltinTypeRegistry::getTypeInfoRegistry` (Xbox PDB): the global at `01269b18`.
pub fn hk_default_builtin_type_registry_get_type_info_registry(e: &mut Engine) -> u32 {
    e.mem.u32(TYPE_INFO_REGISTRY)
}

// Translated from 00d47a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultBuiltinTypeRegistry::getClassNameRegistry` (Xbox PDB): the global at `01269b14`.
pub fn hk_default_builtin_type_registry_get_class_name_registry(e: &mut Engine) -> u32 {
    e.mem.u32(CLASS_NAME_REGISTRY)
}

// Translated from 00d47a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultBuiltinTypeRegistry::getVtableClassRegistry` (Xbox PDB): the global at `01269b10`.
pub fn hk_default_builtin_type_registry_get_vtable_class_registry(e: &mut Engine) -> u32 {
    e.mem.u32(VTABLE_CLASS_REGISTRY)
}

// Translated from 00d47a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::registerList` (Xbox PDB): for each class of the 0-ended pointer
/// list at `list` calls slot `0x1c` of the registry with `(class, 0)`.
pub fn hk_dynamic_class_name_registry_register_list(e: &mut Engine, this: Ptr, list: u32) {
    let mut cursor = list;
    loop {
        let class = e.mem.u32(cursor);
        if class == 0 {
            break;
        }
        e.vcall(this.addr(), 0x1c, &args![class, 0u32]);
        cursor += 4;
    }
}

// Translated from 00d47a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::getClassByName` (Xbox PDB): the entry of `name` in the string
/// map at `+0xc`, 0 when there is none.
pub fn hk_dynamic_class_name_registry_get_class_by_name(
    e: &mut Engine,
    this: Ptr,
    name: u32,
) -> u32 {
    e.call(
        STRING_MAP_GET_WITH_DEFAULT,
        &args![this.addr() + 0xc, name, 0u32],
    )
    .u32()
}

// Translated from 00d47aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::registerClass` (Xbox PDB): inserts `class` into the string map
/// at `+0xc` under `name`, which is the class's own name (`00d46d40`) when `name` is 0.
pub fn hk_dynamic_class_name_registry_register_class(
    e: &mut Engine,
    this: Ptr,
    class: u32,
    name: u32,
) {
    let name = if name == 0 {
        e.call(CLASS_GET_NAME, &args![class]).u32()
    } else {
        name
    };
    e.call(STRING_MAP_INSERT, &args![this.addr() + 0xc, name, class]);
}

// Translated from 00d47ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::merge` (Xbox PDB): walks the string map at `other` itself (the
/// exe passes the argument straight to the iterator functions) and calls slot `0x1c` of the
/// registry with `(value, 0)` for every entry.
pub fn hk_dynamic_class_name_registry_merge(e: &mut Engine, this: Ptr, other: u32) {
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![other]).u32();
    if !iterator_is_valid(e, other, iterator) {
        return;
    }
    loop {
        let value = e.call(STRING_MAP_GET_VALUE, &args![other, iterator]).u32();
        e.vcall(this.addr(), 0x1c, &args![value, 0u32]);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![other, iterator]).u32();
        if !iterator_is_valid(e, other, iterator) {
            break;
        }
    }
}

// Translated from 00d47b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::_scalar_deleting_destructor_` (Xbox PDB): destroys the string
/// map at `+0xc` (`00d877d0`), stores the base vtable `0102e388` and, when bit 0 of `flags` is
/// set, returns the block to the memory router's allocator (slot `8`) with the 16-bit size kept
/// at `this + 4`. Returns `this`.
pub fn deleting_destructor_00d47b30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    e.call(STRING_MAP_DESTRUCT, &args![t + 0xc]);
    e.mem.set_u32(t, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d47b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTypeInfoRegistry::_scalar_deleting_destructor_` (Xbox PDB): the same with the string map
/// at `+8`.
pub fn deleting_destructor_00d47b70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    e.call(STRING_MAP_DESTRUCT, &args![t + 8]);
    e.mem.set_u32(t, BASE_VTABLE);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d47bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::getClasses` (Xbox PDB): appends every value of the string map
/// at `+0xc` (iterator order) to the `hkArray` of words at `array`.
pub fn hk_dynamic_class_name_registry_get_classes(e: &mut Engine, this: Ptr, array: u32) {
    let map = this.addr() + 0xc;
    let mut iterator = e.call(STRING_MAP_GET_ITERATOR, &args![map]).u32();
    if !iterator_is_valid(e, map, iterator) {
        return;
    }
    loop {
        let class = e.call(STRING_MAP_GET_VALUE, &args![map, iterator]).u32();
        array_push_word(e, array, class);
        iterator = e.call(STRING_MAP_GET_NEXT, &args![map, iterator]).u32();
        if !iterator_is_valid(e, map, iterator) {
            break;
        }
    }
}

// Translated from 00d47c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVtableClassRegistry::getClassFromVirtualInstance` (Xbox PDB): the class registered for
/// the vtable pointer at the start of `instance`, looked up in the map at `+8` (default 0).
pub fn hk_vtable_class_registry_get_class_from_virtual_instance(
    e: &mut Engine,
    this: Ptr,
    instance: u32,
) -> u32 {
    let vtable = e.mem.u32(instance);
    e.call(MAP_GET_WITH_DEFAULT, &args![this.addr() + 8, vtable, 0u32])
        .u32()
}

// Translated from 00d47c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVtableClassRegistry::registerVtable` (Xbox PDB): inserts `class` under `vtable` into the
/// map at `+8` with the memory router's allocator.
pub fn hk_vtable_class_registry_register_vtable(
    e: &mut Engine,
    this: Ptr,
    vtable: u32,
    class: u32,
) {
    let allocator = router_allocator(e);
    e.call(
        MAP_INSERT,
        &args![this.addr() + 8, allocator, vtable, class],
    );
}

/// The first index from `from` on whose key word in the 8-byte entries at `entries` is not
/// `-1`; one past `mask` (or `from` when that is already past it) when there is none.
fn next_used_entry(e: &mut Engine, entries: u32, from: i32, mask: i32) -> i32 {
    let mut index = from;
    while index <= mask {
        let key = e
            .mem
            .u32(entries.wrapping_add((index as u32).wrapping_mul(8)));
        if key != 0xffff_ffff {
            break;
        }
        index += 1;
    }
    index
}

// Translated from 00d47ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVtableClassRegistry::getClasses` (Xbox PDB): appends the value (`+4`) of every used
/// entry (key word not `-1`) of the map at `+8` (entries of 8 bytes at `+8`, last index at
/// `+0x10`) to the `hkArray` of words at `array`.
pub fn hk_vtable_class_registry_get_classes(e: &mut Engine, this: Ptr, array: u32) {
    let t = this.addr();
    let mask = e.mem.u32(t + 0x10) as i32;
    let entries = e.mem.u32(t + 8);
    let mut index = next_used_entry(e, entries, 0, mask);
    if index > mask {
        return;
    }
    loop {
        let entries = e.mem.u32(t + 8);
        let entry = entries.wrapping_add((index as u32).wrapping_mul(8));
        let class = e.mem.u32(entry + 4);
        array_push_word(e, array, class);
        let mask = e.mem.u32(t + 0x10) as i32;
        let entries = e.mem.u32(t + 8);
        index = next_used_entry(e, entries, index + 1, mask);
        if index > mask {
            break;
        }
    }
}

// Translated from 00d47d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDefaultBuiltinTypeRegistry::reinitialize` (Xbox PDB; `ECX` = `this`, which the code only
/// keeps to pass on): builds three new registries from the memory router's allocator (slot 4)
/// and installs each in its global after releasing the old one (`00c903b0`, when non-null):
/// the class name registry (0x20 bytes: size word `0x20` at `+4`, word 1 at `+6`, vtable
/// `010cf7d4` then `010cf804` once its string map at `+0xc` is built, `010cf830` at `+8`;
/// global `01269b14`), the type info registry (0x1c bytes, vtable `010cf7b0`, string map at
/// `+8`, the word 1 at `+0x18`; global `01269b18`) and the vtable class registry (0x14
/// bytes, vtable `010cf844`, an empty map at `+8` reserved with capacity 0; global
/// `01269b10`). The string map constructors get a dword whose low byte is 0 and whose upper
/// bytes are the stack leftover in the exe, the upper bytes of `this`; they read only the low
/// byte. It ends by running [`hk_default_builtin_type_registry_init`].
pub fn hk_default_builtin_type_registry_reinitialize(e: &mut Engine, this: Ptr) {
    let ignored = this.addr() & 0xffff_ff00;

    let allocator = router_allocator(e);
    let names = e.vcall(allocator, 4, &args![0x20u32]).u32();
    e.mem.set_u16(names + 6, 1);
    e.mem.set_u16(names + 4, 0x20);
    e.mem.set_u32(names, VTABLE_CLASS_NAMES_FIRST);
    e.mem.set_u32(names + 8, VTABLE_CLASS_NAMES_AUX);
    e.call(STRING_MAP_CONSTRUCT, &args![names + 0xc, ignored]);
    e.mem.set_u32(names, VTABLE_CLASS_NAMES);
    replace_registry(e, CLASS_NAME_REGISTRY, names);

    let allocator = router_allocator(e);
    let type_infos = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(type_infos + 6, 1);
    e.mem.set_u16(type_infos + 4, 0x1c);
    e.mem.set_u32(type_infos, VTABLE_TYPE_INFOS);
    e.call(STRING_MAP_CONSTRUCT, &args![type_infos + 8, ignored]);
    e.mem.set_u32(type_infos + 0x18, 1);
    replace_registry(e, TYPE_INFO_REGISTRY, type_infos);

    let allocator = router_allocator(e);
    let vtable_classes = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u16(vtable_classes + 4, 0x14);
    e.mem.set_u16(vtable_classes + 6, 1);
    e.mem.set_u32(vtable_classes, VTABLE_VTABLE_CLASSES);
    let map = vtable_classes + 8;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    replace_registry(e, VTABLE_CLASS_REGISTRY, vtable_classes);

    hk_default_builtin_type_registry_init(e);
}

// Translated from 00d47eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDynamicClassNameRegistry::merge_ov2` (Xbox PDB): asks `other` (slot `0x14`) for its
/// classes in a local `hkArray` (not owned: capacity word `0x80000000`) and registers each
/// through slot `0x1c` of the registry with `(class, 0)`. The array's block is freed through
/// the router's allocator (slot `0x10`, `(data, capacity * 4)`) when its capacity word is not
/// flagged "not owned".
pub fn hk_dynamic_class_name_registry_merge_ov2(e: &mut Engine, this: Ptr, other: u32) {
    e.with_stack(12, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.vcall(other, 0x14, &args![array]);
        let mut index: i32 = 0;
        while index < e.mem.u32(array + 4) as i32 {
            let data = e.mem.u32(array);
            let class = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.vcall(this.addr(), 0x1c, &args![class, 0u32]);
            index += 1;
        }
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if capacity as i32 >= 0 {
            let data = e.mem.u32(array);
            let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00d47820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, no stack words. Answers null unless the three words at
/// `01269b10`, `01269b14` and `01269b18` are all non-zero. Then it allocates 8 bytes through the
/// memory router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10),
/// stores the size as a 16-bit word at +4 and ends with a jump to the constructor `00d478c0`
/// (`this` in `ECX`, no stack words; it stores the count 1 at +6 and a vtable word), whose answer
/// it returns. Ghidra made no function here; the address is stored in the data word at 011b4df8.
pub fn fn_00d47820(e: &mut Engine) -> u32 {
    if e.mem.u32(0x0126_9b10) == 0 || e.mem.u32(0x0126_9b14) == 0 || e.mem.u32(0x0126_9b18) == 0 {
        return 0;
    }
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![8u32]).u32();
    e.mem.set_u16(block + 4, 8);
    e.call(0x00d4_78c0, &args![block]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_77c0,
            hk_builtin_type_registry_add_type(u32, u32, u32)
        ),
        entry!(0x00d4_7860, hk_default_builtin_type_registry_init()),
        entry!(0x00d4_78c0, construct_00d478c0(Ptr) -> Ptr),
        entry!(
            0x00d4_78e0,
            hk_type_info_registry_register_type_info(Ptr, u32)
        ),
        entry!(0x00d4_7900, hk_type_info_registry_finish_loaded_object(Ptr, u32, u32) -> u32),
        entry!(0x00d4_7940, hk_type_info_registry_cleanup_loaded_object(Ptr, u32, u32) -> u32),
        entry!(0x00d4_7970, hk_type_info_registry_merge(Ptr, u32)),
        entry!(0x00d4_79e0, hk_type_info_registry_get_type_info(Ptr, u32) -> u32),
        entry!(0x00d4_7a00, hk_default_builtin_type_registry_get_type_info_registry() -> u32),
        entry!(0x00d4_7a10, hk_default_builtin_type_registry_get_class_name_registry() -> u32),
        entry!(0x00d4_7a20, hk_default_builtin_type_registry_get_vtable_class_registry() -> u32),
        entry!(
            0x00d4_7a50,
            hk_dynamic_class_name_registry_register_list(Ptr, u32)
        ),
        entry!(0x00d4_7a80, hk_dynamic_class_name_registry_get_class_by_name(Ptr, u32) -> u32),
        entry!(
            0x00d4_7aa0,
            hk_dynamic_class_name_registry_register_class(Ptr, u32, u32)
        ),
        entry!(0x00d4_7ad0, hk_dynamic_class_name_registry_merge(Ptr, u32)),
        entry!(0x00d4_7b30, deleting_destructor_00d47b30(Ptr, u8) -> Ptr),
        entry!(0x00d4_7b70, deleting_destructor_00d47b70(Ptr, u8) -> Ptr),
        entry!(
            0x00d4_7bb0,
            hk_dynamic_class_name_registry_get_classes(Ptr, u32)
        ),
        entry!(0x00d4_7c40, hk_vtable_class_registry_get_class_from_virtual_instance(Ptr, u32) -> u32),
        entry!(
            0x00d4_7c60,
            hk_vtable_class_registry_register_vtable(Ptr, u32, u32)
        ),
        entry!(0x00d4_7ce0, hk_vtable_class_registry_get_classes(Ptr, u32)),
        entry!(
            0x00d4_7d70,
            hk_default_builtin_type_registry_reinitialize(Ptr)
        ),
        entry!(
            0x00d4_7eb0,
            hk_dynamic_class_name_registry_merge_ov2(Ptr, u32)
        ),
        entry!(0x00d4_7820, fn_00d47820() -> u32),
    ]
}

#[cfg(test)]
mod translated_tests {
    use super::*;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    /// Doubles used behind the registries' virtual slots.
    const DOUBLE_A: u32 = 0x0900_0a01;
    const DOUBLE_B: u32 = 0x0900_0a02;
    const DOUBLE_C: u32 = 0x0900_0a03;
    const DOUBLE_FREE: u32 = 0x0900_0a04;
    const DOUBLE_ALLOCATE: u32 = 0x0900_0a05;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An object whose vtable holds the given doubles at the given byte offsets.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (offset, double) in slots {
            e.mem.set_u32(vtable + offset, *double);
            answer(e, *double, 0);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Installs a memory router whose allocator has the given slot doubles; returns the allocator.
    fn router(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let allocator = object_with(e, slots);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        allocator
    }

    /// String map iterator doubles: the iterator starts at `first`, is valid up to `last`,
    /// steps by 1, and answers value `it * 100` and key `it * 1000`.
    fn iterator_doubles(e: &mut Engine, first: u32, last: u32) {
        answer(e, STRING_MAP_GET_ITERATOR, first);
        e.register_double(STRING_MAP_IS_VALID, move |e, a| {
            e.mem.set_u8(a[1], u8::from(a[2] <= last));
            Ret::default()
        });
        e.register_double(STRING_MAP_GET_NEXT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register_double(STRING_MAP_GET_VALUE, |_, a| Ret {
            eax: a[1] * 100,
            ..Ret::default()
        });
        e.register_double(STRING_MAP_GET_KEY, |_, a| Ret {
            eax: a[1] * 1000,
            ..Ret::default()
        });
    }

    /// Installs the three registries with one double each in the slots `init` and `addType` use.
    fn global_registries(e: &mut Engine) -> (u32, u32, u32) {
        let names = object_with(e, &[(0x1c, DOUBLE_A), (0x24, 0x0900_0a06)]);
        let type_infos = object_with(e, &[(0xc, DOUBLE_B)]);
        let vtables = object_with(e, &[(0xc, DOUBLE_C)]);
        e.mem.set_u32(CLASS_NAME_REGISTRY, names);
        e.mem.set_u32(TYPE_INFO_REGISTRY, type_infos);
        e.mem.set_u32(VTABLE_CLASS_REGISTRY, vtables);
        (names, type_infos, vtables)
    }

    #[test]
    fn add_type_registers_in_the_three_registries() {
        let mut e = engine();
        let (names, type_infos, vtables) = global_registries(&mut e);
        answer(&mut e, CLASS_GET_NAME, 0x7000);
        answer(&mut e, CLASS_GET_NUM_INTERFACES, 2);
        let type_info = e.mem.alloc(0x20);
        e.mem.set_u32(type_info + 0xc, 0x5555);
        e.call(0x00d4_77c0, &args![VCALL_THIS, type_info, 0x6000u32]);
        assert_eq!(calls_to(&e, DOUBLE_A), vec![vec![names, 0x6000, 0x7000]]);
        assert_eq!(calls_to(&e, DOUBLE_B), vec![vec![type_infos, type_info]]);
        assert_eq!(calls_to(&e, DOUBLE_C), vec![vec![vtables, 0x5555, 0x6000]]);
        // No interfaces, or a negative count (signed compare): no vtable registration.
        for count in [0, 0xffff_ffff] {
            answer(&mut e, CLASS_GET_NUM_INTERFACES, count);
            e.call(0x00d4_77c0, &args![VCALL_THIS, type_info, 0x6000u32]);
            assert_eq!(calls_to(&e, DOUBLE_C).len(), 1);
        }
    }

    #[test]
    fn init_registers_the_listed_type_infos() {
        let mut e = engine();
        let (names, type_infos, vtables) = global_registries(&mut e);
        e.mem.set_u32(TYPE_INFO_LIST, 0x1111);
        e.mem.set_u32(TYPE_INFO_LIST + 4, 0x2222);
        answer(&mut e, REGISTER_LISTED_VTABLES, 0);
        e.call(0x00d4_7860, &args![]);
        assert_eq!(
            calls_to(&e, 0x0900_0a06),
            vec![vec![names, INIT_CLASS_OBJECT]]
        );
        assert_eq!(
            calls_to(&e, DOUBLE_B),
            vec![vec![type_infos, 0x1111], vec![type_infos, 0x2222]]
        );
        assert_eq!(
            calls_to(&e, REGISTER_LISTED_VTABLES),
            vec![vec![vtables, TYPE_INFO_LIST, VTABLE_LIST]]
        );
    }

    #[test]
    fn constructor_stores_the_vtable_and_runs_init() {
        let mut e = engine();
        global_registries(&mut e);
        answer(&mut e, REGISTER_LISTED_VTABLES, 0);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d4_78c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE_DEFAULT_REGISTRY);
        assert_eq!(calls_to(&e, REGISTER_LISTED_VTABLES).len(), 1);
    }

    #[test]
    fn register_type_info_inserts_under_its_first_word() {
        let mut e = engine();
        answer(&mut e, STRING_MAP_INSERT, 0);
        let this = e.mem.alloc(0x20);
        let type_info = e.mem.alloc(0x10);
        e.mem.set_u32(type_info, 0x7777);
        e.call(0x00d4_78e0, &args![this, type_info]);
        assert_eq!(
            calls_to(&e, STRING_MAP_INSERT),
            vec![vec![this + 8, 0x7777, type_info]]
        );
    }

    /// A type info entry whose words +4 and +8 hold the given function pointers.
    fn entry_with(e: &mut Engine, finish: u32, cleanup: u32) -> u32 {
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, finish);
        e.mem.set_u32(entry + 8, cleanup);
        entry
    }

    #[test]
    fn finish_loaded_object_calls_the_entry_function() {
        let mut e = engine();
        answer(&mut e, DOUBLE_A, 0);
        let entry = entry_with(&mut e, DOUBLE_A, 0);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, entry);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, 5);
        let got = e
            .call(0x00d4_7900, &args![this, 0x4444u32, 0x8888u32])
            .u32();
        assert_eq!(got, entry);
        assert_eq!(
            calls_to(&e, STRING_MAP_GET_WITH_DEFAULT),
            vec![vec![this + 8, 0x8888, 0]]
        );
        assert_eq!(calls_to(&e, DOUBLE_A), vec![vec![0x4444, 5]]);
        // No function: the entry is still returned; no entry: 0.
        let bare = entry_with(&mut e, 0, 0);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, bare);
        assert_eq!(e.call(0x00d4_7900, &args![this, 1u32, 2u32]).u32(), bare);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 1);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, 0);
        assert_eq!(e.call(0x00d4_7900, &args![this, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn cleanup_loaded_object_calls_the_entry_function() {
        let mut e = engine();
        answer(&mut e, DOUBLE_A, 0);
        let entry = entry_with(&mut e, 0, DOUBLE_A);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, entry);
        let this = e.mem.alloc(0x30);
        let got = e
            .call(0x00d4_7940, &args![this, 0x4444u32, 0x8888u32])
            .u32();
        assert_eq!(got, entry);
        assert_eq!(
            calls_to(&e, STRING_MAP_GET_WITH_DEFAULT),
            vec![vec![this + 8, 0x8888, 0]]
        );
        assert_eq!(calls_to(&e, DOUBLE_A), vec![vec![0x4444]]);
        let bare = entry_with(&mut e, 0, 0);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, bare);
        assert_eq!(e.call(0x00d4_7940, &args![this, 1u32, 2u32]).u32(), bare);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 1);
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, 0);
        assert_eq!(e.call(0x00d4_7940, &args![this, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn type_info_merge_copies_every_entry() {
        let mut e = engine();
        iterator_doubles(&mut e, 10, 11);
        answer(&mut e, STRING_MAP_INSERT, 0);
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.call(0x00d4_7970, &args![this, other]);
        assert_eq!(
            calls_to(&e, STRING_MAP_INSERT),
            vec![vec![this + 8, 10_000, 1000], vec![this + 8, 11_000, 1100]]
        );
        assert_eq!(calls_to(&e, STRING_MAP_GET_ITERATOR), vec![vec![other + 8]]);
        // The value is fetched before the key.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [STRING_MAP_GET_VALUE, STRING_MAP_GET_KEY].contains(a))
            .take(2)
            .collect();
        assert_eq!(order, vec![STRING_MAP_GET_VALUE, STRING_MAP_GET_KEY]);
        // An invalid first iterator copies nothing.
        iterator_doubles(&mut e, 12, 11);
        e.call(0x00d4_7970, &args![this, other]);
        assert_eq!(calls_to(&e, STRING_MAP_INSERT).len(), 2);
    }

    #[test]
    fn get_type_info_looks_the_name_up() {
        let mut e = engine();
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, 0x1234);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d4_79e0, &args![this, 0x8888u32]).u32(), 0x1234);
        assert_eq!(
            calls_to(&e, STRING_MAP_GET_WITH_DEFAULT),
            vec![vec![this + 8, 0x8888, 0]]
        );
    }

    #[test]
    fn the_getters_answer_the_globals() {
        let mut e = engine();
        e.mem.set_u32(TYPE_INFO_REGISTRY, 0x11);
        e.mem.set_u32(CLASS_NAME_REGISTRY, 0x22);
        e.mem.set_u32(VTABLE_CLASS_REGISTRY, 0x33);
        assert_eq!(e.call(0x00d4_7a00, &args![]).u32(), 0x11);
        assert_eq!(e.call(0x00d4_7a10, &args![]).u32(), 0x22);
        assert_eq!(e.call(0x00d4_7a20, &args![]).u32(), 0x33);
    }

    #[test]
    fn register_list_registers_each_class() {
        let mut e = engine();
        let this = object_with(&mut e, &[(0x1c, DOUBLE_A)]);
        let list = e.mem.alloc(16);
        e.mem.set_u32(list, 0xa);
        e.mem.set_u32(list + 4, 0xb);
        e.call(0x00d4_7a50, &args![this, list]);
        assert_eq!(
            calls_to(&e, DOUBLE_A),
            vec![vec![this, 0xa, 0], vec![this, 0xb, 0]]
        );
        // An empty list does nothing.
        e.mem.set_u32(list, 0);
        e.call(0x00d4_7a50, &args![this, list]);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 2);
    }

    #[test]
    fn get_class_by_name_looks_the_name_up() {
        let mut e = engine();
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, 0x4321);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d4_7a80, &args![this, 0x8888u32]).u32(), 0x4321);
        assert_eq!(
            calls_to(&e, STRING_MAP_GET_WITH_DEFAULT),
            vec![vec![this + 0xc, 0x8888, 0]]
        );
    }

    #[test]
    fn register_class_defaults_the_name_to_the_class_name() {
        let mut e = engine();
        answer(&mut e, STRING_MAP_INSERT, 0);
        answer(&mut e, CLASS_GET_NAME, 0x7000);
        let this = e.mem.alloc(0x20);
        e.call(0x00d4_7aa0, &args![this, 0x6000u32, 0x9000u32]);
        assert!(calls_to(&e, CLASS_GET_NAME).is_empty());
        e.call(0x00d4_7aa0, &args![this, 0x6000u32, 0u32]);
        assert_eq!(calls_to(&e, CLASS_GET_NAME), vec![vec![0x6000]]);
        assert_eq!(
            calls_to(&e, STRING_MAP_INSERT),
            vec![
                vec![this + 0xc, 0x9000, 0x6000],
                vec![this + 0xc, 0x7000, 0x6000]
            ]
        );
    }

    #[test]
    fn class_name_merge_registers_every_value() {
        let mut e = engine();
        iterator_doubles(&mut e, 10, 11);
        let this = object_with(&mut e, &[(0x1c, DOUBLE_A)]);
        let other = e.mem.alloc(0x20);
        e.call(0x00d4_7ad0, &args![this, other]);
        assert_eq!(
            calls_to(&e, DOUBLE_A),
            vec![vec![this, 1000, 0], vec![this, 1100, 0]]
        );
        assert_eq!(calls_to(&e, STRING_MAP_GET_ITERATOR), vec![vec![other]]);
        iterator_doubles(&mut e, 12, 11);
        e.call(0x00d4_7ad0, &args![this, other]);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 2);
    }

    /// The shared check of the two deleting destructors: `map_offset` is where the string map is.
    fn check_deleting(address: u32, map_offset: u32) {
        let mut e = engine();
        answer(&mut e, STRING_MAP_DESTRUCT, 0);
        let allocator = router(&mut e, &[(8, DOUBLE_FREE)]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x30);
        assert_eq!(e.call(address, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(
            calls_to(&e, STRING_MAP_DESTRUCT),
            vec![vec![this + map_offset]]
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert!(calls_to(&e, DOUBLE_FREE).is_empty());
        assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DOUBLE_FREE), vec![vec![allocator, this, 0x30]]);
    }

    #[test]
    fn class_name_registry_deleting_destructor() {
        check_deleting(0x00d4_7b30, 0xc);
    }

    #[test]
    fn type_info_registry_deleting_destructor() {
        check_deleting(0x00d4_7b70, 8);
    }

    /// An `hkArray` of words at the returned address with `size` 0, room for `capacity` words
    /// and a growth double (`00c90940`) that makes room for 8 words.
    fn small_array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(16);
        let data = e.mem.alloc(32);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        e.register_double(RESERVE_MORE, |e, a| {
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        array
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(e.mem.u32(array) + 4 * i))
            .collect()
    }

    #[test]
    fn class_name_get_classes_appends_every_value() {
        let mut e = engine();
        iterator_doubles(&mut e, 10, 12);
        let allocator = router(&mut e, &[]);
        let this = e.mem.alloc(0x20);
        let array = small_array(&mut e, 2);
        e.call(0x00d4_7bb0, &args![this, array]);
        assert_eq!(array_words(&e, array), vec![1000, 1100, 1200]);
        assert_eq!(
            calls_to(&e, STRING_MAP_GET_ITERATOR),
            vec![vec![this + 0xc]]
        );
        // The third push found size == capacity (2) and grew the array.
        assert_eq!(calls_to(&e, RESERVE_MORE), vec![vec![allocator, array, 4]]);
        // An invalid first iterator appends nothing.
        iterator_doubles(&mut e, 13, 12);
        let empty = small_array(&mut e, 2);
        e.call(0x00d4_7bb0, &args![this, empty]);
        assert!(array_words(&e, empty).is_empty());
    }

    #[test]
    fn class_from_virtual_instance_looks_the_vtable_up() {
        let mut e = engine();
        answer(&mut e, MAP_GET_WITH_DEFAULT, 0x4321);
        let this = e.mem.alloc(0x20);
        let instance = e.mem.alloc(8);
        e.mem.set_u32(instance, 0x010c_0000);
        let got = e.call(0x00d4_7c40, &args![this, instance]).u32();
        assert_eq!(got, 0x4321);
        assert_eq!(
            calls_to(&e, MAP_GET_WITH_DEFAULT),
            vec![vec![this + 8, 0x010c_0000, 0]]
        );
    }

    #[test]
    fn register_vtable_inserts_with_the_router_allocator() {
        let mut e = engine();
        answer(&mut e, MAP_INSERT, 0);
        let allocator = router(&mut e, &[]);
        let this = e.mem.alloc(0x20);
        e.call(0x00d4_7c60, &args![this, 0x010c_0000u32, 0x6000u32]);
        assert_eq!(
            calls_to(&e, MAP_INSERT),
            vec![vec![this + 8, allocator, 0x010c_0000, 0x6000]]
        );
    }

    #[test]
    fn vtable_get_classes_skips_unused_entries() {
        let mut e = engine();
        router(&mut e, &[]);
        let this = e.mem.alloc(0x20);
        let entries = e.mem.alloc(32);
        for (index, (key, value)) in [(NO_KEY, 0u32), (5, 0x50), (NO_KEY, 0), (7, 0x70)]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(entries + 8 * index as u32, key);
            e.mem.set_u32(entries + 8 * index as u32 + 4, value);
        }
        e.mem.set_u32(this + 8, entries);
        e.mem.set_u32(this + 0x10, 3);
        let array = small_array(&mut e, 8);
        e.call(0x00d4_7ce0, &args![this, array]);
        assert_eq!(array_words(&e, array), vec![0x50, 0x70]);
        // All entries unused, and a negative last index: nothing is appended.
        for index in 0..4 {
            e.mem.set_u32(entries + 8 * index, NO_KEY);
        }
        let none = small_array(&mut e, 8);
        e.call(0x00d4_7ce0, &args![this, none]);
        assert!(array_words(&e, none).is_empty());
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        e.call(0x00d4_7ce0, &args![this, none]);
        assert!(array_words(&e, none).is_empty());
    }

    /// The key word of an unused map entry.
    const NO_KEY: u32 = 0xffff_ffff;

    #[test]
    fn reinitialize_builds_and_installs_the_three_registries() {
        let mut e = engine();
        // Old registries in the globals are released.
        e.mem.set_u32(CLASS_NAME_REGISTRY, 0xaaaa);
        e.mem.set_u32(TYPE_INFO_REGISTRY, 0xbbbb);
        e.mem.set_u32(VTABLE_CLASS_REGISTRY, 0xcccc);
        answer(&mut e, REMOVE_REFERENCE, 0);
        answer(&mut e, STRING_MAP_CONSTRUCT, 0);
        answer(&mut e, MAP_RESERVE, 0);
        answer(&mut e, REGISTER_LISTED_VTABLES, 0);
        // The allocator hands out three blocks; `init` then finds its slots in the new vtables.
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x40)).collect();
        let mut next = blocks.clone().into_iter();
        let allocator = router(&mut e, &[]);
        let vtable = e.mem.u32(allocator);
        e.mem.set_u32(vtable + 4, DOUBLE_ALLOCATE);
        e.register_double(DOUBLE_ALLOCATE, move |_, _| Ret {
            eax: next.next().unwrap(),
            ..Ret::default()
        });
        e.mem.set_u32(VTABLE_CLASS_NAMES + 0x24, DOUBLE_A);
        e.mem.set_u32(VTABLE_TYPE_INFOS + 0xc, DOUBLE_B);
        answer(&mut e, DOUBLE_A, 0);
        answer(&mut e, DOUBLE_B, 0);
        let this: u32 = 0x1234_5678;
        e.call(0x00d4_7d70, &args![this]);
        let [names, type_infos, vtable_classes] = [blocks[0], blocks[1], blocks[2]];
        assert_eq!(
            calls_to(&e, DOUBLE_ALLOCATE),
            vec![
                vec![allocator, 0x20],
                vec![allocator, 0x1c],
                vec![allocator, 0x14]
            ]
        );
        assert_eq!(e.mem.u32(CLASS_NAME_REGISTRY), names);
        assert_eq!(e.mem.u32(TYPE_INFO_REGISTRY), type_infos);
        assert_eq!(e.mem.u32(VTABLE_CLASS_REGISTRY), vtable_classes);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0xaaaa], vec![0xbbbb], vec![0xcccc]]
        );
        assert_eq!((e.mem.u16(names + 4), e.mem.u16(names + 6)), (0x20, 1));
        assert_eq!(e.mem.u32(names), VTABLE_CLASS_NAMES);
        assert_eq!(e.mem.u32(names + 8), VTABLE_CLASS_NAMES_AUX);
        assert_eq!(
            (e.mem.u16(type_infos + 4), e.mem.u16(type_infos + 6)),
            (0x1c, 1)
        );
        assert_eq!(e.mem.u32(type_infos), VTABLE_TYPE_INFOS);
        assert_eq!(e.mem.u32(type_infos + 0x18), 1);
        assert_eq!(
            (e.mem.u16(vtable_classes + 4), e.mem.u16(vtable_classes + 6)),
            (0x14, 1)
        );
        assert_eq!(e.mem.u32(vtable_classes), VTABLE_VTABLE_CLASSES);
        let map = vtable_classes + 8;
        assert_eq!(
            (e.mem.u32(map), e.mem.u32(map + 4), e.mem.u32(map + 8)),
            (0, 0, 0xffff_ffff)
        );
        assert_eq!(calls_to(&e, MAP_RESERVE), vec![vec![map, allocator, 0]]);
        assert_eq!(
            calls_to(&e, STRING_MAP_CONSTRUCT),
            vec![
                vec![names + 0xc, 0x1234_5600],
                vec![type_infos + 8, 0x1234_5600]
            ]
        );
        // `init` ran on the new registries.
        assert_eq!(calls_to(&e, DOUBLE_A), vec![vec![names, INIT_CLASS_OBJECT]]);
        assert_eq!(
            calls_to(&e, REGISTER_LISTED_VTABLES),
            vec![vec![vtable_classes, TYPE_INFO_LIST, VTABLE_LIST]]
        );
        // Empty globals are not released.
        e.mem.set_u32(CLASS_NAME_REGISTRY, 0);
        e.mem.set_u32(TYPE_INFO_REGISTRY, 0);
        e.mem.set_u32(VTABLE_CLASS_REGISTRY, 0);
        let mut again = blocks.clone().into_iter();
        e.register_double(DOUBLE_ALLOCATE, move |_, _| Ret {
            eax: again.next().unwrap(),
            ..Ret::default()
        });
        e.call(0x00d4_7d70, &args![this]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 3);
    }

    #[test]
    fn merge_ov2_registers_the_classes_of_the_other_registry() {
        let mut e = engine();
        let this = object_with(&mut e, &[(0x1c, DOUBLE_A)]);
        let allocator = router(&mut e, &[(0x10, DOUBLE_FREE)]);
        // The other registry fills the array with two words in an owned block of four.
        let block = e.mem.alloc(16);
        e.mem.write(block, &[0xa, 0, 0, 0, 0xb, 0, 0, 0]);
        let other = object_with(&mut e, &[(0x14, DOUBLE_B)]);
        e.register_double(DOUBLE_B, move |e, a| {
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 4, 2);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.call(0x00d4_7eb0, &args![this, other]);
        assert_eq!(
            calls_to(&e, DOUBLE_A),
            vec![vec![this, 0xa, 0], vec![this, 0xb, 0]]
        );
        assert_eq!(calls_to(&e, DOUBLE_FREE), vec![vec![allocator, block, 16]]);
        // A block flagged "not owned" is not freed.
        e.register_double(DOUBLE_B, move |e, a| {
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 4, 1);
            e.mem.set_u32(a[1] + 8, 0x8000_0004);
            Ret::default()
        });
        e.call(0x00d4_7eb0, &args![this, other]);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 3);
        assert_eq!(calls_to(&e, DOUBLE_FREE).len(), 1);
        // An other registry that adds nothing registers nothing.
        e.register_double(DOUBLE_B, |e, a| {
            e.mem.set_u32(a[1] + 8, 0x8000_0000);
            Ret::default()
        });
        e.call(0x00d4_7eb0, &args![this, other]);
        assert_eq!(calls_to(&e, DOUBLE_A).len(), 3);
    }

    #[test]
    fn the_builtin_type_registry_waits_for_the_three_registries() {
        let mut e = Engine::new();
        e.map(0x0126_9000, 0x1000);
        e.register(0x00c8_5750, |_, _| {
            panic!("no allocation while a registry is missing")
        });
        assert_eq!(e.call(0x00d4_7820, &args![]).u32(), 0);
        e.mem.set_u32(0x0126_9b10, 1);
        e.mem.set_u32(0x0126_9b14, 1);
        assert_eq!(e.call(0x00d4_7820, &args![]).u32(), 0);
        // All three there: 8 bytes, the size halfword, the constructor's answer.
        e.mem.set_u32(0x0126_9b18, 1);
        e.map(0x0000_7000, 0x1000);
        e.mem.set_u32(0x7010, 0x7100);
        e.mem.set_u32(0x7100, 0x7200);
        e.mem.set_u32(0x7204, 0x00ab_0004);
        e.register(0x00c8_5750, |_, _| 0x7000u32.into_ret());
        e.register(0x00ab_0004, |_, a| {
            assert_eq!(a, [0x7100, 8]);
            0x7800u32.into_ret()
        });
        e.register(0x00d4_78c0, |_, a| a[0].wrapping_add(1).into_ret());
        assert_eq!(e.call(0x00d4_7820, &args![]).u32(), 0x7801);
        assert_eq!(e.mem.u16(0x7804), 8);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    /// Any of the three words being zero gives null without allocating.
    #[test]
    fn fn_00d47820_needs_all_three_words() {
        for missing in 0..3u32 {
            let (mut e, sizes) = allocator_engine();
            e.map(0x0126_9000, 0x1000);
            for word in 0..3u32 {
                e.set_global(0x0126_9b10 + 4 * word, u32::from(word != missing));
            }
            assert_eq!(e.call(0x00d4_7820, &args![]).u32(), 0);
            assert!(sizes.borrow().is_empty());
        }
    }

    /// With the three words set it allocates 8 bytes and returns the constructor's answer.
    #[test]
    fn fn_00d47820_allocates_and_calls_the_constructor() {
        let (mut e, sizes) = allocator_engine();
        e.map(0x0126_9000, 0x1000);
        for word in 0..3u32 {
            e.set_global(0x0126_9b10 + 4 * word, 1u32);
        }
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d4_78c0, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u16(a[0] + 4)));
            Ret {
                eax: 0x4321,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00d4_7820, &args![]).u32(), 0x4321);
        assert_eq!(*sizes.borrow(), vec![8]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, 8);
    }
}
