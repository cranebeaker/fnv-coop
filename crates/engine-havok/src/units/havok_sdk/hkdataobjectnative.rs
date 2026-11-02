//! `Havok SDK/hkdataobjectnative.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Callees outside this file (engine-map names are Xbox PDB names). The `hkClass`, `hkClassMember`
// and `hkClassMemberAccessor` methods are thiscall with the stack words noted.
/// `hkReferencedObject` reference-count increment and decrement (thiscall, no stack words).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkClass::getName` (the word at `+0`), `getParent`, `getNumMembers`, `getMember(index)`,
/// `getDeclaredMember(index)`, `getMemberByName(name)`, `getObjectSize`, `hasVtable(flag_out)`
/// (returns `flag_out`), `getFlags`, `getNumDeclaredMembers`, `getDescribedVersion`,
/// `getDeclaredMemberIndexByName(name)` and `getMemberIndexByName(name)`.
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const CLASS_GET_PARENT: u32 = 0x00d4_6d50;
const CLASS_GET_NUM_MEMBERS: u32 = 0x00d4_6f00;
const CLASS_GET_MEMBER: u32 = 0x00d4_6f20;
const CLASS_GET_NUM_DECLARED_MEMBERS: u32 = 0x00d4_6f70;
const CLASS_GET_DECLARED_MEMBER: u32 = 0x00d4_6f80;
const CLASS_GET_MEMBER_BY_NAME: u32 = 0x00d4_6fe0;
const CLASS_GET_MEMBER_INDEX_BY_NAME: u32 = 0x00d4_7030;
const CLASS_GET_DECLARED_MEMBER_INDEX_BY_NAME: u32 = 0x00d4_7080;
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
const CLASS_GET_DESCRIBED_VERSION: u32 = 0x00d4_7120;
const CLASS_GET_FLAGS: u32 = 0x00d4_7280;
/// `hkClassMember::getClassMemberTypeProperties(type)` (cdecl, one word; the record's 16-bit
/// size is at `+8`), `getClass`, `getCstyleArraySize` and `getAttribute(name)`.
const MEMBER_TYPE_PROPERTIES: u32 = 0x00d8_68a0;
const MEMBER_GET_CLASS: u32 = 0x00d8_6920;
const MEMBER_GET_ARRAY_SIZE: u32 = 0x00d8_6940;
const MEMBER_GET_ATTRIBUTE: u32 = 0x00d8_6960;
/// `hkClassMemberAccessor::hkClassMemberAccessor(accessor, name)` (the `this` is an 8-byte
/// local), `isOk(flag_out)`, `getAddress`, `asPointer(0)`, `asCstring(0)`, `asStringPtr(0)`,
/// `asVariant(0)` and `getClassMember`.
const ACCESSOR_INIT: u32 = 0x00d8_71a0;
const ACCESSOR_IS_OK: u32 = 0x00d8_71e0;
const ACCESSOR_GET_ADDRESS: u32 = 0x00d8_7200;
const ACCESSOR_AS_POINTER: u32 = 0x00d8_7210;
const ACCESSOR_AS_CSTRING: u32 = 0x00d8_7220;
const ACCESSOR_AS_STRING_PTR: u32 = 0x00d8_7230;
const ACCESSOR_AS_VARIANT: u32 = 0x00d8_7360;
const ACCESSOR_GET_CLASS_MEMBER: u32 = 0x00d8_7370;
/// The typed address functions of the accessor (`(index 0)`, thiscall on the accessor): a
/// `float`, a half stored as 16 bits, `int`, `unsigned int`, 64-bit signed, 64-bit unsigned,
/// `unsigned long`, `short`, `unsigned short`, `char` and `unsigned char`.
const ACCESSOR_ADDRESS_FLOAT: u32 = 0x00d8_7250;
const ACCESSOR_ADDRESS_HALF: u32 = 0x00d8_7260;
const ACCESSOR_ADDRESS_I32: u32 = 0x00d8_7270;
const ACCESSOR_ADDRESS_U32: u32 = 0x00d8_7280;
const ACCESSOR_ADDRESS_I64: u32 = 0x00d8_7290;
const ACCESSOR_ADDRESS_U64: u32 = 0x00d8_72a0;
const ACCESSOR_ADDRESS_ULONG: u32 = 0x00d8_72b0;
const ACCESSOR_ADDRESS_I16: u32 = 0x00d8_72c0;
const ACCESSOR_ADDRESS_U16: u32 = 0x00d8_72d0;
const ACCESSOR_ADDRESS_I8: u32 = 0x00d8_72e0;
const ACCESSOR_ADDRESS_U8: u32 = 0x00d8_72f0;
/// `hkClassAccessor` constructors: `(object, class)` words (`00d873b0`, two stack words) and
/// a copy of the pair at an address (`00d873d0`, one stack word).
const CLASS_ACCESSOR_INIT_PAIR: u32 = 0x00d8_73b0;
const CLASS_ACCESSOR_COPY: u32 = 0x00d8_73d0;
/// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>`: constructor (one
/// stack word), `getIterator`, `getValue(it)`, `getNext(it)`, `isValid(flag_out, it)`,
/// `insert(key, value)`, destructor and `getWithDefault(key, default)`.
const MAP_CONSTRUCT: u32 = 0x00d8_7790;
const MAP_GET_ITERATOR: u32 = 0x00d8_7410;
const MAP_GET_VALUE: u32 = 0x00d8_7440;
const MAP_GET_NEXT: u32 = 0x00d8_7480;
const MAP_IS_VALID: u32 = 0x00d8_74b0;
const MAP_INSERT: u32 = 0x00d8_74d0;
const MAP_DESTRUCT: u32 = 0x00d8_77d0;
const MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
/// `hkDataClass::getName` (calls slot 8 of the implementation) and the `hkDataObject`
/// constructor `(this, implementation)`.
const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
/// `hkDataObjectUtil::getTypeFromMemberType(type, subtype, class, tuple_size)` (cdecl).
const TYPE_FROM_MEMBER_TYPE: u32 = 0x00dc_1c50;
/// `hkString::strCmp` and `hkString::memSet` (cdecl), and `hkArrayUtil::_reserve`'s growth
/// helper `(allocator, array, element_size)` (cdecl).
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const MEMORY_SET: u32 = 0x00cb_1280;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// The message stream (`hkOstream`) the error reports build: constructor `(buffer, size)`,
/// `operator<<(text)` (returns the stream), `operator<<(pointer)` and destructor.
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const STREAM_WRITE_POINTER: u32 = 0x00cc_0120;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The error object (a pointer at `01268360`; slot `0xc` reports `(severity, id, text, file,
/// line)`) and the source file name it is given.
const ERROR_OBJECT: u32 = 0x0126_8360;
const SOURCE_FILE_NAME: u32 = 0x010d_6394;
/// The class registry singleton (a pointer at `01268b08`): slot `0xc` answers the
/// vtable-class registry, `0x10` and `0x14` the two registries the world keeps.
const REGISTRY_SINGLETON: u32 = 0x0126_8b08;
/// The `hk.DataObjectType` attribute name (a pointer at `011b7958`) and `"typeName"`.
const DATA_OBJECT_TYPE_ATTRIBUTE: u32 = 0x011b_7958;
const TYPE_NAME_TEXT: u32 = 0x010d_4790;
/// Vtables (RTTI names): `hkDataObjectNative`, `hkDataWorldNative`, `hkDataClassNative`,
/// `hkDataArrayNative`, `hkDataRefCounted` and `hkBaseObject`.
const OBJECT_VTABLE: u32 = 0x010d_6334;
const WORLD_VTABLE: u32 = 0x010d_6518;
const CLASS_VTABLE: u32 = 0x010d_62fc;
const ARRAY_VTABLE: u32 = 0x010d_642c;
const REF_COUNTED_VTABLE: u32 = 0x010d_62f4;
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// Message texts and report ids of the errors (strings in the exe).
const TEXT_MOST_DERIVED: u32 = 0x010d_63bc;
const TEXT_AT: u32 = 0x010d_63f8;
const TEXT_REPLACED_WITH_NULL: u32 = 0x010d_6400;
const TEXT_IGNORE_OBJECT: u32 = 0x010d_64bc;
const TEXT_CLASS_OPEN: u32 = 0x010d_64d0;
const TEXT_NOT_REGISTERED_PROVIDED: u32 = 0x010d_64dc;
const TEXT_CANNOT_SET_CONTENTS: u32 = 0x010d_6544;
const TEXT_NOT_REGISTERED_IGNORE: u32 = 0x010d_656c;

layout! {
    /// `hkDataWorldNative` (RTTI name; the Xbox PDB has no record), 0x28 bytes.
    pub struct HkDataWorldNative: 0x28 {
        /// Size of the block, for the memory router.
        0x04 memory_size: u16,
        0x06 reference_count: u16,
        /// The registry asked for a class by name (slot `0x10`); `setClassRegistry` keeps a
        /// reference to it. Role confirmed from `findClass`.
        0x08 class_registry: Ptr,
        /// The registry asked for the class of a virtual object (slot `0x10`).
        0x0c instance_registry: Ptr,
        /// The class cache: an `hkCachedHashMap` keyed by class name.
        0x10 classes: Inline<ClassMap>,
        /// What `setContents` stored (object address).
        0x20 contents: Ptr,
        /// Its `hkDataClassNative`.
        0x24 contents_class: Ptr,
    }
    /// The 0x10-byte `hkCachedHashMap` embedded in the world.
    pub struct ClassMap: 0x10 {}
    /// `hkDataClassNative` (RTTI name), 0x14 bytes.
    pub struct HkDataClassNative: 0x14 {
        0x04 memory_size: u16,
        0x06 external_count: u16,
        0x08 reference_count: u32,
        /// The `hkClass`.
        0x0c class: Ptr,
        /// The `hkDataWorldNative` it belongs to.
        0x10 world: Ptr<HkDataWorldNative>,
    }
    /// `hkDataObjectNative` (RTTI name), 0x18 bytes.
    pub struct HkDataObjectNative: 0x18 {
        0x04 memory_size: u16,
        0x06 external_count: u16,
        0x08 reference_count: u32,
        /// The embedded `hkClassAccessor`: the object's address ...
        0x0c object: Ptr,
        /// ... and its `hkClass`.
        0x10 class: Ptr,
        0x14 world: Ptr<HkDataWorldNative>,
    }
    /// `hkDataArrayNative` (RTTI name), 0x2c bytes.
    pub struct HkDataArrayNative: 0x2c {
        0x04 memory_size: u16,
        0x06 external_count: u16,
        0x08 reference_count: u32,
        0x0c world: Ptr<HkDataWorldNative>,
        /// Address of the first element.
        0x10 data: Ptr,
        0x14 count: u32,
        /// Bytes between elements.
        0x18 stride: u32,
        /// `hkClassMember` type code of the elements.
        0x1c member_type: u32,
        /// `hkClass` of the elements (zero for plain types).
        0x20 class: Ptr,
        /// `getTypeFromMemberType(0x16, type, class, tuple_size)` minus `0x10`, at least
        /// `0x10` (what it means is unconfirmed).
        0x24 element_type: u32,
        /// Values per element (zero for a plain array).
        0x28 tuple_size: u32,
    }
}

/// The allocator the memory router hands out (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// A block of `size` bytes from the router's allocator (slot `4`).
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let allocator = router_allocator(e);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// The header every `hkDataRefCounted` object gets: size at `+4`, counts at `+6` and `+8`
/// cleared, the vtable at `+0`.
fn init_ref_counted(e: &mut Engine, object: u32, size: u16, vtable: u32) {
    e.mem.set_u16(object + 6, 0);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u16(object + 4, size);
    e.mem.set_u32(object, vtable);
}

/// A message stream over a 0x200-byte buffer, as the error reports build it on the stack
/// (constructor `00cb1600`, destructor `00cc00e0`).
struct MessageStream {
    stream: u32,
    buffer: u32,
}

fn open_message_stream(e: &mut Engine) -> MessageStream {
    let stream = e.mem.alloc(0x10);
    let buffer = e.mem.alloc(0x200);
    e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    MessageStream { stream, buffer }
}

/// `stream << text`: returns the stream the call hands back.
fn say(e: &mut Engine, stream: u32, text: u32) -> u32 {
    e.call(STREAM_WRITE_TEXT, &args![stream, text]).u32()
}

/// `stream << pointer`.
fn say_pointer(e: &mut Engine, stream: u32, value: u32) -> u32 {
    e.call(STREAM_WRITE_POINTER, &args![stream, value]).u32()
}

/// Reports the finished message to the error object (severity 1) and ends the stream.
fn report_and_close(e: &mut Engine, message: MessageStream, id: u32, line: u32) {
    let error_object = e.mem.u32(ERROR_OBJECT);
    e.vcall(
        error_object,
        0xc,
        &args![1u32, id, message.buffer, SOURCE_FILE_NAME, line],
    );
    e.call(STREAM_DESTRUCT, &args![message.stream]);
    e.mem.free(message.stream);
    e.mem.free(message.buffer);
}

/// Takes a reference on `object` when it is not null.
fn add_reference_if_set(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
}

/// Releases the reference held in a field when it is set.
fn remove_reference_if_set(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
}

/// `(object, class)` as the pair the exe returns in EAX:EDX.
fn pair_result(object: u32, class: u32) -> u64 {
    u64::from(class) << 32 | u64::from(object)
}

/// An `hkClassMemberAccessor` over this object's embedded accessor (at `+0xc`) for `name`,
/// built into a scratch local of 8 bytes; `f` gets the local.
fn with_member_accessor<R>(
    e: &mut Engine,
    owner: u32,
    name: u32,
    f: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(8, |e, accessor| {
        e.call(ACCESSOR_INIT, &args![accessor, owner + 0xc, name]);
        f(e, accessor.addr())
    })
}

// Translated from 00dbc640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00db_b3f0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dbc640(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00db_b3f0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dbb120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::newClass` (Xbox PDB), thiscall, `RET 4`: returns 0; neither `this` nor
/// the argument is read.
pub fn hk_data_world_native_new_class(
    _e: &mut Engine,
    _this: Ptr<HkDataWorldNative>,
    _unused_1: u32,
) -> u32 {
    0
}

// Translated from 00dbb130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::newArray` (Xbox PDB), thiscall, `RET 8`: returns 0; neither `this` nor
/// the two arguments are read.
pub fn hk_data_world_native_new_array(
    _e: &mut Engine,
    _this: Ptr<HkDataWorldNative>,
    _unused_1: u32,
    _unused_2: u32,
) -> u32 {
    0
}

// Translated from 00dbb140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getVariantWithMostDerivedClass` (engine map name). Takes the object in EBX (first
/// parameter here) and three stack words the caller removes: the class, the instance registry
/// (slot `0x10` answers the class of a virtual object) and the name registry (slot `0x10`
/// answers a class by name). Returns the pair `(object, class)` in EAX:EDX: both zero when the
/// class or the object is null; unchanged when the class has no vtable; otherwise the class
/// the instance registry finds for the object, looked up again by name in the name registry.
/// When either lookup fails an error is reported and the result is zero.
pub fn get_variant_with_most_derived_class(
    e: &mut Engine,
    object: u32,
    class: u32,
    instance_registry: u32,
    name_registry: u32,
) -> u64 {
    if class == 0 || object == 0 {
        return 0;
    }
    if !class_has_vtable(e, class) {
        return pair_result(object, class);
    }
    let found = e.vcall(instance_registry, 0x10, &args![object]).u32();
    if found != 0 {
        let registry_vtable = e.mem.u32(name_registry);
        let name = e.call(CLASS_GET_NAME, &args![found]).u32();
        let lookup = e.mem.u32(registry_vtable + 0x10);
        let derived = e.call(lookup, &args![name_registry, name]).u32();
        if derived != 0 {
            return pair_result(object, derived);
        }
    }
    let message = open_message_stream(e);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let stream = say(e, message.stream, TEXT_MOST_DERIVED);
    let stream = say(e, stream, name);
    let stream = say(e, stream, TEXT_AT);
    let stream = say_pointer(e, stream, object);
    say(e, stream, TEXT_REPLACED_WITH_NULL);
    report_and_close(e, message, 0x3476_d70f, 0x37);
    0
}

// Translated from 00dbb260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkNativeDataObjectImplCreate` (engine map name). Takes the world in EBX and the address of
/// an `(object, class)` pair in EDI (first and second parameters here). A non-null class is
/// looked up by name through the world's slot `0x24` (`findClass`): found, the pair's class
/// becomes the `hkClass` of the found class (`+0xc`); not found, an error is reported and the
/// pair is cleared. Returns a new 0x18-byte `hkDataObjectNative` holding a copy of the pair
/// (`hkClassAccessor`) and the world.
pub fn hk_native_data_object_impl_create(
    e: &mut Engine,
    world: Ptr<HkDataWorldNative>,
    pair: u32,
) -> u32 {
    let class = e.mem.u32(pair + 4);
    if class != 0 {
        let world_vtable = e.mem.u32(world.addr());
        let name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let find_class = e.mem.u32(world_vtable + 0x24);
        let found = e.call(find_class, &args![world, name]).u32();
        if found == 0 {
            let message = open_message_stream(e);
            let class = e.mem.u32(pair + 4);
            let name = e.call(CLASS_GET_NAME, &args![class]).u32();
            let object = e.mem.u32(pair);
            let stream = say(e, message.stream, TEXT_IGNORE_OBJECT);
            let stream = say_pointer(e, stream, object);
            let stream = say(e, stream, TEXT_CLASS_OPEN);
            let stream = say(e, stream, name);
            say(e, stream, TEXT_NOT_REGISTERED_PROVIDED);
            report_and_close(e, message, 0x1800_473a, 0x2c7);
            e.mem.set_u32(pair, 0);
            e.mem.set_u32(pair + 4, 0);
        } else {
            let found_class = e.mem.u32(found + 0xc);
            e.mem.set_u32(pair + 4, found_class);
        }
    }
    let object = allocate(e, 0x18);
    init_ref_counted(e, object, 0x18, OBJECT_VTABLE);
    e.call(CLASS_ACCESSOR_COPY, &args![object + 0xc, pair]);
    e.mem.set_u32(object + 0x14, world.addr());
    object
}

// Translated from 00dbb360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::hkDataWorldNative` (Xbox PDB), thiscall: reference count 1, the
/// vtable, both registries empty, the class map constructed (its one stack word is `this`
/// with the low byte cleared, as the exe leaves it), no contents; then asks the registry
/// singleton for the instance registry (slot `0x14`, kept at `+0xc`) and the class registry
/// (slot `0x10`, kept at `+8`), taking a reference on each and releasing what the field held.
/// Returns `this`.
pub fn hk_data_world_native_constructor(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
) -> Ptr<HkDataWorldNative> {
    let me = this.addr();
    e.set(this, HkDataWorldNative::reference_count, 1);
    e.mem.set_u32(me, WORLD_VTABLE);
    e.set(this, HkDataWorldNative::class_registry, Ptr::NULL);
    e.set(this, HkDataWorldNative::instance_registry, Ptr::NULL);
    e.call(MAP_CONSTRUCT, &args![me + 0x10, me & 0xffff_ff00]);
    e.set(this, HkDataWorldNative::contents_class, Ptr::NULL);
    e.set(this, HkDataWorldNative::contents, Ptr::NULL);
    let singleton = e.mem.u32(REGISTRY_SINGLETON);
    let registry = e.vcall(singleton, 0x14, &args![]).u32();
    add_reference_if_set(e, registry);
    let held = e.get(this, HkDataWorldNative::instance_registry).addr();
    remove_reference_if_set(e, held);
    e.set(
        this,
        HkDataWorldNative::instance_registry,
        Ptr::new(registry),
    );
    let singleton = e.mem.u32(REGISTRY_SINGLETON);
    let registry = e.vcall(singleton, 0x10, &args![]).u32();
    add_reference_if_set(e, registry);
    let held = e.get(this, HkDataWorldNative::class_registry).addr();
    remove_reference_if_set(e, held);
    e.set(this, HkDataWorldNative::class_registry, Ptr::new(registry));
    this
}

/// `hkCachedHashMap::isValid(iterator)` through its flag out-parameter.
fn map_iterator_is_valid(e: &mut Engine, map: u32, iterator: u32) -> bool {
    e.with_stack(4, |e, flag| {
        e.call(MAP_IS_VALID, &args![map, flag, iterator]);
        e.mem.u8(flag.addr()) != 0
    })
}

// Translated from 00dbb3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::~hkDataWorldNative` (Xbox PDB), thiscall: sets the world vtable, runs
/// the deleting destructor (slot 0, argument 1) of every class held in the class map, destroys
/// the map, releases and clears both registries and sets the base-object vtable.
pub fn hk_data_world_native_destructor(e: &mut Engine, this: Ptr<HkDataWorldNative>) {
    let me = this.addr();
    let map = me + 0x10;
    e.mem.set_u32(me, WORLD_VTABLE);
    let mut iterator = e.call(MAP_GET_ITERATOR, &args![map]).u32();
    let mut valid = map_iterator_is_valid(e, map, iterator);
    while valid {
        let value = e.call(MAP_GET_VALUE, &args![map, iterator]).u32();
        if value != 0 {
            e.vcall(value, 0, &args![1u32]);
        }
        iterator = e.call(MAP_GET_NEXT, &args![map, iterator]).u32();
        valid = map_iterator_is_valid(e, map, iterator);
    }
    e.call(MAP_DESTRUCT, &args![map]);
    let held = e.get(this, HkDataWorldNative::instance_registry).addr();
    remove_reference_if_set(e, held);
    e.set(this, HkDataWorldNative::instance_registry, Ptr::NULL);
    let held = e.get(this, HkDataWorldNative::class_registry).addr();
    remove_reference_if_set(e, held);
    e.set(this, HkDataWorldNative::class_registry, Ptr::NULL);
    e.mem.set_u32(me, BASE_OBJECT_VTABLE);
}

// Translated from 00dbb490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::setClassRegistry` (Xbox PDB), thiscall, `RET 4`: takes a reference on
/// the new registry (when not null), releases the one held at `+8` (when set) and stores the
/// new one.
pub fn hk_data_world_native_set_class_registry(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    registry: u32,
) {
    add_reference_if_set(e, registry);
    let held = e.get(this, HkDataWorldNative::class_registry).addr();
    remove_reference_if_set(e, held);
    e.set(this, HkDataWorldNative::class_registry, Ptr::new(registry));
}

// Translated from 00dbb4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::wrapObject` (Xbox PDB), thiscall, `RET 8`: the two arguments form an
/// `(object, class)` pair on the stack that `hkNativeDataObjectImplCreate` is run on with
/// `this` as the world; returns what it returns.
pub fn hk_data_world_native_wrap_object(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    object: u32,
    class: u32,
) -> u32 {
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), object);
        e.mem.set_u32(pair.addr() + 4, class);
        hk_native_data_object_impl_create(e, this, pair.addr())
    })
}

// Translated from 00dbb520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::findClass` (Xbox PDB), thiscall, `RET 4`: a non-null name found in the
/// class map (`getWithDefault(name, 0)`) answers that class. Otherwise (a null name never
/// searches) the class registry's slot `0x10` is asked for the `hkClass` of the name (0 for a
/// null name), a 0x14-byte `hkDataClassNative` is made over it and the world, inserted in the
/// map under the name and returned.
pub fn hk_data_world_native_find_class(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    name: u32,
) -> Ptr<HkDataClassNative> {
    let me = this.addr();
    let mut class = 0;
    if name != 0 {
        let found = e
            .call(MAP_GET_WITH_DEFAULT, &args![me + 0x10, name, 0u32])
            .u32();
        if found != 0 {
            return Ptr::new(found);
        }
        let registry = e.get(this, HkDataWorldNative::class_registry).addr();
        class = e.vcall(registry, 0x10, &args![name]).u32();
    }
    let object = allocate(e, 0x14);
    init_ref_counted(e, object, 0x14, CLASS_VTABLE);
    let created: Ptr<HkDataClassNative> = Ptr::new(object);
    e.set(created, HkDataClassNative::class, Ptr::new(class));
    e.set(created, HkDataClassNative::world, this);
    e.call(MAP_INSERT, &args![me + 0x10, name, object]);
    created
}

// Translated from 00dbb5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::findObject` (Xbox PDB), thiscall, `RET 8`: builds the `hkDataObject`
/// handle `out` over a new 0x18-byte `hkDataObjectNative` made from the `(object, class)`
/// pair at `handle` and this world. A null class is first taken from the instance registry's
/// slot `0x10` for the object. Returns `out`.
pub fn hk_data_world_native_find_object(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    out: Ptr,
    handle: u32,
) -> Ptr {
    let object = allocate(e, 0x18);
    init_ref_counted(e, object, 0x18, OBJECT_VTABLE);
    let mut class = e.mem.u32(handle + 4);
    if class == 0 {
        let registry = e.get(this, HkDataWorldNative::instance_registry).addr();
        let instance = e.mem.u32(handle);
        class = e.vcall(registry, 0x10, &args![instance]).u32();
    }
    let instance = e.mem.u32(handle);
    e.call(
        CLASS_ACCESSOR_INIT_PAIR,
        &args![object + 0xc, instance, class],
    );
    e.mem.set_u32(object + 0x14, this.addr());
    e.call(DATA_OBJECT_CONSTRUCT, &args![out, object]);
    out
}

// Translated from 00dbb610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::getType` (Xbox PDB), thiscall, `RET 4`: stores 0 at `out`.
/// Answers `out` (the `hkEnum<DataWorldType>` returned through the hidden pointer, Xbox PDB
/// `getType` returns `?AV?$hkEnum`; `EAX` is `out`, loaded at 00dbb610, at the RET 00dbb61a):
/// the tagfile reader's readClass tests the type through it (`CMP [EAX],EBX` at 00dc6ccc after
/// its world's slot +0x18 call at 00dc6cca, the world at reader +0x10).
pub fn hk_data_world_native_get_type(
    e: &mut Engine,
    _this: Ptr<HkDataWorldNative>,
    out: Ptr,
) -> Ptr {
    e.mem.set_u32(out.addr(), 0);
    out
}

// Translated from 00dbb620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::getContents` (Xbox PDB), thiscall, `RET 4`: builds the `hkDataObject`
/// handle `out` over a new 0x18-byte `hkDataObjectNative` made from the `(contents, class)`
/// pair stored at `+0x20` and this world. Returns `out`.
pub fn hk_data_world_native_get_contents(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    out: Ptr,
) -> Ptr {
    let me = this.addr();
    let object = allocate(e, 0x18);
    init_ref_counted(e, object, 0x18, OBJECT_VTABLE);
    e.call(CLASS_ACCESSOR_COPY, &args![object + 0xc, me + 0x20]);
    e.mem.set_u32(object + 0x14, me);
    e.call(DATA_OBJECT_CONSTRUCT, &args![out, object]);
    out
}

// Translated from 00dbb680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::newObject` (Xbox PDB), thiscall, `RET 4`; `class` is the
/// `hkDataClass` handle. Asks the class registry (slot `0x10`) for the `hkClass` of the class's
/// name (`hkDataClass::getName`), allocates its object size and, when that succeeded, zeroes
/// the block and runs the constructor the vtable-class registry (singleton slot `0xc`, entry
/// slot `0x1c` by name) records at `+4` of the entry (arguments `(block, 0)`, cdecl) when it
/// has one. Returns `hkNativeDataObjectImplCreate` on the pair `(block, hkClass)`.
pub fn hk_data_world_native_new_object(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    class: Ptr,
) -> u32 {
    let registry = e.get(this, HkDataWorldNative::class_registry).addr();
    let registry_vtable = e.mem.u32(registry);
    let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
    let lookup = e.mem.u32(registry_vtable + 0x10);
    let hk_class = e.call(lookup, &args![registry, name]).u32();
    let size = e.call(CLASS_GET_OBJECT_SIZE, &args![hk_class]).u32();
    let block = allocate(e, size);
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), block);
        e.mem.set_u32(pair.addr() + 4, hk_class);
        if block != 0 {
            let size = e.call(CLASS_GET_OBJECT_SIZE, &args![hk_class]).u32();
            e.call(MEMORY_SET, &args![block, 0u32, size]);
            let singleton = e.mem.u32(REGISTRY_SINGLETON);
            let vtable_registry = e.vcall(singleton, 0xc, &args![]).u32();
            let registry_vtable = e.mem.u32(vtable_registry);
            let name = e.call(DATA_CLASS_GET_NAME, &args![class]).u32();
            let entry_lookup = e.mem.u32(registry_vtable + 0x1c);
            let entry = e.call(entry_lookup, &args![vtable_registry, name]).u32();
            if entry != 0 {
                let finish = e.mem.u32(entry + 4);
                if finish != 0 {
                    e.call(finish, &args![block, 0u32]);
                }
            }
        }
        hk_native_data_object_impl_create(e, this, pair.addr())
    })
}

// Translated from 00dbb730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::setContents` (Xbox PDB), thiscall, `RET 8`: stores the object at `+0x20`
/// and finds its class (`+0x24`): a class with a vtable asks the instance registry (slot
/// `0x10`) for the object's `hkClass` and, when that exists, the class registry (slot `0x10`)
/// for its name; a class without one is looked up by its own name in the class registry. When
/// nothing is found an error is reported and the stored object is cleared.
pub fn hk_data_world_native_set_contents(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    object: u32,
    class: u32,
) {
    let me = this.addr();
    e.set(this, HkDataWorldNative::contents, Ptr::new(object));
    let found = if class_has_vtable(e, class) {
        let instance_registry = e.get(this, HkDataWorldNative::instance_registry).addr();
        let instance_vtable = e.mem.u32(instance_registry);
        let name_registry = e.get(this, HkDataWorldNative::class_registry).addr();
        let class_of_object = e.mem.u32(instance_vtable + 0x10);
        let hk_class = e
            .call(class_of_object, &args![instance_registry, object])
            .u32();
        if hk_class != 0 {
            let name_vtable = e.mem.u32(name_registry);
            let name = e.call(CLASS_GET_NAME, &args![hk_class]).u32();
            let lookup = e.mem.u32(name_vtable + 0x10);
            e.call(lookup, &args![name_registry, name]).u32()
        } else {
            0
        }
    } else {
        let name_registry = e.get(this, HkDataWorldNative::class_registry).addr();
        let name_vtable = e.mem.u32(name_registry);
        let name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let lookup = e.mem.u32(name_vtable + 0x10);
        e.call(lookup, &args![name_registry, name]).u32()
    };
    e.set(this, HkDataWorldNative::contents_class, Ptr::new(found));
    if found == 0 {
        let message = open_message_stream(e);
        let name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let stream = say(e, message.stream, TEXT_CANNOT_SET_CONTENTS);
        let stream = say_pointer(e, stream, object);
        let stream = say(e, stream, TEXT_CLASS_OPEN);
        let stream = say(e, stream, name);
        say(e, stream, TEXT_NOT_REGISTERED_IGNORE);
        report_and_close(e, message, 0x1800_473b, 0x2fc);
        e.mem.set_u32(me + 0x20, 0);
    }
}

// Translated from 00dbb840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataWorldNative::findAllClasses` (Xbox PDB), thiscall, `RET 4`; `out` is an
/// `hkArray<hkDataClass*>`. With a class registry set, asks it (slot `0x14`) to fill a local
/// array of `hkClass`es, finds (`findClass`, slot `0x24`, by name) each class whose flags do
/// not have bit 0 and appends it to `out` (growing it through `00c90940` when full), then
/// frees the local array through the allocator's slot `0x10` unless it does not own its
/// storage.
pub fn hk_data_world_native_find_all_classes(
    e: &mut Engine,
    this: Ptr<HkDataWorldNative>,
    out: u32,
) {
    let me = this.addr();
    let registry = e.get(this, HkDataWorldNative::class_registry).addr();
    if registry == 0 {
        return;
    }
    e.with_stack(12, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, 0x8000_0000);
        e.vcall(registry, 0x14, &args![local]);
        let count = e.mem.i32(local + 4);
        let mut index = 0;
        while index < count {
            let data = e.mem.u32(local);
            let hk_class = e.mem.u32(data + index as u32 * 4);
            let flags = e.call(CLASS_GET_FLAGS, &args![hk_class]).u32();
            if e.mem.u8(flags) & 1 == 0 {
                let world_vtable = e.mem.u32(me);
                let data = e.mem.u32(local);
                let hk_class = e.mem.u32(data + index as u32 * 4);
                let name = e.call(CLASS_GET_NAME, &args![hk_class]).u32();
                let find_class = e.mem.u32(world_vtable + 0x24);
                let class = e.call(find_class, &args![this, name]).u32();
                let allocator = router_allocator(e);
                if e.mem.u32(out + 4) == e.mem.u32(out + 8) & 0x3fff_ffff {
                    e.call(ARRAY_GROW, &args![allocator, out, 4u32]);
                }
                let size = e.mem.u32(out + 4);
                let slot = e.mem.u32(out).wrapping_add(size.wrapping_mul(4));
                e.mem.set_u32(slot, class);
                e.mem.set_u32(out + 4, size.wrapping_add(1));
            }
            index += 1;
        }
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(local + 8);
        e.mem.set_u32(local + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(local);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00dbb920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getName` (Xbox PDB), thiscall: `hkClass::getName` of the `hkClass` at
/// `+0xc` (the exe jumps to it).
pub fn hk_data_class_native_get_name(e: &mut Engine, this: Ptr<HkDataClassNative>) -> u32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_NAME, &args![class]).u32()
}

// Translated from 00dbb930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getVersion` (Xbox PDB), thiscall: `hkClass::getDescribedVersion` of
/// the `hkClass` at `+0xc` (the exe jumps to it).
pub fn hk_data_class_native_get_version(e: &mut Engine, this: Ptr<HkDataClassNative>) -> u32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_DESCRIBED_VERSION, &args![class]).u32()
}

// Translated from 00dbb940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getParent` (Xbox PDB), thiscall: the parent of the `hkClass`, found
/// by name through the world's slot `0x24` (`findClass`); 0 without a parent.
pub fn hk_data_class_native_get_parent(e: &mut Engine, this: Ptr<HkDataClassNative>) -> u32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    let parent = e.call(CLASS_GET_PARENT, &args![class]).u32();
    if parent == 0 {
        return 0;
    }
    let world = e.get(this, HkDataClassNative::world).addr();
    let world_vtable = e.mem.u32(world);
    let name = e.call(CLASS_GET_NAME, &args![parent]).u32();
    let find_class = e.mem.u32(world_vtable + 0x24);
    e.call(find_class, &args![world, name]).u32()
}

// Translated from 00dbb970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::isSuperClass` (Xbox PDB), thiscall, `RET 8`: walks `other` and its
/// parents (virtual slot `0x10` of each) and stores a 1 byte at `out` as soon as one has the
/// name of this class (slot `8` of each, `hkString::strCmp` equal), else a 0 byte.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hk_data_class_native_is_super_class(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    out: Ptr,
    other: u32,
) -> Ptr {
    let mut candidate = other;
    while candidate != 0 {
        let own_name = e.vcall(this.addr(), 8, &args![]).u32();
        let candidate_name = e.vcall(candidate, 8, &args![]).u32();
        let difference = e
            .call(STRING_COMPARE, &args![candidate_name, own_name])
            .u32();
        if difference == 0 {
            e.mem.set_u8(out.addr(), 1);
            return out;
        }
        candidate = e.vcall(candidate, 0x10, &args![]).u32();
    }
    e.mem.set_u8(out.addr(), 0);
    out
}

// Translated from 00dbb9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getNumDeclaredMembers` (Xbox PDB), thiscall: `hkClass::getNumDeclaredMembers`
/// of the `hkClass` at `+0xc` (the exe jumps to it).
pub fn hk_data_class_native_get_num_declared_members(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
) -> i32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_NUM_DECLARED_MEMBERS, &args![class]).i32()
}

// Translated from 00dbb9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getDeclaredMemberIndexByName` (Xbox PDB), thiscall: the exe jumps to
/// `hkClass::getDeclaredMemberIndexByName` with the `hkClass` at `+0xc`; its one stack word
/// (the name) passes through.
pub fn hk_data_class_native_get_declared_member_index_by_name(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    name: u32,
) -> i32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_DECLARED_MEMBER_INDEX_BY_NAME, &args![class, name])
        .i32()
}

// Translated from 00dbb9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getNumMembers` (Xbox PDB), thiscall: `hkClass::getNumMembers` of the
/// `hkClass` at `+0xc` (the exe jumps to it).
pub fn hk_data_class_native_get_num_members(e: &mut Engine, this: Ptr<HkDataClassNative>) -> i32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32()
}

// Translated from 00dbba00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getMemberIndexByName` (Xbox PDB), thiscall: the exe jumps to
/// `hkClass::getMemberIndexByName` with the `hkClass` at `+0xc`; its one stack word (the
/// name) passes through.
pub fn hk_data_class_native_get_member_index_by_name(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    name: u32,
) -> i32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    e.call(CLASS_GET_MEMBER_INDEX_BY_NAME, &args![class, name])
        .i32()
}

// Translated from 00dbba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable slot of the data-array and data-object classes,
/// thiscall, `RET 4`): returns the `float` 0.0 (`FLDZ`); neither `this` nor the argument is
/// read.
pub fn fn_00dbba10(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> f32 {
    0.0
}

// Translated from 00dbba20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayImpl::asHalf` (Xbox PDB), thiscall, `RET 8`: stores the 16-bit half 0 at `out`
/// (the exe converts 0.0 to its upper 16 bits); `this` and the second argument are not read.
/// Answers `out` (`hkHalf` returned through the hidden pointer, Xbox PDB `asHalf` returns
/// `?AVhkHalf`; `EAX` is `out`, loaded at 00dbba23, at the RET 00dbba34). No exe reader of
/// the slot +0x38 was found.
pub fn hk_data_array_impl_as_half(e: &mut Engine, _this: Ptr, out: Ptr, _unused_2: u32) -> Ptr {
    e.mem.set_u16(out.addr(), 0);
    out
}

// Translated from 00dbba40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a vtable slot of the data-array and data-object classes,
/// thiscall, `RET 4`): returns the 64-bit value 0 in EAX:EDX; neither `this` nor the argument
/// is read.
pub fn fn_00dbba40(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u64 {
    0
}

/// Whether the class at `class` has a vtable, through `hkClass::hasVtable`'s flag
/// out-parameter (the exe reads the byte at the pointer the call returns).
fn class_has_vtable(e: &mut Engine, class: u32) -> bool {
    e.with_stack(4, |e, flag| {
        let flag = e.call(CLASS_HAS_VTABLE, &args![class, flag]).u32();
        e.mem.u8(flag) != 0
    })
}

// Translated from 00dbba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::getHandle` (Xbox PDB), thiscall, `RET 4`: `out` receives the object
/// (`+0xc`) and its class (`+0x10`), the class being 0 when it has a vtable. Returns `out`.
pub fn hk_data_object_native_get_handle(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    out: Ptr,
) -> Ptr {
    let object = e.get(this, HkDataObjectNative::object).addr();
    let class = e.get(this, HkDataObjectNative::class).addr();
    e.mem.set_u32(out.addr(), object);
    let has_vtable = class_has_vtable(e, class);
    e.mem
        .set_u32(out.addr() + 4, if has_vtable { 0 } else { class });
    out
}

// Translated from 00dbba80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::getClass` (Xbox PDB), thiscall: 0 without a class at `+0x10`;
/// otherwise the class found by name through the world's slot `0x24` (`findClass`).
pub fn hk_data_object_native_get_class(e: &mut Engine, this: Ptr<HkDataObjectNative>) -> u32 {
    let class = e.get(this, HkDataObjectNative::class).addr();
    if class == 0 {
        return 0;
    }
    let world = e.get(this, HkDataObjectNative::world).addr();
    let world_vtable = e.mem.u32(world);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let find_class = e.mem.u32(world_vtable + 0x24);
    e.call(find_class, &args![world, name]).u32()
}

// Translated from 00dbbab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::accessByName` (Xbox PDB), thiscall, `RET 8`: builds an
/// `hkClassMemberAccessor` for the name over the embedded accessor (`+0xc`) into a local the
/// exe then drops, and stores `(this, name)` at `out`. Returns `out`.
pub fn hk_data_object_native_access_by_name(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    out: Ptr,
    name: u32,
) -> Ptr {
    with_member_accessor(e, this.addr(), name, |_, _| {});
    e.mem.set_u32(out.addr() + 4, name);
    e.mem.set_u32(out.addr(), this.addr());
    out
}

// Translated from 00dbbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::hasMember` (Xbox PDB), thiscall, `RET 4`: whether an
/// `hkClassMemberAccessor` for the name over the embedded accessor is ok (`isOk`).
pub fn hk_data_object_native_has_member(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> bool {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        e.with_stack(4, |e, flag| {
            let flag = e.call(ACCESSOR_IS_OK, &args![accessor, flag]).u32();
            e.mem.u8(flag) != 0
        })
    })
}

// Translated from 00dbbb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::isValid` (Xbox PDB), thiscall, `RET 4`: whether `index` (signed) is
/// below the number of members of the class at `+0x10`.
pub fn hk_data_object_native_is_valid(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    index: i32,
) -> bool {
    let class = e.get(this, HkDataObjectNative::class).addr();
    let count = e.call(CLASS_GET_NUM_MEMBERS, &args![class]).i32();
    index < count
}

// Translated from 00dbbb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::getMemberValue` (Xbox PDB), thiscall, `RET 8`: asks its own slot
/// `0x20` (`getMemberName`) for the member `index` and stores `(this, result)` at `out`.
/// Returns `out`.
pub fn hk_data_object_native_get_member_value(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    out: Ptr,
    index: u32,
) -> Ptr {
    let value = e.vcall(this.addr(), 0x20, &args![index]).u32();
    e.mem.set_u32(out.addr(), this.addr());
    e.mem.set_u32(out.addr() + 4, value);
    out
}

// Translated from 00dbbb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asVec` (Xbox PDB), thiscall, `RET 8` (the second argument is never
/// read): the address of the member `name` (`hkClassMemberAccessor::getAddress`).
pub fn hk_data_object_native_as_vec(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
    _unused_2: u32,
) -> u32 {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32()
    })
}

// Translated from 00dbbb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::hkDataArrayNative` (Xbox PDB), thiscall, `RET 0x1c`: stores the world
/// (`+0xc`), data (`+0x10`), count (`+0x14`), stride (`+0x18`), member type (`+0x1c`), class
/// (`+0x20`) and tuple size (`+0x28`), the counts cleared and the vtable set, then
/// `+0x24` = `getTypeFromMemberType(0x16, type, class, tuple_size) - 0x10` when that is above
/// `0x10`, else `0x10` (signed compare). Returns `this`.
#[allow(clippy::too_many_arguments)] // the exe's own signature: this and seven stack words
pub fn hk_data_array_native_constructor(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    world: Ptr<HkDataWorldNative>,
    data: Ptr,
    count: u32,
    stride: u32,
    member_type: u32,
    class: Ptr,
    tuple_size: u32,
) -> Ptr<HkDataArrayNative> {
    e.set(this, HkDataArrayNative::world, world);
    e.set(this, HkDataArrayNative::external_count, 0);
    e.set(this, HkDataArrayNative::reference_count, 0);
    e.set(this, HkDataArrayNative::data, data);
    e.set(this, HkDataArrayNative::stride, stride);
    e.set(this, HkDataArrayNative::count, count);
    e.mem.set_u32(this.addr(), ARRAY_VTABLE);
    e.set(this, HkDataArrayNative::member_type, member_type);
    e.set(this, HkDataArrayNative::class, class);
    e.set(this, HkDataArrayNative::tuple_size, tuple_size);
    let code = e
        .call(
            TYPE_FROM_MEMBER_TYPE,
            &args![0x16u32, member_type, class, tuple_size],
        )
        .i32();
    let element_type = if code > 0x10 { code - 0x10 } else { 0x10 };
    e.set(this, HkDataArrayNative::element_type, element_type as u32);
    this
}

// Translated from 00dbbc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (shared code, a vtable slot of the array classes), thiscall:
/// the word at `this + 0x24` (for `hkDataArrayNative` the element type code).
pub fn fn_00dbbc00(e: &mut Engine, this: Ptr<HkDataArrayNative>) -> u32 {
    e.get(this, HkDataArrayNative::element_type)
}

// Translated from 00dbbc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (shared code, a vtable slot of several classes), thiscall: the
/// word at `this + 0x14` (for `hkDataArrayNative` the element count).
pub fn fn_00dbbc10(e: &mut Engine, this: Ptr<HkDataArrayNative>) -> u32 {
    e.get(this, HkDataArrayNative::count)
}

// Translated from 00dbbc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::getClass` (Xbox PDB), thiscall: 0 without a class at `+0x20`;
/// otherwise the class found by name through the world's slot `0x24` (`findClass`).
pub fn hk_data_array_native_get_class(e: &mut Engine, this: Ptr<HkDataArrayNative>) -> u32 {
    let class = e.get(this, HkDataArrayNative::class).addr();
    if class == 0 {
        return 0;
    }
    let world = e.get(this, HkDataArrayNative::world).addr();
    let world_vtable = e.mem.u32(world);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let find_class = e.mem.u32(world_vtable + 0x24);
    e.call(find_class, &args![world, name]).u32()
}

/// The address of element `index` (`stride * index + data`, wrapping).
fn element_address(e: &Engine, this: Ptr<HkDataArrayNative>, index: u32) -> u32 {
    let stride = e.get(this, HkDataArrayNative::stride);
    let data = e.get(this, HkDataArrayNative::data).addr();
    stride.wrapping_mul(index).wrapping_add(data)
}

// Translated from 00dbbc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asVec` (Xbox PDB), thiscall, `RET 4`: the address of element `index`
/// (`stride * index + data`).
pub fn hk_data_array_native_as_vec(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> u32 {
    element_address(e, this, index)
}

// Translated from 00dbbc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asString` (Xbox PDB), thiscall, `RET 4`: for member type 0x1d
/// (C string) the pointer stored in element `index`, for 0x21 (string pointer) that pointer
/// with bit 0 cleared, otherwise 0.
pub fn hk_data_array_native_as_string(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> u32 {
    let member_type = e.get(this, HkDataArrayNative::member_type);
    if member_type == 0x1d {
        let address = element_address(e, this, index);
        e.mem.u32(address)
    } else if member_type == 0x21 {
        let address = element_address(e, this, index);
        e.mem.u32(address) & 0xffff_fffe
    } else {
        0
    }
}

// Translated from 00dbbca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asReal` (Xbox PDB), thiscall, `RET 4`, result in ST0: member type 0xb
/// (`float`) the element loaded as a `float` (a signalling NaN is quieted), 0x20 (half) the
/// 16-bit element shifted into the upper half of a `float`, otherwise 0.0.
pub fn hk_data_array_native_as_real(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> f32 {
    let member_type = e.get(this, HkDataArrayNative::member_type);
    if member_type == 0xb {
        let address = element_address(e, this, index);
        f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
    } else if member_type == 0x20 {
        let address = element_address(e, this, index);
        let half = i32::from(e.mem.i16(address));
        f32::from_bits(x87_quiet_bits((half << 16) as u32))
    } else {
        0.0
    }
}

// Translated from 00dbbcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asInt` (Xbox PDB), thiscall, `RET 4`: element `index` read as the
/// integer its member type says: types 1 to 3 signed byte, 4 unsigned byte, 5 signed
/// 16-bit, 6 unsigned 16-bit, 7 to 10 and 0x1e a 32-bit word; anything else 0.
pub fn hk_data_array_native_as_int(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> u32 {
    let address = element_address(e, this, index);
    match e.get(this, HkDataArrayNative::member_type) {
        1..=3 => i32::from(e.mem.i8(address)) as u32,
        4 => u32::from(e.mem.u8(address)),
        5 => i32::from(e.mem.i16(address)) as u32,
        6 => u32::from(e.mem.u16(address)),
        7..=10 | 0x1e => e.mem.u32(address),
        _ => 0,
    }
}

// Translated from 00dbbd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asInt64` (Xbox PDB), thiscall, `RET 4`: the 64-bit word at element
/// `index`, returned in EAX:EDX.
pub fn hk_data_array_native_as_int64(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> u64 {
    let address = element_address(e, this, index);
    e.mem.u64(address)
}

// Translated from 00dbbd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asArray` (Xbox PDB), thiscall, `RET 4`: a new 0x2c-byte
/// `hkDataArrayNative` for the element `index`. With a tuple size (`+0x28`) it views the
/// element's own storage: the element size is the 16-bit size of the member type's properties
/// (`getClassMemberTypeProperties`, sign-extended), the count is the tuple size and the type,
/// class and world are this array's. Without one the element is an array header and the new
/// array takes its data pointer (`+0`) and count (`+4`) with no stride, type or class.
/// Returns the new array.
pub fn hk_data_array_native_as_array(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> Ptr<HkDataArrayNative> {
    let element = element_address(e, this, index);
    let tuple_size = e.get(this, HkDataArrayNative::tuple_size);
    let world = e.get(this, HkDataArrayNative::world);
    if tuple_size != 0 {
        let member_type = e.get(this, HkDataArrayNative::member_type);
        let properties = e.call(MEMBER_TYPE_PROPERTIES, &args![member_type]).u32();
        let size = i32::from(e.mem.i16(properties + 8)) as u32;
        let array = allocate(e, 0x2c);
        e.mem.set_u16(array + 4, 0x2c);
        let member_type = e.get(this, HkDataArrayNative::member_type);
        let class = e.get(this, HkDataArrayNative::class);
        hk_data_array_native_constructor(
            e,
            Ptr::new(array),
            world,
            Ptr::new(element),
            tuple_size,
            size,
            member_type,
            class,
            0,
        )
    } else {
        let array = allocate(e, 0x2c);
        e.mem.set_u16(array + 4, 0x2c);
        let data = e.mem.u32(element);
        let count = e.mem.u32(element + 4);
        hk_data_array_native_constructor(
            e,
            Ptr::new(array),
            world,
            Ptr::new(data),
            count,
            0,
            0,
            Ptr::NULL,
            0,
        )
    }
}

// Translated from 00dbbe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::getMemberName` (Xbox PDB), thiscall, `RET 4`: the name word (the first
/// word) of member `index` of the class at `+0x10` (`hkClass::getMember`).
pub fn hk_data_object_native_get_member_name(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    index: u32,
) -> u32 {
    let class = e.get(this, HkDataObjectNative::class).addr();
    let member = e.call(CLASS_GET_MEMBER, &args![class, index]).u32();
    e.mem.u32(member)
}

// Translated from 00dbbe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataRefCounted::_scalar_deleting_destructor_` (Xbox PDB), thiscall, `RET 4`: sets the
/// `hkDataRefCounted` vtable and, when bit 0 of `flags` is set, returns the block to the
/// memory router's allocator (slot `8`, `(this, 16-bit size at +4)`). Returns `this`.
pub fn hk_data_ref_counted_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), REF_COUNTED_VTABLE);
    if flags & 1 != 0 {
        let allocator = router_allocator(e);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dbbe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::fillMemberInfo` (Xbox PDB), thiscall, `RET 8`: fills the five-word
/// record `out` for the `hkClassMember` `member`: its name, this class, the type code
/// (`getTypeFromMemberType(type, subtype, class, tuple_size)`), the class it refers to and the
/// tuple size. A member whose flags have bit `0x400` set gets zeros for the last three. The
/// referring class is found by name through the world's slot `0x24`: the name comes from the
/// member's `hk.DataObjectType` attribute (`typeName`) when it has one, else from the member's
/// own class; a member with no class gets 0. Returns what EAX holds at the end: the tuple
/// size, or 0 for the flagged member.
pub fn hk_data_class_native_fill_member_info(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    member: u32,
    out: u32,
) -> u32 {
    let name = e.mem.u32(member);
    e.mem.set_u32(out, name);
    e.mem.set_u32(out + 4, this.addr());
    if e.mem.u16(member + 0x10) & 0x400 == 0x400 {
        e.mem.set_u32(out + 8, 0);
        e.mem.set_u32(out + 0xc, 0);
        e.mem.set_u32(out + 0x10, 0);
        return 0;
    }
    let member_type = u32::from(e.mem.u8(member + 0xc));
    let subtype = u32::from(e.mem.u8(member + 0xd));
    let tuple_size = e.call(MEMBER_GET_ARRAY_SIZE, &args![member]).u32();
    let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
    let code = e
        .call(
            TYPE_FROM_MEMBER_TYPE,
            &args![member_type, subtype, class, tuple_size],
        )
        .u32();
    e.mem.set_u32(out + 8, code);
    let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
    let world = e.get(this, HkDataClassNative::world).addr();
    if class == 0 {
        e.mem.set_u32(out + 0xc, 0);
        let tuple_size = e.call(MEMBER_GET_ARRAY_SIZE, &args![member]).u32();
        e.mem.set_u32(out + 0x10, tuple_size);
        return tuple_size;
    }
    let attribute_name = e.mem.u32(DATA_OBJECT_TYPE_ATTRIBUTE);
    let attribute = e
        .call(MEMBER_GET_ATTRIBUTE, &args![member, attribute_name])
        .u32();
    let found = if attribute != 0 {
        e.with_stack(8, |e, accessor| {
            e.call(ACCESSOR_INIT, &args![accessor, attribute, TYPE_NAME_TEXT]);
            let world_vtable = e.mem.u32(world);
            let type_name = e.call(ACCESSOR_AS_CSTRING, &args![accessor, 0u32]).u32();
            let type_name = e.mem.u32(type_name);
            let find_class = e.mem.u32(world_vtable + 0x24);
            e.call(find_class, &args![world, type_name]).u32()
        })
    } else {
        let world_vtable = e.mem.u32(world);
        let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
        let name = e.call(CLASS_GET_NAME, &args![class]).u32();
        let find_class = e.mem.u32(world_vtable + 0x24);
        e.call(find_class, &args![world, name]).u32()
    };
    e.mem.set_u32(out + 0xc, found);
    let tuple_size = e.call(MEMBER_GET_ARRAY_SIZE, &args![member]).u32();
    e.mem.set_u32(out + 0x10, tuple_size);
    tuple_size
}

// Translated from 00dbbfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getDeclaredMemberInfo` (Xbox PDB), thiscall, `RET 8`: `fillMemberInfo`
/// of declared member `index` of the class at `+0xc` into `out`; returns what it returns.
pub fn hk_data_class_native_get_declared_member_info(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    index: u32,
    out: u32,
) -> u32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    let member = e
        .call(CLASS_GET_DECLARED_MEMBER, &args![class, index])
        .u32();
    hk_data_class_native_fill_member_info(e, this, member, out)
}

// Translated from 00dbbfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataClassNative::getMemberInfo` (Xbox PDB), thiscall, `RET 8`: `fillMemberInfo` of
/// member `index` of the class at `+0xc` into `out`; returns what it returns.
pub fn hk_data_class_native_get_member_info(
    e: &mut Engine,
    this: Ptr<HkDataClassNative>,
    index: u32,
    out: u32,
) -> u32 {
    let class = e.get(this, HkDataClassNative::class).addr();
    let member = e.call(CLASS_GET_MEMBER, &args![class, index]).u32();
    hk_data_class_native_fill_member_info(e, this, member, out)
}

// Translated from 00dbc000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::asObject` (Xbox PDB), thiscall, `RET 4`: for member type 0x14
/// (pointer) the object is the pointer stored in element `index`, for 0x19 (struct) the
/// element's address, both run through `getVariantWithMostDerivedClass` with this array's
/// class and the world's two registries; for 0x1c (variant) the element's `(object, class)`
/// pair. Any other type, or a null object or class, gives 0; otherwise
/// `hkNativeDataObjectImplCreate` on the pair with this array's world.
pub fn hk_data_array_native_as_object(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    index: u32,
) -> u32 {
    let world = e.get(this, HkDataArrayNative::world);
    let member_type = e.get(this, HkDataArrayNative::member_type);
    let (object, class) = match member_type {
        0x14 => {
            let name_registry = e.mem.u32(world.addr() + 8);
            let instance_registry = e.mem.u32(world.addr() + 0xc);
            let class = e.get(this, HkDataArrayNative::class).addr();
            let address = element_address(e, this, index);
            let object = e.mem.u32(address);
            let found = get_variant_with_most_derived_class(
                e,
                object,
                class,
                instance_registry,
                name_registry,
            );
            (found as u32, (found >> 32) as u32)
        }
        0x19 => {
            let name_registry = e.mem.u32(world.addr() + 8);
            let instance_registry = e.mem.u32(world.addr() + 0xc);
            let class = e.get(this, HkDataArrayNative::class).addr();
            let object = element_address(e, this, index);
            let found = get_variant_with_most_derived_class(
                e,
                object,
                class,
                instance_registry,
                name_registry,
            );
            (found as u32, (found >> 32) as u32)
        }
        0x1c => {
            let address = element_address(e, this, index);
            (e.mem.u32(address), e.mem.u32(address + 4))
        }
        _ => return 0,
    };
    if object == 0 || class == 0 {
        return 0;
    }
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), object);
        e.mem.set_u32(pair.addr() + 4, class);
        hk_native_data_object_impl_create(e, world, pair.addr())
    })
}

// Translated from 00dbc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataArrayNative::swizzleObjectMember` (Xbox PDB), thiscall, `RET 4`: a new 0x2c-byte
/// `hkDataArrayNative` over the member `name` of this array's class (`hkClass::getMemberByName`)
/// in every element: data at the member's 16-bit offset (`+0x12`) from this array's data,
/// this array's count and stride, the member's type (its subtype for types 0x18 and 0x1f), its
/// class and its tuple size. Returns the new array.
pub fn hk_data_array_native_swizzle_object_member(
    e: &mut Engine,
    this: Ptr<HkDataArrayNative>,
    name: u32,
) -> Ptr<HkDataArrayNative> {
    let class = e.get(this, HkDataArrayNative::class).addr();
    let member = e.call(CLASS_GET_MEMBER_BY_NAME, &args![class, name]).u32();
    let offset = u32::from(e.mem.u16(member + 0x12));
    let member_type = e.mem.u8(member + 0xc);
    let data = offset.wrapping_add(e.get(this, HkDataArrayNative::data).addr());
    let element_type = if member_type == 0x18 || member_type == 0x1f {
        u32::from(e.mem.u8(member + 0xd))
    } else {
        u32::from(member_type)
    };
    let array = allocate(e, 0x2c);
    e.mem.set_u16(array + 4, 0x2c);
    let tuple_size = e.call(MEMBER_GET_ARRAY_SIZE, &args![member]).u32();
    let member_class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
    let world = e.get(this, HkDataArrayNative::world);
    let count = e.get(this, HkDataArrayNative::count);
    let stride = e.get(this, HkDataArrayNative::stride);
    hk_data_array_native_constructor(
        e,
        Ptr::new(array),
        world,
        Ptr::new(data),
        count,
        stride,
        element_type,
        Ptr::new(member_class),
        tuple_size,
    )
}

/// A new 0x2c-byte `hkDataArrayNative` made the way `hkDataObjectNative::asArray` ends:
/// allocated, its size word set, then constructed over the object's world.
fn new_object_array(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    data: u32,
    count: u32,
    stride: u32,
    member_type: u32,
    class: u32,
) -> Ptr<HkDataArrayNative> {
    let array = allocate(e, 0x2c);
    e.mem.set_u16(array + 4, 0x2c);
    let world = e.get(this, HkDataObjectNative::world);
    hk_data_array_native_constructor(
        e,
        Ptr::new(array),
        world,
        Ptr::new(data),
        count,
        stride,
        member_type,
        Ptr::new(class),
        0,
    )
}

// Translated from 00dbc150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asArray` (Xbox PDB), thiscall, `RET 4`: a new `hkDataArrayNative` over
/// the member `name`, by the member's type: types 1 to 0x12, 0x14, 0x19, 0x1e and 0x20 view the
/// member's own address with the member's tuple size as the count (the element size is the
/// 16-bit size of the type's properties, or the class's object size for 0x19); types 0x16 and
/// 0x1a (array, simple array) view the `(data, count)` header stored there with the subtype
/// (element size from its properties, or the class's object size when that is `-1`; a zero
/// subtype gives an empty array); type 0x1b (homogeneous array) views `(count at +8, data at
/// +4)` of the class pointer at `+0` as structs; every other type gives an empty array.
/// Returns the new array.
pub fn hk_data_object_native_as_array(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> Ptr<HkDataArrayNative> {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        let member = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
        let member_type = e.mem.u8(member + 0xc);
        let (data, count, stride, element_type, class) = match member_type {
            1..=0x12 | 0x14 | 0x19 | 0x1e | 0x20 => {
                let data = e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32();
                let again = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
                let count = e.call(MEMBER_GET_ARRAY_SIZE, &args![again]).u32();
                let member_type = u32::from(e.mem.u8(member + 0xc));
                let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                let stride = if member_type == 0x19 {
                    e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32()
                } else {
                    let properties = e.call(MEMBER_TYPE_PROPERTIES, &args![member_type]).u32();
                    i32::from(e.mem.i16(properties + 8)) as u32
                };
                (data, count, stride, member_type, class)
            }
            0x16 | 0x1a => {
                let header = e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32();
                let data = e.mem.u32(header);
                let count = e.mem.u32(header + 4);
                let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                let subtype = u32::from(e.mem.u8(member + 0xd));
                let properties = e.call(MEMBER_TYPE_PROPERTIES, &args![subtype]).u32();
                let size = i32::from(e.mem.i16(properties + 8)) as u32;
                if subtype == 0 {
                    return new_object_array(e, this, 0, 0, 0, 0, 0);
                }
                let stride = if size == 0xffff_ffff {
                    e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32()
                } else {
                    size
                };
                (data, count, stride, subtype, class)
            }
            0x1b => {
                let header = e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32();
                let count = e.mem.u32(header + 8);
                let data = e.mem.u32(header + 4);
                let header = e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32();
                let class = e.mem.u32(header);
                if class != 0 {
                    let stride = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).u32();
                    (data, count, stride, 0x19, class)
                } else {
                    (data, count, 0, 0, class)
                }
            }
            _ => (0, 0, 0, 0, 0),
        };
        new_object_array(e, this, data, count, stride, element_type, class)
    })
}

// Translated from 00dbc310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asString` (Xbox PDB), thiscall, `RET 4`: for a member of type 0x1d
/// (C string) the string pointer (`asCstring`), for 0x21 (string pointer) that pointer with
/// bit 0 cleared (`asStringPtr`), otherwise 0.
pub fn hk_data_object_native_as_string(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> u32 {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        let member = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
        let member_type = e.mem.u8(member + 0xc);
        if member_type == 0x1d {
            let address = e.call(ACCESSOR_AS_CSTRING, &args![accessor, 0u32]).u32();
            e.mem.u32(address)
        } else if member_type == 0x21 {
            let address = e.call(ACCESSOR_AS_STRING_PTR, &args![accessor, 0u32]).u32();
            e.mem.u32(address) & 0xffff_fffe
        } else {
            0
        }
    })
}

// Translated from 00dbc370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asInt` (Xbox PDB), thiscall, `RET 4`, 64-bit result in EAX:EDX: the
/// member `name` read as the integer its type says, through the accessor's typed address
/// function: types 1 to 3 signed byte, 4 unsigned byte, 5 signed 16-bit, 6 unsigned 16-bit,
/// 7 signed 32-bit (sign-extended), 8 unsigned 32-bit, 9 and 10 a 64-bit word, 0x1e an
/// unsigned 32-bit word. Types 0x18 and 0x1f (enum, flags) are read as their subtype (`+0xd`,
/// retried in the same way: as in the exe, a subtype that is itself 0x18 or 0x1f never
/// ends). Anything else gives 0.
pub fn hk_data_object_native_as_int(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> u64 {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        let member = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
        let mut member_type = u32::from(e.mem.u8(member + 0xc));
        loop {
            match member_type {
                1..=3 => {
                    let address = e.call(ACCESSOR_ADDRESS_I8, &args![accessor, 0u32]).u32();
                    return i64::from(e.mem.i8(address)) as u64;
                }
                4 => {
                    let address = e.call(ACCESSOR_ADDRESS_U8, &args![accessor, 0u32]).u32();
                    return u64::from(e.mem.u8(address));
                }
                5 => {
                    let address = e.call(ACCESSOR_ADDRESS_I16, &args![accessor, 0u32]).u32();
                    return i64::from(e.mem.i16(address)) as u64;
                }
                6 => {
                    let address = e.call(ACCESSOR_ADDRESS_U16, &args![accessor, 0u32]).u32();
                    return u64::from(e.mem.u16(address));
                }
                7 => {
                    let address = e.call(ACCESSOR_ADDRESS_I32, &args![accessor, 0u32]).u32();
                    return i64::from(e.mem.i32(address)) as u64;
                }
                8 => {
                    let address = e.call(ACCESSOR_ADDRESS_U32, &args![accessor, 0u32]).u32();
                    return u64::from(e.mem.u32(address));
                }
                9 => {
                    let address = e.call(ACCESSOR_ADDRESS_I64, &args![accessor, 0u32]).u32();
                    return e.mem.u64(address);
                }
                10 => {
                    let address = e.call(ACCESSOR_ADDRESS_U64, &args![accessor, 0u32]).u32();
                    return e.mem.u64(address);
                }
                0x1e => {
                    let address = e.call(ACCESSOR_ADDRESS_ULONG, &args![accessor, 0u32]).u32();
                    return u64::from(e.mem.u32(address));
                }
                0x18 | 0x1f => member_type = u32::from(e.mem.u8(member + 0xd)),
                _ => return 0,
            }
        }
    })
}

// Translated from 00dbc4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asObject` (Xbox PDB), thiscall, `RET 4`: for member type 0x14
/// (pointer) the pointer stored in the member (`asPointer`), for 0x19 (struct) the member's
/// address, both run through `getVariantWithMostDerivedClass` with the member's class and the
/// world's two registries; for 0x1c (variant) the `(object, class)` pair `asVariant` gives.
/// Any other type, or a null object or class, gives 0; otherwise
/// `hkNativeDataObjectImplCreate` on the pair with this object's world.
pub fn hk_data_object_native_as_object(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> u32 {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        let member = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
        let member_type = e.mem.u8(member + 0xc);
        let world = e.get(this, HkDataObjectNative::world);
        let (object, class) = match member_type {
            0x14 => {
                let name_registry = e.mem.u32(world.addr() + 8);
                let instance_registry = e.mem.u32(world.addr() + 0xc);
                let pointer = e.call(ACCESSOR_AS_POINTER, &args![accessor, 0u32]).u32();
                let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                let object = e.mem.u32(pointer);
                let found = get_variant_with_most_derived_class(
                    e,
                    object,
                    class,
                    instance_registry,
                    name_registry,
                );
                (found as u32, (found >> 32) as u32)
            }
            0x19 => {
                let name_registry = e.mem.u32(world.addr() + 8);
                let instance_registry = e.mem.u32(world.addr() + 0xc);
                let object = e.call(ACCESSOR_GET_ADDRESS, &args![accessor]).u32();
                let class = e.call(MEMBER_GET_CLASS, &args![member]).u32();
                let found = get_variant_with_most_derived_class(
                    e,
                    object,
                    class,
                    instance_registry,
                    name_registry,
                );
                (found as u32, (found >> 32) as u32)
            }
            0x1c => {
                let variant = e.call(ACCESSOR_AS_VARIANT, &args![accessor, 0u32]).u32();
                (e.mem.u32(variant), e.mem.u32(variant + 4))
            }
            _ => return 0,
        };
        if object == 0 || class == 0 {
            return 0;
        }
        e.with_stack(8, |e, pair| {
            e.mem.set_u32(pair.addr(), object);
            e.mem.set_u32(pair.addr() + 4, class);
            hk_native_data_object_impl_create(e, world, pair.addr())
        })
    })
}

// Translated from 00dbc5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::asReal` (Xbox PDB), thiscall, `RET 4`, result in ST0: for a member of
/// type 0xb (`float`) the value at the accessor's `float` address (a signalling NaN is
/// quieted), for 0x20 (half) the 16-bit value shifted into the upper half of a `float`,
/// otherwise 0.0.
pub fn hk_data_object_native_as_real(
    e: &mut Engine,
    this: Ptr<HkDataObjectNative>,
    name: u32,
) -> f32 {
    with_member_accessor(e, this.addr(), name, |e, accessor| {
        let member = e.call(ACCESSOR_GET_CLASS_MEMBER, &args![accessor]).u32();
        let member_type = e.mem.u8(member + 0xc);
        if member_type == 0xb {
            let address = e.call(ACCESSOR_ADDRESS_FLOAT, &args![accessor, 0u32]).u32();
            f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
        } else if member_type == 0x20 {
            let address = e.call(ACCESSOR_ADDRESS_HALF, &args![accessor, 0u32]).u32();
            let half = i32::from(e.mem.i16(address));
            f32::from_bits(x87_quiet_bits((half << 16) as u32))
        } else {
            0.0
        }
    })
}

// Translated from 00dc9520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDataObjectNative::getNextMember` (Xbox PDB), thiscall, `RET 4`: `index + 1`; `this` is
/// not read.
pub fn hk_data_object_native_get_next_member(
    _e: &mut Engine,
    _this: Ptr<HkDataObjectNative>,
    index: i32,
) -> i32 {
    index.wrapping_add(1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00db_b120,
            hk_data_world_native_new_class(Ptr<HkDataWorldNative>, u32) -> u32
        ),
        entry!(
            0x00db_b130,
            hk_data_world_native_new_array(Ptr<HkDataWorldNative>, u32, u32) -> u32
        ),
        entry!(0x00db_b140, get_variant_with_most_derived_class(u32, u32, u32, u32) -> u64),
        entry!(
            0x00db_b260,
            hk_native_data_object_impl_create(Ptr<HkDataWorldNative>, u32) -> u32
        ),
        entry!(
            0x00db_b360,
            hk_data_world_native_constructor(Ptr<HkDataWorldNative>) -> Ptr<HkDataWorldNative>
        ),
        entry!(
            0x00db_b3f0,
            hk_data_world_native_destructor(Ptr<HkDataWorldNative>)
        ),
        entry!(
            0x00db_b490,
            hk_data_world_native_set_class_registry(Ptr<HkDataWorldNative>, u32)
        ),
        entry!(
            0x00db_b4f0,
            hk_data_world_native_wrap_object(Ptr<HkDataWorldNative>, u32, u32) -> u32
        ),
        entry!(
            0x00db_b520,
            hk_data_world_native_find_class(Ptr<HkDataWorldNative>, u32) -> Ptr<HkDataClassNative>
        ),
        entry!(
            0x00db_b5a0,
            hk_data_world_native_find_object(Ptr<HkDataWorldNative>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00db_b610,
            hk_data_world_native_get_type(Ptr<HkDataWorldNative>, Ptr) -> Ptr
        ),
        entry!(
            0x00db_b620,
            hk_data_world_native_get_contents(Ptr<HkDataWorldNative>, Ptr) -> Ptr
        ),
        entry!(
            0x00db_b680,
            hk_data_world_native_new_object(Ptr<HkDataWorldNative>, Ptr) -> u32
        ),
        entry!(
            0x00db_b730,
            hk_data_world_native_set_contents(Ptr<HkDataWorldNative>, u32, u32)
        ),
        entry!(
            0x00db_b840,
            hk_data_world_native_find_all_classes(Ptr<HkDataWorldNative>, u32)
        ),
        entry!(
            0x00db_b920,
            hk_data_class_native_get_name(Ptr<HkDataClassNative>) -> u32
        ),
        entry!(
            0x00db_b930,
            hk_data_class_native_get_version(Ptr<HkDataClassNative>) -> u32
        ),
        entry!(
            0x00db_b940,
            hk_data_class_native_get_parent(Ptr<HkDataClassNative>) -> u32
        ),
        entry!(
            0x00db_b970,
            hk_data_class_native_is_super_class(Ptr<HkDataClassNative>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00db_b9d0,
            hk_data_class_native_get_num_declared_members(Ptr<HkDataClassNative>) -> i32
        ),
        entry!(
            0x00db_b9e0,
            hk_data_class_native_get_declared_member_index_by_name(
                Ptr<HkDataClassNative>,
                u32,
            ) -> i32
        ),
        entry!(
            0x00db_b9f0,
            hk_data_class_native_get_num_members(Ptr<HkDataClassNative>) -> i32
        ),
        entry!(
            0x00db_ba00,
            hk_data_class_native_get_member_index_by_name(Ptr<HkDataClassNative>, u32) -> i32
        ),
        entry!(0x00db_ba10, fn_00dbba10(Ptr, u32) -> f32),
        entry!(0x00db_ba20, hk_data_array_impl_as_half(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00db_ba40, fn_00dbba40(Ptr, u32) -> u64),
        entry!(
            0x00db_ba50,
            hk_data_object_native_get_handle(Ptr<HkDataObjectNative>, Ptr) -> Ptr
        ),
        entry!(
            0x00db_ba80,
            hk_data_object_native_get_class(Ptr<HkDataObjectNative>) -> u32
        ),
        entry!(
            0x00db_bab0,
            hk_data_object_native_access_by_name(Ptr<HkDataObjectNative>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00db_bae0,
            hk_data_object_native_has_member(Ptr<HkDataObjectNative>, u32) -> bool
        ),
        entry!(
            0x00db_bb20,
            hk_data_object_native_is_valid(Ptr<HkDataObjectNative>, i32) -> bool
        ),
        entry!(
            0x00db_bb40,
            hk_data_object_native_get_member_value(Ptr<HkDataObjectNative>, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00db_bb60,
            hk_data_object_native_as_vec(Ptr<HkDataObjectNative>, u32, u32) -> u32
        ),
        entry!(
            0x00db_bb90,
            hk_data_array_native_constructor(
                Ptr<HkDataArrayNative>,
                Ptr<HkDataWorldNative>,
                Ptr,
                u32,
                u32,
                u32,
                Ptr,
                u32,
            ) -> Ptr<HkDataArrayNative>
        ),
        entry!(0x00db_bc00, fn_00dbbc00(Ptr<HkDataArrayNative>) -> u32),
        entry!(0x00db_bc10, fn_00dbbc10(Ptr<HkDataArrayNative>) -> u32),
        entry!(
            0x00db_bc20,
            hk_data_array_native_get_class(Ptr<HkDataArrayNative>) -> u32
        ),
        entry!(
            0x00db_bc50,
            hk_data_array_native_as_vec(Ptr<HkDataArrayNative>, u32) -> u32
        ),
        entry!(
            0x00db_bc60,
            hk_data_array_native_as_string(Ptr<HkDataArrayNative>, u32) -> u32
        ),
        entry!(
            0x00db_bca0,
            hk_data_array_native_as_real(Ptr<HkDataArrayNative>, u32) -> f32
        ),
        entry!(
            0x00db_bcf0,
            hk_data_array_native_as_int(Ptr<HkDataArrayNative>, u32) -> u32
        ),
        entry!(
            0x00db_bd70,
            hk_data_array_native_as_int64(Ptr<HkDataArrayNative>, u32) -> u64
        ),
        entry!(
            0x00db_bd90,
            hk_data_array_native_as_array(Ptr<HkDataArrayNative>, u32) -> Ptr<HkDataArrayNative>
        ),
        entry!(
            0x00db_be30,
            hk_data_object_native_get_member_name(Ptr<HkDataObjectNative>, u32) -> u32
        ),
        entry!(0x00db_be50, hk_data_ref_counted_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(
            0x00db_be80,
            hk_data_class_native_fill_member_info(Ptr<HkDataClassNative>, u32, u32) -> u32
        ),
        entry!(
            0x00db_bfa0,
            hk_data_class_native_get_declared_member_info(Ptr<HkDataClassNative>, u32, u32) -> u32
        ),
        entry!(
            0x00db_bfd0,
            hk_data_class_native_get_member_info(Ptr<HkDataClassNative>, u32, u32) -> u32
        ),
        entry!(
            0x00db_c000,
            hk_data_array_native_as_object(Ptr<HkDataArrayNative>, u32) -> u32
        ),
        entry!(
            0x00db_c0c0,
            hk_data_array_native_swizzle_object_member(
                Ptr<HkDataArrayNative>,
                u32,
            ) -> Ptr<HkDataArrayNative>
        ),
        entry!(
            0x00db_c150,
            hk_data_object_native_as_array(Ptr<HkDataObjectNative>, u32) -> Ptr<HkDataArrayNative>
        ),
        entry!(
            0x00db_c310,
            hk_data_object_native_as_string(Ptr<HkDataObjectNative>, u32) -> u32
        ),
        entry!(
            0x00db_c370,
            hk_data_object_native_as_int(Ptr<HkDataObjectNative>, u32) -> u64
        ),
        entry!(
            0x00db_c4d0,
            hk_data_object_native_as_object(Ptr<HkDataObjectNative>, u32) -> u32
        ),
        entry!(
            0x00db_c5c0,
            hk_data_object_native_as_real(Ptr<HkDataObjectNative>, u32) -> f32
        ),
        entry!(0x00db_c640, deleting_destructor_00dbc640(Ptr, u8) -> Ptr),
        entry!(
            0x00dc_9520,
            hk_data_object_native_get_next_member(Ptr<HkDataObjectNative>, i32) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

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
    fn deleting_destructor_00dbc640_frees_only_when_bit_zero_is_set() {
        check(0x00db_c640, 0x00db_b3f0);
    }

    // The Native data classes: shared scaffolding. Fake pointers (small numbers) are only ever
    // passed on, never read.
    type PairLog = Rc<RefCell<Vec<(u32, u32)>>>;

    const ALLOCATE_DOUBLE: u32 = 0x0910_0001;
    const FREE_SIZED_DOUBLE: u32 = 0x0910_0003;
    const REPORT_DOUBLE: u32 = 0x0910_0004;
    const FIND_CLASS_DOUBLE: u32 = 0x0910_0005;
    const SLOT_DOUBLE: u32 = 0x0910_0006;
    const OTHER_SLOT_DOUBLE: u32 = 0x0910_0007;
    const FINISH_DOUBLE: u32 = 0x0910_0009;

    fn logged() -> Engine {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| answer(eax));
    }

    /// An object whose vtable holds `(byte offset, function)` slots.
    fn object_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The memory router with an allocator whose slot 4 allocates, slot 8 frees (nothing is
    /// recorded for it) and slot `0x10` frees with a size (logged).
    fn install_allocator(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_SIZED_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        stub(e, MEMORY_ROUTER, router);
        e.register(ALLOCATE_DOUBLE, |e, a| {
            answer(if a[1] == 0xdead { 0 } else { e.mem.alloc(a[1]) })
        });
        stub(e, FREE_SLOT_DOUBLE, 0);
        stub(e, FREE_SIZED_DOUBLE, 0);
        allocator
    }

    /// Doubles for the message stream (the writers return the stream) and an error object
    /// whose slot `0xc` is `REPORT_DOUBLE`; returns the error object.
    fn install_reporting(e: &mut Engine) -> u32 {
        stub(e, STREAM_CONSTRUCT, 0);
        e.register(STREAM_WRITE_TEXT, |_, a| answer(a[0]));
        e.register(STREAM_WRITE_POINTER, |_, a| answer(a[0]));
        stub(e, STREAM_DESTRUCT, 0);
        let error_object = object_with(e, &[(0xc, REPORT_DOUBLE)]);
        e.mem.set_u32(ERROR_OBJECT, error_object);
        stub(e, REPORT_DOUBLE, 0);
        error_object
    }

    /// The words written with `operator<<(text)`, in order.
    fn texts(e: &Engine) -> Vec<u32> {
        calls_to(e, STREAM_WRITE_TEXT)
            .iter()
            .map(|words| words[1])
            .collect()
    }

    /// `hkClass::getName` double: the name of a class is its address plus `0x5000`.
    fn install_class_names(e: &mut Engine) {
        e.register(CLASS_GET_NAME, |_, a| answer(a[0] + 0x5000));
    }

    /// `hkClass::hasVtable` double: true for the listed classes, written through the flag
    /// pointer that is also returned.
    fn install_has_vtable(e: &mut Engine, with_vtable: &'static [u32]) {
        e.register_double(CLASS_HAS_VTABLE, move |e, a| {
            e.mem.set_u8(a[1], u8::from(with_vtable.contains(&a[0])));
            answer(a[1])
        });
    }

    /// `hkClassAccessor` copy double: logs the `(object, class)` pair it is given and copies it.
    fn install_pair_copy(e: &mut Engine) -> PairLog {
        let log: PairLog = Rc::new(RefCell::new(Vec::new()));
        let pairs = Rc::clone(&log);
        e.register_double(CLASS_ACCESSOR_COPY, move |e, a| {
            let pair = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            pairs.borrow_mut().push(pair);
            e.mem.set_u32(a[0], pair.0);
            e.mem.set_u32(a[0] + 4, pair.1);
            answer(a[0])
        });
        log
    }

    /// The one report the error object received, as `(severity, id, file, line)`.
    fn report(e: &Engine) -> (u32, u32, u32, u32) {
        let reports = calls_to(e, REPORT_DOUBLE);
        assert_eq!(reports.len(), 1);
        let words = &reports[0];
        (words[1], words[2], words[4], words[5])
    }

    #[test]
    fn world_new_class_returns_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x00db_b120, &args![0x1000u32, 7u32]).u32(), 0);
    }

    #[test]
    fn world_new_array_returns_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x00db_b130, &args![0x1000u32, 7u32, 8u32]).u32(), 0);
    }

    #[test]
    fn get_variant_with_most_derived_class_covers_each_outcome() {
        let mut e = logged();
        let error_object = install_reporting(&mut e);
        install_class_names(&mut e);
        install_has_vtable(&mut e, &[0x2000]);
        let instance_registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        let name_registry = object_with(&mut e, &[(0x10, OTHER_SLOT_DOUBLE)]);
        // The instance registry knows every object but 0x7001; the name registry knows the
        // name of class 0x3000 (0x8000).
        e.register(SLOT_DOUBLE, |_, a| {
            answer(if a[1] == 0x7001 { 0 } else { 0x3000 })
        });
        e.register(OTHER_SLOT_DOUBLE, |_, a| {
            answer(if a[1] == 0x8000 { 0x4000 } else { 0 })
        });
        let pair = |object: u32, class: u32| u64::from(class) << 32 | u64::from(object);
        let run = |e: &mut Engine, object: u32, class: u32| {
            e.call(
                0x00db_b140,
                &args![object, class, instance_registry, name_registry],
            )
            .u64()
        };
        assert_eq!(run(&mut e, 0x7000, 0), 0);
        assert_eq!(run(&mut e, 0, 0x2000), 0);
        assert_eq!(run(&mut e, 0x7000, 0x1000), pair(0x7000, 0x1000));
        assert!(calls_to(&e, SLOT_DOUBLE).is_empty());
        assert_eq!(run(&mut e, 0x7000, 0x2000), pair(0x7000, 0x4000));
        assert_eq!(
            calls_to(&e, SLOT_DOUBLE),
            vec![vec![instance_registry, 0x7000]]
        );
        assert_eq!(
            calls_to(&e, OTHER_SLOT_DOUBLE),
            vec![vec![name_registry, 0x8000]]
        );
        assert!(calls_to(&e, REPORT_DOUBLE).is_empty());
        // The instance registry does not know the object.
        assert_eq!(run(&mut e, 0x7001, 0x2000), 0);
        assert_eq!(report(&e), (1, 0x3476_d70f, SOURCE_FILE_NAME, 0x37));
        assert_eq!(calls_to(&e, REPORT_DOUBLE)[0][0], error_object);
        assert_eq!(
            texts(&e),
            vec![TEXT_MOST_DERIVED, 0x7000, TEXT_AT, TEXT_REPLACED_WITH_NULL]
        );
        assert_eq!(calls_to(&e, STREAM_WRITE_POINTER)[0][1], 0x7001);
    }

    #[test]
    fn impl_create_wraps_a_known_pair() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        let pairs = install_pair_copy(&mut e);
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        let found = e.mem.alloc(0x20);
        e.mem.set_u32(found + 0xc, 0x6000);
        stub(&mut e, FIND_CLASS_DOUBLE, found);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x7000);
        e.mem.set_u32(pair + 4, 0x5000);
        let object = e.call(0x00db_b260, &args![world, pair]).u32();
        assert_eq!(e.mem.u32(pair + 4), 0x6000);
        assert_eq!(e.mem.u16(object + 4), 0x18);
        assert_eq!(e.mem.u16(object + 6), 0);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), world);
        assert_eq!(pairs.borrow().as_slice(), &[(0x7000, 0x6000)]);
        assert_eq!(
            calls_to(&e, CLASS_ACCESSOR_COPY)[0][..2],
            [object + 0xc, pair]
        );
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE), vec![vec![world, 0xa000]]);
        assert!(calls_to(&e, REPORT_DOUBLE).is_empty());
    }

    #[test]
    fn impl_create_without_a_class_does_not_look_one_up() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        let pairs = install_pair_copy(&mut e);
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        stub(&mut e, FIND_CLASS_DOUBLE, 0);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x7000);
        let object = e.call(0x00db_b260, &args![world, pair]).u32();
        assert_eq!(e.mem.u32(object + 0x14), world);
        assert_eq!(pairs.borrow().as_slice(), &[(0x7000, 0)]);
        assert!(calls_to(&e, FIND_CLASS_DOUBLE).is_empty());
        assert!(calls_to(&e, REPORT_DOUBLE).is_empty());
    }

    #[test]
    fn impl_create_reports_and_clears_an_unregistered_class() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        let pairs = install_pair_copy(&mut e);
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        stub(&mut e, FIND_CLASS_DOUBLE, 0);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x7000);
        e.mem.set_u32(pair + 4, 0x5000);
        let object = e.call(0x00db_b260, &args![world, pair]).u32();
        assert_eq!(e.mem.u32(pair), 0);
        assert_eq!(e.mem.u32(pair + 4), 0);
        assert_eq!(pairs.borrow().as_slice(), &[(0, 0)]);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(report(&e), (1, 0x1800_473a, SOURCE_FILE_NAME, 0x2c7));
        assert_eq!(
            texts(&e),
            vec![
                TEXT_IGNORE_OBJECT,
                TEXT_CLASS_OPEN,
                0xa000,
                TEXT_NOT_REGISTERED_PROVIDED
            ]
        );
        assert_eq!(calls_to(&e, STREAM_WRITE_POINTER)[0][1], 0x7000);
    }

    #[test]
    fn world_constructor_takes_both_registries_from_the_singleton() {
        let mut e = logged();
        stub(&mut e, MAP_CONSTRUCT, 0);
        stub(&mut e, ADD_REFERENCE, 0);
        stub(&mut e, REMOVE_REFERENCE, 0);
        let singleton = object_with(&mut e, &[(0x10, SLOT_DOUBLE), (0x14, OTHER_SLOT_DOUBLE)]);
        e.mem.set_u32(REGISTRY_SINGLETON, singleton);
        stub(&mut e, SLOT_DOUBLE, 0xaaa0);
        stub(&mut e, OTHER_SLOT_DOUBLE, 0xbbb0);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, 0x1111);
        e.mem.set_u32(this + 0xc, 0x2222);
        assert_eq!(e.call(0x00db_b360, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), WORLD_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0xaaa0);
        assert_eq!(e.mem.u32(this + 0xc), 0xbbb0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(
            calls_to(&e, MAP_CONSTRUCT),
            vec![vec![this + 0x10, this & 0xffff_ff00]]
        );
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![0xbbb0], vec![0xaaa0]]
        );
        // The constructor clears both fields first, so nothing is ever released.
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        // Null registries are not referenced.
        stub(&mut e, SLOT_DOUBLE, 0);
        stub(&mut e, OTHER_SLOT_DOUBLE, 0);
        e.call(0x00db_b360, &args![this]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 2);
    }

    #[test]
    fn world_destructor_deletes_the_cached_classes_and_clears_the_registries() {
        let mut e = logged();
        stub(&mut e, MAP_GET_ITERATOR, 1);
        e.register(MAP_IS_VALID, |e, a| {
            e.mem.set_u8(a[1], u8::from(a[2] <= 2));
            Ret::default()
        });
        let class = object_with(&mut e, &[(0, SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0);
        e.register_double(MAP_GET_VALUE, move |_, a| {
            answer(if a[1] == 1 { class } else { 0 })
        });
        e.register(MAP_GET_NEXT, |_, a| answer(a[1] + 1));
        stub(&mut e, MAP_DESTRUCT, 0);
        stub(&mut e, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, 0x1111);
        e.mem.set_u32(this + 0xc, 0x2222);
        e.call(0x00db_b3f0, &args![this]);
        assert_eq!(calls_to(&e, SLOT_DOUBLE), vec![vec![class, 1]]);
        assert_eq!(calls_to(&e, MAP_DESTRUCT), vec![vec![this + 0x10]]);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0x2222], vec![0x1111]]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(calls_to(&e, MAP_GET_NEXT).len(), 2);
    }

    #[test]
    fn world_set_class_registry_swaps_the_references() {
        let mut e = logged();
        stub(&mut e, ADD_REFERENCE, 0);
        stub(&mut e, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x28);
        e.call(0x00db_b490, &args![this, 0x3333u32]);
        assert_eq!(e.mem.u32(this + 8), 0x3333);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x3333]]);
        assert!(calls_to(&e, REMOVE_REFERENCE).is_empty());
        e.call(0x00db_b490, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![0x3333]]);
    }

    #[test]
    fn world_wrap_object_builds_the_pair_on_the_stack() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        let pairs = install_pair_copy(&mut e);
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        let object = e.call(0x00db_b4f0, &args![world, 0x7000u32, 0u32]).u32();
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), world);
        assert_eq!(pairs.borrow().as_slice(), &[(0x7000, 0)]);
    }

    #[test]
    fn world_find_class_uses_the_cache_then_the_registry() {
        let mut e = logged();
        install_allocator(&mut e);
        stub(&mut e, MAP_GET_WITH_DEFAULT, 0x4444);
        stub(&mut e, MAP_INSERT, 0);
        let registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0x7000);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, registry);
        // A cached class is answered without anything else.
        assert_eq!(e.call(0x00db_b520, &args![this, 0x5555u32]).u32(), 0x4444);
        assert_eq!(
            calls_to(&e, MAP_GET_WITH_DEFAULT),
            vec![vec![this + 0x10, 0x5555, 0]]
        );
        assert!(calls_to(&e, MAP_INSERT).is_empty());
        // A new class is made over the registry's class and inserted under the name.
        stub(&mut e, MAP_GET_WITH_DEFAULT, 0);
        let made = e.call(0x00db_b520, &args![this, 0x5555u32]).u32();
        assert_eq!(e.mem.u16(made + 4), 0x14);
        assert_eq!(e.mem.u16(made + 6), 0);
        assert_eq!(e.mem.u32(made + 8), 0);
        assert_eq!(e.mem.u32(made), CLASS_VTABLE);
        assert_eq!(e.mem.u32(made + 0xc), 0x7000);
        assert_eq!(e.mem.u32(made + 0x10), this);
        assert_eq!(calls_to(&e, SLOT_DOUBLE), vec![vec![registry, 0x5555]]);
        assert_eq!(
            calls_to(&e, MAP_INSERT),
            vec![vec![this + 0x10, 0x5555, made]]
        );
        // A null name never searches: the class is empty and is inserted under 0.
        let empty = e.call(0x00db_b520, &args![this, 0u32]).u32();
        assert_eq!(e.mem.u32(empty + 0xc), 0);
        assert_eq!(calls_to(&e, MAP_GET_WITH_DEFAULT).len(), 2);
        assert_eq!(calls_to(&e, SLOT_DOUBLE).len(), 1);
        assert_eq!(calls_to(&e, MAP_INSERT)[1], vec![this + 0x10, 0, empty]);
    }

    #[test]
    fn world_find_object_takes_a_missing_class_from_the_instance_registry() {
        let mut e = logged();
        install_allocator(&mut e);
        stub(&mut e, CLASS_ACCESSOR_INIT_PAIR, 0);
        stub(&mut e, DATA_OBJECT_CONSTRUCT, 0);
        let registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0x6000);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0xc, registry);
        let handle = e.mem.alloc(8);
        e.mem.set_u32(handle, 0x7000);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_b5a0, &args![this, out, handle]).u32(), out);
        let init = calls_to(&e, CLASS_ACCESSOR_INIT_PAIR);
        assert_eq!(init.len(), 1);
        let object = init[0][0] - 0xc;
        assert_eq!(init[0][1..], [0x7000, 0x6000]);
        assert_eq!(e.mem.u16(object + 4), 0x18);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), this);
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, object]]);
        assert_eq!(calls_to(&e, SLOT_DOUBLE), vec![vec![registry, 0x7000]]);
        // With a class in the handle the registry is not asked.
        e.mem.set_u32(handle + 4, 0x6100);
        e.call(0x00db_b5a0, &args![this, out, handle]);
        assert_eq!(calls_to(&e, SLOT_DOUBLE).len(), 1);
        assert_eq!(
            calls_to(&e, CLASS_ACCESSOR_INIT_PAIR)[1][1..],
            [0x7000, 0x6100]
        );
    }

    #[test]
    fn world_get_type_answers_the_result_pointer() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        // EAX is `out` at the RET 00dbb61a; the tagfile reader tests `[EAX]` (00dc6ccc).
        assert_eq!(e.call(0x00db_b610, &args![0x1000u32, out]).u32(), out);
        let half = e.mem.alloc(4);
        // asHalf: EAX is `out` at the RET 00dbba34.
        assert_eq!(
            e.call(0x00db_ba20, &args![0x1000u32, half, 5u32]).u32(),
            half
        );
    }

    #[test]
    fn world_get_type_stores_zero() {
        let mut e = engine();
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0xdead_beef);
        e.call(0x00db_b610, &args![0x1000u32, out]);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn world_get_contents_wraps_the_stored_pair() {
        let mut e = logged();
        install_allocator(&mut e);
        stub(&mut e, CLASS_ACCESSOR_COPY, 0);
        stub(&mut e, DATA_OBJECT_CONSTRUCT, 0);
        let this = e.mem.alloc(0x28);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_b620, &args![this, out]).u32(), out);
        let copy = calls_to(&e, CLASS_ACCESSOR_COPY);
        let object = copy[0][0] - 0xc;
        assert_eq!(copy[0][1], this + 0x20);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(e.mem.u16(object + 4), 0x18);
        assert_eq!(e.mem.u32(object + 0x14), this);
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, object]]);
    }

    #[test]
    fn world_new_object_makes_a_zeroed_block_and_runs_the_registered_constructor() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        let pairs = install_pair_copy(&mut e);
        e.register(DATA_CLASS_GET_NAME, |_, a| answer(a[0] + 0x100));
        stub(&mut e, CLASS_GET_OBJECT_SIZE, 0x20);
        stub(&mut e, MEMORY_SET, 0);
        stub(&mut e, FINISH_DOUBLE, 0);
        // Class registry: the hkClass of a name; world: findClass; vtable-class registry: the
        // entry holding the constructor at +4.
        let class_registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0x6000);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 4, FINISH_DOUBLE);
        let vtable_registry = object_with(&mut e, &[(0x1c, OTHER_SLOT_DOUBLE)]);
        stub(&mut e, OTHER_SLOT_DOUBLE, entry);
        let singleton = object_with(&mut e, &[(0xc, FIND_CLASS_DOUBLE)]);
        e.mem.set_u32(REGISTRY_SINGLETON, singleton);
        stub(&mut e, FIND_CLASS_DOUBLE, vtable_registry);
        let world_find = 0x0910_000a;
        let world = object_with(&mut e, &[(0x24, world_find)]);
        let registered = e.mem.alloc(0x20);
        e.mem.set_u32(registered + 0xc, 0x6500);
        stub(&mut e, world_find, registered);
        e.mem.set_u32(world + 8, class_registry);
        let object = e.call(0x00db_b680, &args![world, 0x9000u32]).u32();
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
        assert_eq!(
            calls_to(&e, SLOT_DOUBLE),
            vec![vec![class_registry, 0x9100]]
        );
        let memset = calls_to(&e, MEMORY_SET);
        let block = memset[0][0];
        assert_eq!(memset, vec![vec![block, 0, 0x20]]);
        assert_eq!(
            calls_to(&e, OTHER_SLOT_DOUBLE),
            vec![vec![vtable_registry, 0x9100]]
        );
        assert_eq!(calls_to(&e, FINISH_DOUBLE), vec![vec![block, 0]]);
        // The object wraps the block with the class the world registered for the hkClass.
        assert_eq!(pairs.borrow().as_slice(), &[(block, 0x6500)]);
    }

    #[test]
    fn world_new_object_copes_without_memory_entry_or_constructor() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        let pairs = install_pair_copy(&mut e);
        e.register(DATA_CLASS_GET_NAME, |_, a| answer(a[0] + 0x100));
        stub(&mut e, CLASS_GET_OBJECT_SIZE, 0x20);
        stub(&mut e, MEMORY_SET, 0);
        let class_registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0x6000);
        let vtable_registry = object_with(&mut e, &[(0x1c, OTHER_SLOT_DOUBLE)]);
        // No entry for the class.
        stub(&mut e, OTHER_SLOT_DOUBLE, 0);
        let singleton = object_with(&mut e, &[(0xc, FIND_CLASS_DOUBLE)]);
        e.mem.set_u32(REGISTRY_SINGLETON, singleton);
        stub(&mut e, FIND_CLASS_DOUBLE, vtable_registry);
        let world = object_with(&mut e, &[(0x24, 0x0910_000a)]);
        stub(&mut e, 0x0910_000a, 0);
        e.mem.set_u32(world + 8, class_registry);
        e.call(0x00db_b680, &args![world, 0x9000u32]);
        assert_eq!(calls_to(&e, MEMORY_SET).len(), 1);
        assert_eq!(calls_to(&e, OTHER_SLOT_DOUBLE).len(), 1);
        // The world does not know the class: the pair is reported and cleared.
        assert_eq!(pairs.borrow().as_slice(), &[(0, 0)]);
        // An entry without a constructor (word at +4 zero) does not call one.
        let entry = e.mem.alloc(0x10);
        stub(&mut e, OTHER_SLOT_DOUBLE, entry);
        e.call(0x00db_b680, &args![world, 0x9000u32]);
        assert_eq!(calls_to(&e, MEMORY_SET).len(), 2);
        // A class whose object size cannot be allocated is not zeroed or constructed.
        stub(&mut e, CLASS_GET_OBJECT_SIZE, 0xdead);
        e.call(0x00db_b680, &args![world, 0x9000u32]);
        assert_eq!(calls_to(&e, MEMORY_SET).len(), 2);
        assert_eq!(calls_to(&e, OTHER_SLOT_DOUBLE).len(), 2);
    }

    #[test]
    fn world_set_contents_finds_the_class_of_a_virtual_object_or_by_name() {
        let mut e = logged();
        install_reporting(&mut e);
        install_class_names(&mut e);
        install_has_vtable(&mut e, &[0x2000]);
        let instance_registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        let class_registry = object_with(&mut e, &[(0x10, OTHER_SLOT_DOUBLE)]);
        e.register(SLOT_DOUBLE, |_, a| {
            answer(if a[1] == 0x7001 { 0 } else { 0x3000 })
        });
        e.register(OTHER_SLOT_DOUBLE, |_, a| answer(a[1] + 1));
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, class_registry);
        e.mem.set_u32(this + 0xc, instance_registry);
        // A class with a vtable: the instance registry gives the hkClass, whose name is looked up.
        e.call(0x00db_b730, &args![this, 0x7000u32, 0x2000u32]);
        assert_eq!(e.mem.u32(this + 0x20), 0x7000);
        assert_eq!(e.mem.u32(this + 0x24), 0x8001);
        assert_eq!(
            calls_to(&e, SLOT_DOUBLE),
            vec![vec![instance_registry, 0x7000]]
        );
        assert_eq!(
            calls_to(&e, OTHER_SLOT_DOUBLE),
            vec![vec![class_registry, 0x8000]]
        );
        // A class without a vtable: the name of the class itself.
        e.call(0x00db_b730, &args![this, 0x7002u32, 0x1000u32]);
        assert_eq!(e.mem.u32(this + 0x20), 0x7002);
        assert_eq!(e.mem.u32(this + 0x24), 0x6001);
        assert_eq!(calls_to(&e, SLOT_DOUBLE).len(), 1);
        assert!(calls_to(&e, REPORT_DOUBLE).is_empty());
    }

    #[test]
    fn world_set_contents_reports_an_unregistered_object() {
        let mut e = logged();
        install_reporting(&mut e);
        install_class_names(&mut e);
        install_has_vtable(&mut e, &[0x2000]);
        let instance_registry = object_with(&mut e, &[(0x10, SLOT_DOUBLE)]);
        let class_registry = object_with(&mut e, &[(0x10, OTHER_SLOT_DOUBLE)]);
        stub(&mut e, SLOT_DOUBLE, 0);
        stub(&mut e, OTHER_SLOT_DOUBLE, 0);
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 8, class_registry);
        e.mem.set_u32(this + 0xc, instance_registry);
        // The instance registry has no class for the object: nothing is looked up by name.
        e.call(0x00db_b730, &args![this, 0x7000u32, 0x2000u32]);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert!(calls_to(&e, OTHER_SLOT_DOUBLE).is_empty());
        assert_eq!(report(&e), (1, 0x1800_473b, SOURCE_FILE_NAME, 0x2fc));
        assert_eq!(
            texts(&e),
            vec![
                TEXT_CANNOT_SET_CONTENTS,
                TEXT_CLASS_OPEN,
                0x7000,
                TEXT_NOT_REGISTERED_IGNORE
            ]
        );
        assert_eq!(calls_to(&e, STREAM_WRITE_POINTER)[0][1], 0x7000);
    }

    #[test]
    fn world_find_all_classes_appends_the_unflagged_classes() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_class_names(&mut e);
        let registry = object_with(&mut e, &[(0x14, SLOT_DOUBLE)]);
        // The registry fills the local array with three classes (0x1000, 0x1001, 0x1002) with
        // room for four.
        e.register(SLOT_DOUBLE, |e, a| {
            let data = e.mem.alloc(16);
            for (i, class) in [0x1000u32, 0x1001, 0x1002].into_iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, class);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, 3);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        // The middle class has flag bit 0.
        e.register_double(CLASS_GET_FLAGS, |e, a| {
            let flags = e.mem.alloc(4);
            e.mem.set_u8(flags, u8::from(a[0] == 0x1001));
            answer(flags)
        });
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        e.register(FIND_CLASS_DOUBLE, |_, a| answer(a[1] + 0x100));
        e.mem.set_u32(world + 8, registry);
        // The output array is empty and full, so it is grown first.
        let out = e.mem.alloc(12);
        e.register_double(ARRAY_GROW, |e, a| {
            let data = e.mem.alloc(32);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        e.call(0x00db_b840, &args![world, out]);
        assert_eq!(e.mem.u32(out + 4), 2);
        let data = e.mem.u32(out);
        assert_eq!(e.mem.u32(data), 0x6100);
        assert_eq!(e.mem.u32(data + 4), 0x6102);
        assert_eq!(calls_to(&e, ARRAY_GROW), vec![vec![allocator, out, 4]]);
        // The local array is freed with its capacity in bytes.
        let freed = calls_to(&e, FREE_SIZED_DOUBLE);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][0], allocator);
        assert_eq!(freed[0][2], 16);
        assert_eq!(calls_to(&e, SLOT_DOUBLE).len(), 1);
        assert_eq!(calls_to(&e, SLOT_DOUBLE)[0][0], registry);
    }

    #[test]
    fn world_find_all_classes_without_a_registry_does_nothing() {
        let mut e = logged();
        let world = e.mem.alloc(0x28);
        let out = e.mem.alloc(12);
        e.call(0x00db_b840, &args![world, out]);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    // Classes, objects and arrays.

    /// A `hkDataClassNative` over `hk_class` in `world`.
    fn class_native(e: &mut Engine, hk_class: u32, world: u32) -> u32 {
        let class = e.mem.alloc(0x14);
        e.mem.set_u32(class + 0xc, hk_class);
        e.mem.set_u32(class + 0x10, world);
        class
    }

    /// A `hkDataObjectNative` with the accessor pair `(object, class)` in `world`.
    fn object_native(e: &mut Engine, object: u32, class: u32, world: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, object);
        e.mem.set_u32(this + 0x10, class);
        e.mem.set_u32(this + 0x14, world);
        this
    }

    /// A `hkDataArrayNative` with the given fields.
    #[allow(clippy::too_many_arguments)] // one word per field of the layout
    fn array_native(
        e: &mut Engine,
        world: u32,
        data: u32,
        count: u32,
        stride: u32,
        member_type: u32,
        class: u32,
        tuple_size: u32,
    ) -> u32 {
        let this = e.mem.alloc(0x2c);
        for (offset, value) in [
            (0xc, world),
            (0x10, data),
            (0x14, count),
            (0x18, stride),
            (0x1c, member_type),
            (0x20, class),
            (0x28, tuple_size),
        ] {
            e.mem.set_u32(this + offset, value);
        }
        this
    }

    /// An `hkClassMember` record: name word, type, subtype, flags and 16-bit offset.
    fn member_record(e: &mut Engine, name: u32, member_type: u8, subtype: u8, flags: u16) -> u32 {
        let member = e.mem.alloc(0x20);
        e.mem.set_u32(member, name);
        e.mem.set_u8(member + 0xc, member_type);
        e.mem.set_u8(member + 0xd, subtype);
        e.mem.set_u16(member + 0x10, flags);
        member
    }

    /// A world whose slot `0x24` (`findClass`) answers the name plus `0x100`.
    fn finding_world(e: &mut Engine) -> u32 {
        e.register(FIND_CLASS_DOUBLE, |_, a| answer(a[1] + 0x100));
        object_with(e, &[(0x24, FIND_CLASS_DOUBLE)])
    }

    #[test]
    fn class_native_get_name_forwards_to_the_hk_class() {
        let mut e = logged();
        e.register(CLASS_GET_NAME, |_, a| answer(a[0] + 1));
        let this = class_native(&mut e, 0x6000, 0);
        assert_eq!(e.call(0x00db_b920, &args![this]).u32(), 0x6001);
    }

    #[test]
    fn class_native_get_version_forwards_to_the_hk_class() {
        let mut e = logged();
        e.register(CLASS_GET_DESCRIBED_VERSION, |_, a| answer(a[0] + 2));
        let this = class_native(&mut e, 0x6000, 0);
        assert_eq!(e.call(0x00db_b930, &args![this]).u32(), 0x6002);
    }

    #[test]
    fn class_native_get_parent_finds_the_parent_class_by_name() {
        let mut e = logged();
        install_class_names(&mut e);
        let world = finding_world(&mut e);
        e.register(CLASS_GET_PARENT, |_, a| {
            answer(if a[0] == 0x6000 { 0x6100 } else { 0 })
        });
        let this = class_native(&mut e, 0x6000, world);
        // Parent 0x6100 has the name 0xb100, which the world finds as 0xb200.
        assert_eq!(e.call(0x00db_b940, &args![this]).u32(), 0xb200);
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE), vec![vec![world, 0xb100]]);
        let orphan = class_native(&mut e, 0x6200, world);
        assert_eq!(e.call(0x00db_b940, &args![orphan]).u32(), 0);
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE).len(), 1);
    }

    /// An object with the slots `isSuperClass` walks: name at slot 8, parent at slot `0x10`.
    fn named_object(e: &mut Engine, name: u32, parent: u32) -> u32 {
        let object = object_with(e, &[(8, SLOT_DOUBLE), (0x10, OTHER_SLOT_DOUBLE)]);
        e.mem.set_u32(object + 0x20, name);
        e.mem.set_u32(object + 0x24, parent);
        object
    }

    #[test]
    fn class_native_is_super_class_walks_the_parents() {
        let mut e = logged();
        e.register(SLOT_DOUBLE, |e, a| answer(e.mem.u32(a[0] + 0x20)));
        e.register(OTHER_SLOT_DOUBLE, |e, a| answer(e.mem.u32(a[0] + 0x24)));
        e.register(STRING_COMPARE, |_, a| answer(u32::from(a[0] != a[1])));
        let this = named_object(&mut e, 2, 0);
        let grandparent = named_object(&mut e, 2, 0);
        let parent = named_object(&mut e, 3, grandparent);
        let child = named_object(&mut e, 4, parent);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xff);
        // The grandparent has the name of this class.
        e.call(0x00db_b970, &args![this, out, child]);
        assert_eq!(e.mem.u8(out), 1);
        // The walk stops at the first match: child, parent, grandparent.
        assert_eq!(calls_to(&e, STRING_COMPARE).len(), 3);
        // No ancestor matches.
        let unrelated = named_object(&mut e, 9, 0);
        let other = named_object(&mut e, 8, unrelated);
        e.call(0x00db_b970, &args![this, out, other]);
        assert_eq!(e.mem.u8(out), 0);
        // A null class has no ancestors.
        e.mem.set_u8(out, 0xff);
        e.call(0x00db_b970, &args![this, out, 0u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00dbb970_answers_its_result_pointer() {
        let mut e = logged();
        e.register(SLOT_DOUBLE, |e, a| answer(e.mem.u32(a[0] + 0x20)));
        e.register(OTHER_SLOT_DOUBLE, |e, a| answer(e.mem.u32(a[0] + 0x24)));
        e.register(STRING_COMPARE, |_, a| answer(u32::from(a[0] != a[1])));
        let this = named_object(&mut e, 2, 0);
        let grandparent = named_object(&mut e, 2, 0);
        let parent = named_object(&mut e, 3, grandparent);
        let child = named_object(&mut e, 4, parent);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xff);
        // The grandparent has the name of this class.
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            e.call(0x00db_b970, &args![this, out, child]).u32(),
            args![out][0]
        );
    }

    #[test]
    fn class_native_member_counts_and_lookups_forward_to_the_hk_class() {
        let mut e = logged();
        e.register(CLASS_GET_NUM_DECLARED_MEMBERS, |_, a| answer(a[0] + 3));
        e.register(CLASS_GET_NUM_MEMBERS, |_, a| answer(a[0] + 4));
        e.register(CLASS_GET_DECLARED_MEMBER_INDEX_BY_NAME, |_, a| {
            answer(a[0] + a[1])
        });
        e.register(CLASS_GET_MEMBER_INDEX_BY_NAME, |_, a| answer(a[0] ^ a[1]));
        let this = class_native(&mut e, 0x6000, 0);
        assert_eq!(e.call(0x00db_b9d0, &args![this]).u32(), 0x6003);
        assert_eq!(e.call(0x00db_b9f0, &args![this]).u32(), 0x6004);
        assert_eq!(e.call(0x00db_b9e0, &args![this, 0x10u32]).u32(), 0x6010);
        assert_eq!(e.call(0x00db_ba00, &args![this, 0x0fu32]).u32(), 0x600f);
    }

    #[test]
    fn data_vtable_slots_that_return_nothing_return_zero() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00db_ba10, &args![0x1000u32, 5u32]).f32().to_bits(),
            0
        );
        assert_eq!(e.call(0x00db_ba40, &args![0x1000u32, 5u32]).u64(), 0);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xffff_ffff);
        e.call(0x00db_ba20, &args![0x1000u32, out, 5u32]);
        assert_eq!(e.mem.u16(out), 0);
        assert_eq!(e.mem.u16(out + 2), 0xffff);
    }

    #[test]
    fn object_native_get_handle_hides_the_class_of_a_virtual_object() {
        let mut e = logged();
        install_has_vtable(&mut e, &[0x6100]);
        let plain = object_native(&mut e, 0x7000, 0x6000, 0);
        let virtual_object = object_native(&mut e, 0x7001, 0x6100, 0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_ba50, &args![plain, out]).u32(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x7000, 0x6000));
        assert_eq!(e.call(0x00db_ba50, &args![virtual_object, out]).u32(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x7001, 0));
    }

    #[test]
    fn object_native_get_class_finds_the_class_by_name() {
        let mut e = logged();
        install_class_names(&mut e);
        let world = finding_world(&mut e);
        let this = object_native(&mut e, 0x7000, 0x6000, world);
        assert_eq!(e.call(0x00db_ba80, &args![this]).u32(), 0xb100);
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE), vec![vec![world, 0xb000]]);
        let classless = object_native(&mut e, 0x7000, 0, world);
        assert_eq!(e.call(0x00db_ba80, &args![classless]).u32(), 0);
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE).len(), 1);
    }

    /// The accessor double: the constructor keeps the name in the first word of its local.
    fn install_accessor(e: &mut Engine) {
        e.register(ACCESSOR_INIT, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            answer(a[0])
        });
    }

    #[test]
    fn object_native_access_by_name_stores_the_object_and_the_name() {
        let mut e = logged();
        install_accessor(&mut e);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_bab0, &args![this, out, 0x1234u32]).u32(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (this, 0x1234));
        let init = calls_to(&e, ACCESSOR_INIT);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1..], [this + 0xc, 0x1234]);
    }

    #[test]
    fn object_native_has_member_asks_the_accessor() {
        let mut e = logged();
        install_accessor(&mut e);
        e.register_double(ACCESSOR_IS_OK, |e, a| {
            let name = e.mem.u32(a[0]);
            e.mem.set_u8(a[1], u8::from(name == 5));
            answer(a[1])
        });
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        assert!(e.call(0x00db_bae0, &args![this, 5u32]).bool());
        assert!(!e.call(0x00db_bae0, &args![this, 6u32]).bool());
    }

    #[test]
    fn object_native_is_valid_compares_with_the_member_count() {
        let mut e = logged();
        stub(&mut e, CLASS_GET_NUM_MEMBERS, 3);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        assert!(e.call(0x00db_bb20, &args![this, 2u32]).bool());
        assert!(!e.call(0x00db_bb20, &args![this, 3u32]).bool());
        assert!(e.call(0x00db_bb20, &args![this, -1i32]).bool());
        assert_eq!(calls_to(&e, CLASS_GET_NUM_MEMBERS)[0], vec![0x6000]);
    }

    #[test]
    fn object_native_get_member_value_pairs_the_object_with_the_member_name() {
        let mut e = logged();
        e.register(SLOT_DOUBLE, |_, a| answer(a[1] + 0x10));
        let this = object_with(&mut e, &[(0x20, SLOT_DOUBLE)]);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00db_bb40, &args![this, out, 7u32]).u32(), out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (this, 0x17));
    }

    #[test]
    fn object_native_as_vec_returns_the_member_address() {
        let mut e = logged();
        install_accessor(&mut e);
        stub(&mut e, ACCESSOR_GET_ADDRESS, 0x7777);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        assert_eq!(
            e.call(0x00db_bb60, &args![this, 0x1234u32, 9u32]).u32(),
            0x7777
        );
        assert_eq!(calls_to(&e, ACCESSOR_INIT)[0][1..], [this + 0xc, 0x1234]);
    }

    #[test]
    fn array_native_constructor_stores_the_fields_and_the_element_type() {
        let mut e = logged();
        let world = e.mem.alloc(0x28);
        e.register(TYPE_FROM_MEMBER_TYPE, |e, a| {
            // The code is the class pointer's first word.
            answer(e.mem.u32(a[2]))
        });
        let class = e.mem.alloc(4);
        let this = e.mem.alloc(0x2c);
        for (code, element_type) in [
            (0x30u32, 0x20u32),
            (0x11, 1),
            (0x10, 0x10),
            (0xffff_fff0, 0x10),
        ] {
            e.mem.set_u32(class, code);
            e.mem.set_u32(this + 6, 0x1_0001);
            let got = e
                .call(
                    0x00db_bb90,
                    &args![this, world, 0x7000u32, 5u32, 12u32, 0x19u32, class, 3u32],
                )
                .u32();
            assert_eq!(got, this);
            assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
            assert_eq!(e.mem.u16(this + 6), 0);
            assert_eq!(e.mem.u32(this + 8), 0);
            assert_eq!(e.mem.u32(this + 0xc), world);
            assert_eq!(e.mem.u32(this + 0x10), 0x7000);
            assert_eq!(e.mem.u32(this + 0x14), 5);
            assert_eq!(e.mem.u32(this + 0x18), 12);
            assert_eq!(e.mem.u32(this + 0x1c), 0x19);
            assert_eq!(e.mem.u32(this + 0x20), class);
            assert_eq!(e.mem.u32(this + 0x28), 3);
            assert_eq!(e.mem.u32(this + 0x24), element_type);
            let last = calls_to(&e, TYPE_FROM_MEMBER_TYPE).pop().unwrap();
            assert_eq!(last, vec![0x16, 0x19, class, 3]);
        }
    }

    #[test]
    fn two_shared_getters_read_the_fields() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x14, 0x1414);
        e.mem.set_u32(this + 0x24, 0x2424);
        assert_eq!(e.call(0x00db_bc00, &args![this]).u32(), 0x2424);
        assert_eq!(e.call(0x00db_bc10, &args![this]).u32(), 0x1414);
    }

    #[test]
    fn array_native_get_class_finds_the_class_by_name() {
        let mut e = logged();
        install_class_names(&mut e);
        let world = finding_world(&mut e);
        let array = array_native(&mut e, world, 0, 0, 0, 0, 0x6000, 0);
        assert_eq!(e.call(0x00db_bc20, &args![array]).u32(), 0xb100);
        assert_eq!(calls_to(&e, FIND_CLASS_DOUBLE), vec![vec![world, 0xb000]]);
        let classless = array_native(&mut e, world, 0, 0, 0, 0, 0, 0);
        assert_eq!(e.call(0x00db_bc20, &args![classless]).u32(), 0);
    }

    #[test]
    fn array_native_as_vec_is_the_element_address() {
        let mut e = engine();
        let array = array_native(&mut e, 0, 0x7000, 10, 12, 0, 0, 0);
        assert_eq!(e.call(0x00db_bc50, &args![array, 3u32]).u32(), 0x7000 + 36);
    }

    #[test]
    fn array_native_as_string_reads_c_strings_and_string_pointers() {
        let mut e = engine();
        let data = e.mem.alloc(16);
        e.mem.set_u32(data + 4, 0x9001);
        let array = array_native(&mut e, 0, data, 2, 4, 0x1d, 0, 0);
        assert_eq!(e.call(0x00db_bc60, &args![array, 1u32]).u32(), 0x9001);
        let array = array_native(&mut e, 0, data, 2, 4, 0x21, 0, 0);
        assert_eq!(e.call(0x00db_bc60, &args![array, 1u32]).u32(), 0x9000);
        let array = array_native(&mut e, 0, data, 2, 4, 0x1c, 0, 0);
        assert_eq!(e.call(0x00db_bc60, &args![array, 1u32]).u32(), 0);
    }

    #[test]
    fn array_native_as_real_reads_floats_and_halves() {
        let mut e = engine();
        let data = e.mem.alloc(16);
        e.mem.set_f32(data, 1.5);
        e.mem.set_u32(data + 4, 0x7f80_0001);
        e.mem.set_u16(data + 8, 0x3fc0);
        e.mem.set_u16(data + 10, 0xbf80);
        let float_array = array_native(&mut e, 0, data, 2, 4, 0xb, 0, 0);
        assert_eq!(e.call(0x00db_bca0, &args![float_array, 0u32]).f32(), 1.5);
        // A signalling NaN comes back quieted.
        let signalling = e.call(0x00db_bca0, &args![float_array, 1u32]).f32();
        assert_eq!(signalling.to_bits(), 0x7fc0_0001);
        let half_array = array_native(&mut e, 0, data + 8, 2, 2, 0x20, 0, 0);
        assert_eq!(e.call(0x00db_bca0, &args![half_array, 0u32]).f32(), 1.5);
        assert_eq!(e.call(0x00db_bca0, &args![half_array, 1u32]).f32(), -1.0);
        let other = array_native(&mut e, 0, data, 2, 4, 0xc, 0, 0);
        assert_eq!(e.call(0x00db_bca0, &args![other, 0u32]).f32().to_bits(), 0);
    }

    #[test]
    fn array_native_as_int_extends_by_member_type() {
        let mut e = engine();
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x8765_fff0);
        let read = |e: &mut Engine, member_type: u32| {
            let array = array_native(e, 0, data, 1, 4, member_type, 0, 0);
            e.call(0x00db_bcf0, &args![array, 0u32]).u32()
        };
        for member_type in [1, 2, 3] {
            assert_eq!(read(&mut e, member_type), 0xffff_fff0);
        }
        assert_eq!(read(&mut e, 4), 0xf0);
        assert_eq!(read(&mut e, 5), 0xffff_fff0);
        assert_eq!(read(&mut e, 6), 0xfff0);
        for member_type in [7, 8, 9, 10, 0x1e] {
            assert_eq!(read(&mut e, member_type), 0x8765_fff0);
        }
        for member_type in [0, 11, 0x1f] {
            assert_eq!(read(&mut e, member_type), 0);
        }
    }

    #[test]
    fn array_native_as_int64_reads_the_wide_element() {
        let mut e = engine();
        let data = e.mem.alloc(16);
        e.mem.set_u64(data + 8, 0x1122_3344_5566_7788);
        let array = array_native(&mut e, 0, data, 2, 8, 9, 0, 0);
        assert_eq!(
            e.call(0x00db_bd70, &args![array, 1u32]).u64(),
            0x1122_3344_5566_7788
        );
    }

    #[test]
    fn array_native_as_array_views_a_tuple_or_an_array_header() {
        let mut e = logged();
        install_allocator(&mut e);
        let properties = e.mem.alloc(16);
        e.mem.set_u16(properties + 8, 0xfffe);
        stub(&mut e, MEMBER_TYPE_PROPERTIES, properties);
        e.register(TYPE_FROM_MEMBER_TYPE, |_, _| answer(0x30));
        let world = e.mem.alloc(0x28);
        // A tuple: the element is viewed in place with the member type's size as the stride.
        let tuples = array_native(&mut e, world, 0x7000, 4, 24, 0x19, 0x6000, 3);
        let inner = e.call(0x00db_bd90, &args![tuples, 2u32]).u32();
        assert_eq!(e.mem.u16(inner + 4), 0x2c);
        assert_eq!(e.mem.u32(inner), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(inner + 0xc), world);
        assert_eq!(e.mem.u32(inner + 0x10), 0x7000 + 48);
        assert_eq!(e.mem.u32(inner + 0x14), 3);
        assert_eq!(e.mem.u32(inner + 0x18), 0xffff_fffe);
        assert_eq!(e.mem.u32(inner + 0x1c), 0x19);
        assert_eq!(e.mem.u32(inner + 0x20), 0x6000);
        assert_eq!(e.mem.u32(inner + 0x28), 0);
        assert_eq!(calls_to(&e, MEMBER_TYPE_PROPERTIES), vec![vec![0x19]]);
        // No tuple: the element is an array header (data, count).
        let data = e.mem.alloc(16);
        e.mem.set_u32(data + 8, 0x7100);
        e.mem.set_u32(data + 12, 6);
        let arrays = array_native(&mut e, world, data, 2, 8, 0x16, 0, 0);
        let inner = e.call(0x00db_bd90, &args![arrays, 1u32]).u32();
        assert_eq!(e.mem.u32(inner + 0xc), world);
        assert_eq!(e.mem.u32(inner + 0x10), 0x7100);
        assert_eq!(e.mem.u32(inner + 0x14), 6);
        assert_eq!(e.mem.u32(inner + 0x18), 0);
        assert_eq!(e.mem.u32(inner + 0x1c), 0);
        assert_eq!(e.mem.u32(inner + 0x20), 0);
        assert_eq!(calls_to(&e, MEMBER_TYPE_PROPERTIES).len(), 1);
    }

    #[test]
    fn object_native_get_member_name_reads_the_name_word() {
        let mut e = logged();
        let member = member_record(&mut e, 0x9100, 0, 0, 0);
        e.register_double(CLASS_GET_MEMBER, move |_, a| {
            answer(if a[1] == 2 { member } else { 0 })
        });
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        assert_eq!(e.call(0x00db_be30, &args![this, 2u32]).u32(), 0x9100);
        assert_eq!(calls_to(&e, CLASS_GET_MEMBER), vec![vec![0x6000, 2]]);
    }

    #[test]
    fn ref_counted_destructor_frees_the_block_when_bit_zero_is_set() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x18);
        assert_eq!(e.call(0x00db_be50, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), REF_COUNTED_VTABLE);
        assert!(calls_to(&e, FREE_SLOT_DOUBLE).is_empty());
        assert_eq!(e.call(0x00db_be50, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, FREE_SLOT_DOUBLE),
            vec![vec![allocator, this, 0x18]]
        );
    }

    #[test]
    fn fill_member_info_describes_a_member() {
        let mut e = logged();
        install_class_names(&mut e);
        let world = finding_world(&mut e);
        let this = class_native(&mut e, 0x6000, world);
        e.register(MEMBER_GET_ARRAY_SIZE, |_, _| answer(3));
        e.register(TYPE_FROM_MEMBER_TYPE, |_, _| answer(0x77));
        let out = e.mem.alloc(0x14);
        // A flagged member: only the name and the class itself.
        let flagged = member_record(&mut e, 0x9100, 4, 0, 0x400);
        for offset in [8, 0xc, 0x10] {
            e.mem.set_u32(out + offset, 0xffff);
        }
        assert_eq!(e.call(0x00db_be80, &args![this, flagged, out]).u32(), 0);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0x9100, this, 0, 0, 0]);
        assert!(calls_to(&e, TYPE_FROM_MEMBER_TYPE).is_empty());
        // A member without a class: no class lookup.
        let plain = member_record(&mut e, 0x9200, 4, 5, 0);
        stub(&mut e, MEMBER_GET_CLASS, 0);
        assert_eq!(e.call(0x00db_be80, &args![this, plain, out]).u32(), 3);
        let words: Vec<u32> = (0..5).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0x9200, this, 0x77, 0, 3]);
        assert_eq!(calls_to(&e, TYPE_FROM_MEMBER_TYPE), vec![vec![4, 5, 0, 3]]);
        // A member with a class but no attribute: the class is found by its own name.
        let classed = member_record(&mut e, 0x9300, 0x19, 0, 0);
        stub(&mut e, MEMBER_GET_CLASS, 0x6400);
        stub(&mut e, MEMBER_GET_ATTRIBUTE, 0);
        e.mem.set_u32(DATA_OBJECT_TYPE_ATTRIBUTE, 0xaaaa);
        assert_eq!(e.call(0x00db_be80, &args![this, classed, out]).u32(), 3);
        assert_eq!((e.mem.u32(out + 0xc), e.mem.u32(out + 0x10)), (0xb500, 3));
        assert_eq!(
            calls_to(&e, MEMBER_GET_ATTRIBUTE),
            vec![vec![classed, 0xaaaa]]
        );
        // With the attribute the class is found by the `typeName` string.
        let attribute = e.mem.alloc(8);
        stub(&mut e, MEMBER_GET_ATTRIBUTE, attribute);
        stub(&mut e, ACCESSOR_INIT, 0);
        let name_cell = e.mem.alloc(4);
        e.mem.set_u32(name_cell, 0x8100);
        stub(&mut e, ACCESSOR_AS_CSTRING, name_cell);
        assert_eq!(e.call(0x00db_be80, &args![this, classed, out]).u32(), 3);
        assert_eq!(e.mem.u32(out + 0xc), 0x8200);
        let init = calls_to(&e, ACCESSOR_INIT);
        assert_eq!(init[0][1..], [attribute, TYPE_NAME_TEXT]);
        assert_eq!(calls_to(&e, ACCESSOR_AS_CSTRING)[0][1], 0);
    }

    #[test]
    fn declared_member_info_and_member_info_fill_the_record() {
        let mut e = logged();
        let this = class_native(&mut e, 0x6000, 0);
        let member = member_record(&mut e, 0x9100, 4, 0, 0x400);
        e.register_double(CLASS_GET_DECLARED_MEMBER, move |_, a| {
            answer(if a[1] == 1 { member } else { 0 })
        });
        e.register_double(CLASS_GET_MEMBER, move |_, a| {
            answer(if a[1] == 2 { member } else { 0 })
        });
        let out = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00db_bfa0, &args![this, 1u32, out]).u32(), 0);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x9100, this));
        assert_eq!(
            calls_to(&e, CLASS_GET_DECLARED_MEMBER),
            vec![vec![0x6000, 1]]
        );
        e.mem.set_u32(out, 0);
        assert_eq!(e.call(0x00db_bfd0, &args![this, 2u32, out]).u32(), 0);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x9100, this));
        assert_eq!(calls_to(&e, CLASS_GET_MEMBER), vec![vec![0x6000, 2]]);
    }

    #[test]
    fn array_native_as_object_wraps_pointers_structs_and_variants() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        install_has_vtable(&mut e, &[]);
        let pairs = install_pair_copy(&mut e);
        // The world registered class 0x6500 for every name.
        let registered = e.mem.alloc(0x20);
        e.mem.set_u32(registered + 0xc, 0x6500);
        e.register_double(FIND_CLASS_DOUBLE, move |_, _| answer(registered));
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        let data = e.mem.alloc(32);
        e.mem.set_u32(data + 8, 0x7000);
        e.mem.set_u32(data + 16, 0x7100);
        e.mem.set_u32(data + 20, 0x6600);
        let object_at = |e: &mut Engine, member_type: u32, class: u32, index: u32| {
            let array = array_native(e, world, data, 3, 8, member_type, class, 0);
            e.call(0x00db_c000, &args![array, index]).u32()
        };
        // A pointer: the pointer in the element, with the array's class.
        let made = object_at(&mut e, 0x14, 0x6000, 1);
        assert_ne!(made, 0);
        assert_eq!(pairs.borrow().last(), Some(&(0x7000, 0x6500)));
        assert_eq!(e.mem.u32(made + 0x14), world);
        // A struct: the element's address.
        object_at(&mut e, 0x19, 0x6000, 1);
        assert_eq!(pairs.borrow().last(), Some(&(data + 8, 0x6500)));
        // A variant: the pair in the element.
        object_at(&mut e, 0x1c, 0, 2);
        assert_eq!(pairs.borrow().last(), Some(&(0x7100, 0x6500)));
        let count = pairs.borrow().len();
        // Other member types, null objects and null classes give nothing.
        assert_eq!(object_at(&mut e, 0x15, 0x6000, 1), 0);
        assert_eq!(object_at(&mut e, 0x14, 0x6000, 0), 0);
        assert_eq!(object_at(&mut e, 0x14, 0, 1), 0);
        assert_eq!(object_at(&mut e, 0x1c, 0, 0), 0);
        assert_eq!(pairs.borrow().len(), count);
    }

    #[test]
    fn array_native_swizzle_object_member_views_a_member_of_every_element() {
        let mut e = logged();
        install_allocator(&mut e);
        let world = e.mem.alloc(0x28);
        e.register(TYPE_FROM_MEMBER_TYPE, |_, _| answer(0x30));
        e.register(MEMBER_GET_ARRAY_SIZE, |_, _| answer(2));
        let member_class = 0x6700;
        stub(&mut e, MEMBER_GET_CLASS, member_class);
        let member = member_record(&mut e, 0x9100, 0x0b, 0, 0);
        e.mem.set_u16(member + 0x12, 0x14);
        let enum_member = member_record(&mut e, 0x9100, 0x18, 4, 0);
        e.mem.set_u16(enum_member + 0x12, 0x20);
        e.register_double(CLASS_GET_MEMBER_BY_NAME, move |_, a| {
            answer(if a[1] == 1 { member } else { enum_member })
        });
        let array = array_native(&mut e, world, 0x7000, 5, 0x40, 0x19, 0x6000, 0);
        let view = e.call(0x00db_c0c0, &args![array, 1u32]).u32();
        assert_eq!(e.mem.u16(view + 4), 0x2c);
        assert_eq!(e.mem.u32(view + 0xc), world);
        assert_eq!(e.mem.u32(view + 0x10), 0x7014);
        assert_eq!(e.mem.u32(view + 0x14), 5);
        assert_eq!(e.mem.u32(view + 0x18), 0x40);
        assert_eq!(e.mem.u32(view + 0x1c), 0x0b);
        assert_eq!(e.mem.u32(view + 0x20), member_class);
        assert_eq!(e.mem.u32(view + 0x28), 2);
        assert_eq!(
            calls_to(&e, CLASS_GET_MEMBER_BY_NAME),
            vec![vec![0x6000, 1]]
        );
        // An enum member is viewed by its subtype.
        let view = e.call(0x00db_c0c0, &args![array, 2u32]).u32();
        assert_eq!(e.mem.u32(view + 0x10), 0x7020);
        assert_eq!(e.mem.u32(view + 0x1c), 4);
    }

    /// Doubles for an accessor over a member of the given type and subtype whose typed
    /// address functions all return `value_address`.
    fn install_member_accessor(e: &mut Engine, member: u32, value_address: u32) {
        install_accessor(e);
        stub(e, ACCESSOR_GET_CLASS_MEMBER, member);
        for address in [
            ACCESSOR_ADDRESS_FLOAT,
            ACCESSOR_ADDRESS_HALF,
            ACCESSOR_ADDRESS_I32,
            ACCESSOR_ADDRESS_U32,
            ACCESSOR_ADDRESS_I64,
            ACCESSOR_ADDRESS_U64,
            ACCESSOR_ADDRESS_ULONG,
            ACCESSOR_ADDRESS_I16,
            ACCESSOR_ADDRESS_U16,
            ACCESSOR_ADDRESS_I8,
            ACCESSOR_ADDRESS_U8,
            ACCESSOR_AS_CSTRING,
            ACCESSOR_AS_STRING_PTR,
        ] {
            stub(e, address, value_address);
        }
    }

    #[test]
    fn object_native_as_array_chooses_the_view_by_member_type() {
        let mut e = logged();
        install_allocator(&mut e);
        install_class_names(&mut e);
        let world = e.mem.alloc(0x28);
        let properties = e.mem.alloc(16);
        e.mem.set_u16(properties + 8, 4);
        stub(&mut e, MEMBER_TYPE_PROPERTIES, properties);
        e.register(TYPE_FROM_MEMBER_TYPE, |_, _| answer(0x30));
        e.register(MEMBER_GET_ARRAY_SIZE, |_, _| answer(3));
        stub(&mut e, MEMBER_GET_CLASS, 0x6300);
        stub(&mut e, CLASS_GET_OBJECT_SIZE, 0x50);
        let this = object_native(&mut e, 0x7000, 0x6000, world);
        let header = e.mem.alloc(16);
        e.mem.set_u32(header, 0x7200);
        e.mem.set_u32(header + 4, 9);
        e.mem.set_u32(header + 8, 7);
        let run = |e: &mut Engine, member_type: u8, subtype: u8| {
            let member = member_record(e, 0x9100, member_type, subtype, 0);
            install_member_accessor(e, member, 0);
            stub(e, ACCESSOR_GET_ADDRESS, header);
            let array = e.call(0x00db_c150, &args![this, 0x1234u32]).u32();
            let words: Vec<u32> = [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x28]
                .iter()
                .map(|offset| e.mem.u32(array + offset))
                .collect();
            (array, words)
        };
        // A scalar member: its own address, the tuple size as the count, the type's size.
        let (array, words) = run(&mut e, 4, 0);
        assert_eq!(e.mem.u16(array + 4), 0x2c);
        assert_eq!(words, vec![world, header, 3, 4, 4, 0x6300, 0]);
        // A struct member: the class's object size.
        let (_, words) = run(&mut e, 0x19, 0);
        assert_eq!(words, vec![world, header, 3, 0x50, 0x19, 0x6300, 0]);
        // An array member: the header's data and count with the subtype's size.
        let (_, words) = run(&mut e, 0x16, 5);
        assert_eq!(words, vec![world, 0x7200, 9, 4, 5, 0x6300, 0]);
        // An array of a subtype without a size takes the class's object size.
        e.mem.set_u16(properties + 8, 0xffff);
        let (_, words) = run(&mut e, 0x1a, 5);
        assert_eq!(words, vec![world, 0x7200, 9, 0x50, 5, 0x6300, 0]);
        // An array without a subtype is empty.
        let (_, words) = run(&mut e, 0x16, 0);
        assert_eq!(words, vec![world, 0, 0, 0, 0, 0, 0]);
        // A homogeneous array: count at +8, data at +4, and the class at +0 gives the
        // elements the type struct and the class's object size.
        let (_, words) = run(&mut e, 0x1b, 0);
        assert_eq!(words, vec![world, 9, 7, 0x50, 0x19, 0x7200, 0]);
        // Without a class at +0 nothing is typed.
        e.mem.set_u32(header, 0);
        let (_, words) = run(&mut e, 0x1b, 0);
        assert_eq!(words, vec![world, 9, 7, 0, 0, 0, 0]);
        // Any other type is empty.
        let (_, words) = run(&mut e, 0x30, 0);
        assert_eq!(words, vec![world, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn object_native_as_string_reads_c_strings_and_string_pointers() {
        let mut e = logged();
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x9001);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        let read = |e: &mut Engine, member_type: u8| {
            let member = member_record(e, 0x9100, member_type, 0, 0);
            install_member_accessor(e, member, cell);
            e.call(0x00db_c310, &args![this, 0x1234u32]).u32()
        };
        assert_eq!(read(&mut e, 0x1d), 0x9001);
        assert_eq!(read(&mut e, 0x21), 0x9000);
        assert_eq!(read(&mut e, 0x1c), 0);
        assert_eq!(calls_to(&e, ACCESSOR_AS_CSTRING)[0][1..], [0]);
        assert_eq!(calls_to(&e, ACCESSOR_AS_STRING_PTR)[0][1..], [0]);
    }

    #[test]
    fn object_native_as_int_reads_the_member_through_its_typed_address() {
        let mut e = logged();
        let cell = e.mem.alloc(8);
        e.mem.set_u64(cell, 0x8765_4321_8765_fff0);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        let read = |e: &mut Engine, member_type: u8, subtype: u8| {
            let member = member_record(e, 0x9100, member_type, subtype, 0);
            install_member_accessor(e, member, cell);
            e.call(0x00db_c370, &args![this, 0x1234u32]).u64()
        };
        for member_type in [1, 2, 3] {
            assert_eq!(read(&mut e, member_type, 0), 0xffff_ffff_ffff_fff0);
        }
        assert_eq!(read(&mut e, 4, 0), 0xf0);
        assert_eq!(read(&mut e, 5, 0), 0xffff_ffff_ffff_fff0);
        assert_eq!(read(&mut e, 6, 0), 0xfff0);
        // A signed 32-bit value is sign-extended, an unsigned one is not.
        assert_eq!(read(&mut e, 7, 0), 0xffff_ffff_8765_fff0);
        assert_eq!(read(&mut e, 8, 0), 0x8765_fff0);
        assert_eq!(read(&mut e, 9, 0), 0x8765_4321_8765_fff0);
        assert_eq!(read(&mut e, 10, 0), 0x8765_4321_8765_fff0);
        assert_eq!(read(&mut e, 0x1e, 0), 0x8765_fff0);
        // An enum or flags member is read as its subtype.
        assert_eq!(read(&mut e, 0x18, 4), 0xf0);
        assert_eq!(read(&mut e, 0x1f, 6), 0xfff0);
        assert_eq!(read(&mut e, 0x18, 0x40), 0);
        for member_type in [0, 11, 0x20] {
            assert_eq!(read(&mut e, member_type, 0), 0);
        }
        assert_eq!(calls_to(&e, ACCESSOR_ADDRESS_ULONG)[0][1..], [0]);
    }

    #[test]
    fn object_native_as_object_wraps_pointers_structs_and_variants() {
        let mut e = logged();
        install_allocator(&mut e);
        install_reporting(&mut e);
        install_class_names(&mut e);
        install_has_vtable(&mut e, &[]);
        let pairs = install_pair_copy(&mut e);
        let registered = e.mem.alloc(0x20);
        e.mem.set_u32(registered + 0xc, 0x6500);
        e.register_double(FIND_CLASS_DOUBLE, move |_, _| answer(registered));
        let world = object_with(&mut e, &[(0x24, FIND_CLASS_DOUBLE)]);
        let this = object_native(&mut e, 0x7000, 0x6000, world);
        stub(&mut e, MEMBER_GET_CLASS, 0x6300);
        let pointer_cell = e.mem.alloc(4);
        e.mem.set_u32(pointer_cell, 0x7100);
        let variant = e.mem.alloc(8);
        e.mem.set_u32(variant, 0x7200);
        e.mem.set_u32(variant + 4, 0x6400);
        let run = |e: &mut Engine, member_type: u8, pointer: u32| {
            let member = member_record(e, 0x9100, member_type, 0, 0);
            install_accessor(e);
            stub(e, ACCESSOR_GET_CLASS_MEMBER, member);
            stub(e, ACCESSOR_AS_POINTER, pointer_cell);
            stub(e, ACCESSOR_GET_ADDRESS, 0x7300);
            stub(e, ACCESSOR_AS_VARIANT, variant);
            e.mem.set_u32(pointer_cell, pointer);
            e.call(0x00db_c4d0, &args![this, 0x1234u32]).u32()
        };
        assert_ne!(run(&mut e, 0x14, 0x7100), 0);
        assert_eq!(pairs.borrow().last(), Some(&(0x7100, 0x6500)));
        assert_ne!(run(&mut e, 0x19, 0), 0);
        assert_eq!(pairs.borrow().last(), Some(&(0x7300, 0x6500)));
        assert_ne!(run(&mut e, 0x1c, 0), 0);
        assert_eq!(pairs.borrow().last(), Some(&(0x7200, 0x6500)));
        let count = pairs.borrow().len();
        assert_eq!(run(&mut e, 0x15, 0x7100), 0);
        assert_eq!(run(&mut e, 0x14, 0), 0);
        assert_eq!(pairs.borrow().len(), count);
        assert_eq!(calls_to(&e, ACCESSOR_AS_POINTER)[0][1..], [0]);
    }

    #[test]
    fn object_native_as_real_reads_floats_and_halves() {
        let mut e = logged();
        let cell = e.mem.alloc(4);
        let this = object_native(&mut e, 0x7000, 0x6000, 0);
        let read = |e: &mut Engine, member_type: u8, bits: u32| {
            let member = member_record(e, 0x9100, member_type, 0, 0);
            install_member_accessor(e, member, cell);
            e.mem.set_u32(cell, bits);
            e.call(0x00db_c5c0, &args![this, 0x1234u32]).f32().to_bits()
        };
        assert_eq!(read(&mut e, 0xb, 1.5f32.to_bits()), 1.5f32.to_bits());
        assert_eq!(read(&mut e, 0xb, 0x7f80_0001), 0x7fc0_0001);
        assert_eq!(read(&mut e, 0x20, 0xbf80), (-1.0f32).to_bits());
        assert_eq!(read(&mut e, 0x20, 0x3fc0), 1.5f32.to_bits());
        assert_eq!(read(&mut e, 0xc, 0x3fc0), 0);
        assert_eq!(calls_to(&e, ACCESSOR_ADDRESS_FLOAT)[0][1..], [0]);
        assert_eq!(calls_to(&e, ACCESSOR_ADDRESS_HALF)[0][1..], [0]);
    }

    #[test]
    fn object_native_get_next_member_adds_one() {
        let mut e = engine();
        assert_eq!(e.call(0x00dc_9520, &args![0x1000u32, 4u32]).u32(), 5);
        assert_eq!(e.call(0x00dc_9520, &args![0x1000u32, -1i32]).u32(), 0);
    }
}
