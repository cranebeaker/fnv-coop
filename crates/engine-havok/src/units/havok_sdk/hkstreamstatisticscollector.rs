//! `Havok SDK/hkstreamstatisticscollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ddfb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_f240`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ddfb40(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_f240, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// ---------------------------------------------------------------------------------------------
// The statistics collector's own functions.
//
// `hkStreamStatisticsCollector` extends `hkStatisticsCollector` (Xbox PDB, 0x14 bytes: the vtable,
// the referenced-object words, `m_clientData` at +8, `m_vtblRegistry` at +0xc and
// `m_includeUnmanagedArrayMemory` at +0x10; its vtable slots `+0xc` addReferencedObject, `+0x10`
// addObject, `+0x14` addChunk, `+0x18` pushDir, `+0x1c` popDir) with a byte stream of commands
// and a map of the objects already seen. A command is a 6-byte header (the type character at
// +0, the payload size at +1, two 16-bit numbers at +2 and +4) followed by the payload, a
// zero-terminated name padded to an even length: `'P'` pushes a node (`pushDir`,
// `_startObject`), `'C'` adds a chunk, `'p'` pops. `makeTree` replays the stream into a tree
// of `hkMonitorStreamAnalyzer::Node`s. Every command is only recorded while the stream has
// room: its capacity must exceed its size by more than `0x46`.
// ---------------------------------------------------------------------------------------------

/// The class's vtable and the base class's constructor (`00d06f50`,
/// thiscall, one stack word: the registry) and destructor (`00d06f70`, thiscall).
const COLLECTOR_VTABLE: u32 = 0x010d_8040;
const BASE_CONSTRUCTOR: u32 = 0x00d0_6f50;
const BASE_DESTRUCTOR: u32 = 0x00d0_6f70;
/// The pointer map of the objects seen (a `hkMapBase` with 32-bit keys and values at `this +
/// 0x20`): `clear` (`00cb1a60`, thiscall, no stack words), `clearAndDeallocate` (`00cb1a90`,
/// thiscall, one stack word: the allocator), `reserve` (`00cb1fe0`, thiscall, `(allocator,
/// count)`), `insert` (`00cb1870`, `(allocator, key, value)`) and `getWithDefault`
/// (`00cb1930`, `(key, default)`); and the empty map destructor (`00a29680`, a `RET`).
const MAP_CLEAR: u32 = 0x00cb_1a60;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`) and its
/// growth step for a full array (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// An array insert (`00d10710`, thiscall on the array, `(allocator, index, pointer to the
/// value)`).
const ARRAY_INSERT: u32 = 0x00d1_0710;
/// `hkString::strLen` (`00cb10d0`), `strCmp` (`00cb0fa0`) and `memCpy` (`00cb1260`).
const STR_LEN: u32 = 0x00cb_10d0;
const STR_CMP: u32 = 0x00cb_0fa0;
const MEM_CPY: u32 = 0x00cb_1260;
/// `hkStringPtr`: the constructor from a C string (`00cb0b80`, thiscall, one stack word), the
/// copy constructor (`00cb0bd0`, thiscall, one stack word: the other `hkStringPtr`) and the
/// destructor (`00cb0c00`, thiscall).
const STRING_PTR_FROM_TEXT: u32 = 0x00cb_0b80;
const STRING_PTR_COPY: u32 = 0x00cb_0bd0;
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
/// `hkMonitorStreamAnalyzer::Node`: the constructor (`00e0d000`, thiscall, `(parent, name,
/// flag)`, returns the node) and the destructor (`00e0d0a0`, thiscall). A node is `0x58`
/// bytes; the code reads: two floats at `+0` and `+4` (the totals), two 16-bit counts at `+0x20`
/// and `+0x22`, its children array at `+0x30` (data, size at `+0x34`, capacity word at
/// `+0x38`) and its name pointer at `+0x4c`.
const NODE_CONSTRUCTOR: u32 = 0x00e0_d000;
const NODE_DESTRUCTOR: u32 = 0x00e0_d0a0;
const NODE_SIZE: u32 = 0x58;
/// The names: `"self"` (a node name used when none is given, `010d8038`), the same word used
/// as the name of the summary child (`010d8060`) and `"/"` (the root's name).
const DEFAULT_NODE_NAME: u32 = 0x010d_8038;
const SELF_NODE_NAME: u32 = 0x010d_8060;
const ROOT_NODE_NAME: u32 = 0x0106_3cbc;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 (an `f64` argument here) to an integer.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `hkStatisticsCollector::getReferencedObjectSize` (`00d06d90`, thiscall, `(object)`) and the
/// unnamed `00d06d50` (thiscall, `(object)`), `hkClass::getObjectSize` (`00d470d0`, thiscall)
/// and `hkStatisticsCollectorUtil::addClassContentsAll` (`00cf6920`, cdecl `(object, class,
/// collector)`).
const GET_REFERENCED_OBJECT_SIZE: u32 = 0x00d0_6d90;
const REFERENCED_OBJECT_CONTENTS: u32 = 0x00d0_6d50;
const CLASS_GET_OBJECT_SIZE: u32 = 0x00d4_70d0;
const ADD_CLASS_CONTENTS_ALL: u32 = 0x00cf_6920;
/// The size bits of an `hkArray`'s capacity word, and the capacity word of an array with no
/// storage that must not be deallocated.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const EMPTY_ARRAY_CAPACITY: u32 = 0x8000_0000;
/// Commands are recorded only while `capacity - size` is above this.
const COMMAND_ROOM: i32 = 0x46;
/// The command header size and its type characters.
const HEADER_SIZE: i32 = 6;
const COMMAND_PUSH: u8 = 0x50;
const COMMAND_CHUNK: u8 = 0x43;
const COMMAND_POP: u8 = 0x70;
/// The `hkMemoryAllocator` slots used: `+4` allocate `(size)`, `+8` free `(block, size)`,
/// `+0xc` allocate with a granted size `(size pointer)`, `+0x10` free storage `(block, bytes)`
/// and `+0x2c` the 16-bit value `startObject` stores `(address, size)`.
const SLOT_ALLOCATE: u32 = 4;
const SLOT_FREE: u32 = 8;
const SLOT_ALLOCATE_BUFFER: u32 = 0xc;
const SLOT_FREE_BUFFER: u32 = 0x10;
const SLOT_OBJECT_ID: u32 = 0x2c;

layout! {
    /// `hkStreamStatisticsCollector` (engine map name; not in the Xbox PDB, which has only the
    /// base `hkStatisticsCollector`), 0x2c bytes: the base, then the command stream (an
    /// `hkArray<char>`) and the map of objects seen. The field names are descriptive and
    /// unconfirmed.
    pub struct HkStreamStatisticsCollector: 0x2c {
        /// The command stream's data.
        0x14 m_data: Ptr,
        /// Its size in bytes.
        0x18 m_size: i32,
        /// Its capacity word (bytes in the low 30 bits, bit 31 for storage it does not own).
        0x1C m_capacityAndFlags: u32,
        /// The object map's slots.
        0x20 m_mapElem: u32,
        /// The object map's element count word.
        0x24 m_mapNumElems: u32,
        /// The object map's `hashMod`.
        0x28 m_mapHashMod: i32,
    }
}

/// The calling thread's heap allocator, the word at router `+0x10`.
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// `array.capacity & 0x3fffffff` for the `hkArray` at `array`.
fn array_capacity(e: &Engine, array: u32) -> i32 {
    (e.mem.u32(array + 8) & CAPACITY_MASK) as i32
}

/// The exe's push of one pointer onto an `hkArray` of pointers at `array`: the router first,
/// the growth step (`00c90940(allocator, array, 4)`) when the array is full, then the
/// element and the size.
fn push_pointer(e: &mut Engine, array: u32, value: u32) {
    let allocator = heap_allocator(e);
    if e.mem.i32(array + 4) == array_capacity(e, array) {
        e.call(ARRAY_GROW, &args![allocator, array, 4i32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// `float_to_int` on a value an `FLD float` put in ST0.
fn truncate(e: &mut Engine, bits: u32) -> i32 {
    e.call(FLOAT_TO_INT, &args![x87_load32(bits)]).i32()
}

// Translated from 00ddeff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMonitorStreamAnalyzer::Node::_scalar_deleting_destructor_` (engine map name), thiscall,
/// one stack word `flags`: runs the node's destructor (`00e0d0a0`); when bit 0 of `flags` is
/// set and `this` is not null, frees the `0x58`-byte block through the heap allocator's slot
/// `+8`. Returns `this`.
pub fn hk_monitor_stream_analyzer_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.call(NODE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 && !this.is_null() {
        let allocator = heap_allocator(e);
        e.vcall(allocator, SLOT_FREE, &args![this, NODE_SIZE]);
    }
    this
}

// Translated from 00ddf020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::_addKnown` (engine map name), `__fastcall`: a jump to the
/// collector's own virtual slot `+0x1c` (`popDir`) with the same `this`.
pub fn hk_stream_statistics_collector_add_known(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x1c, &args![]);
}

// Translated from 00ddf030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall with one stack word that is not read: clears the object
/// map at `this + 0x20` (`00cb1a60`).
pub fn fn_00ddf030(e: &mut Engine, this: Ptr, _unused_0: u32) {
    e.call(MAP_CLEAR, &args![this.addr().wrapping_add(0x20)]);
}

// Translated from 00ddf040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `__fastcall`: a jump to the same map clear (`00cb1a60`) on
/// `this + 0x20`.
pub fn fn_00ddf040(e: &mut Engine, this: Ptr) {
    e.call(MAP_CLEAR, &args![this.addr().wrapping_add(0x20)]);
}

// Translated from 00ddf050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `__fastcall`: clears the object map at `this + 0x20` and sets the
/// command stream's size (`+0x18`) to 0.
pub fn fn_00ddf050(e: &mut Engine, this: Ptr<HkStreamStatisticsCollector>) {
    e.call(MAP_CLEAR, &args![this.addr().wrapping_add(0x20)]);
    e.set(this, HkStreamStatisticsCollector::m_size, 0);
}

// Translated from 00ddf070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_newNode` (engine map name), cdecl with register inputs: **the node's name is in `EAX` and
/// the array of names (an `hkArray<hkStringPtr>`) in `EBX`**, the first two parameters here,
/// then the stack words `(parent, first, second)`; the code removes them. Finds or makes the
/// child of `parent` with that name and adds the two counts to it.
///
/// A zero `second` is replaced by `first`; a null name by `"self"` (`010d8038`). When `parent`
/// is not null its children (`+0x30`, count at `+0x34`) are searched for one whose name word
/// (`+0x4c`) compares equal (`strCmp`); a found child that is null is not used. Otherwise,
/// when `names` and the name are non-null a copy of the name is appended to `names` (a
/// temporary `hkStringPtr` from the text, grown through `00c90940(allocator, array, 4)`, copy
/// constructed in place, the temporary destroyed) and the name used for the node becomes the
/// stored copy's pointer (its word without bit 0); a `0x58`-byte node is allocated through the
/// heap allocator's slot `+4` and constructed (`00e0d000(parent, name, 1)`; a failed
/// allocation gives a null node, which the exe then dereferences). The node's two 16-bit
/// counts (`+0x20`, `+0x22`) are incremented and `first` and `second` are added to its two
/// floats (`+0`, `+4`) through the x87 (`FILD`, `FADD`, `FSTP`). Returns the node.
pub fn new_node(e: &mut Engine, eax: Ptr, ebx: Ptr, parent: Ptr, first: i32, second: i32) -> Ptr {
    let second = if second == 0 { first } else { second };
    let mut name = eax.addr();
    if name == 0 {
        name = DEFAULT_NODE_NAME;
    }
    let mut node = 0u32;
    if !parent.is_null() {
        let mut index = 0i32;
        while index < e.mem.i32(parent.addr() + 0x34) {
            let children = e.mem.u32(parent.addr() + 0x30);
            let child = e
                .mem
                .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
            let child_name = e.mem.u32(child.wrapping_add(0x4c));
            if e.call(STR_CMP, &args![child_name, name]).i32() == 0 {
                let children = e.mem.u32(parent.addr() + 0x30);
                node = e
                    .mem
                    .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
                break;
            }
            index += 1;
        }
    }
    if node == 0 {
        if !ebx.is_null() && name != 0 {
            let names = ebx.addr();
            e.with_stack(4, |e, temporary| {
                e.call(STRING_PTR_FROM_TEXT, &args![temporary, name]);
                let allocator = heap_allocator(e);
                if e.mem.i32(names + 4) == array_capacity(e, names) {
                    e.call(ARRAY_GROW, &args![allocator, names, 4i32]);
                }
                let size = e.mem.u32(names + 4);
                let data = e.mem.u32(names);
                let slot = data.wrapping_add(size.wrapping_mul(4));
                if slot != 0 {
                    e.call(STRING_PTR_COPY, &args![slot, temporary]);
                }
                e.mem.set_u32(names + 4, size.wrapping_add(1));
                e.call(STRING_PTR_DESTRUCTOR, &args![temporary]);
            });
            let size = e.mem.u32(names + 4);
            let data = e.mem.u32(names);
            name = e
                .mem
                .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4))
                & 0xffff_fffe;
        }
        let allocator = heap_allocator(e);
        let block = e.vcall(allocator, SLOT_ALLOCATE, &args![NODE_SIZE]).u32();
        node = if block == 0 {
            0
        } else {
            e.call(NODE_CONSTRUCTOR, &args![block, parent, name, 1u32])
                .u32()
        };
    }
    let count = e.mem.u16(node.wrapping_add(0x20)).wrapping_add(1);
    e.mem.set_u16(node.wrapping_add(0x20), count);
    let total = x87_add(f64::from(first), x87_m32(e.mem.u32(node)));
    let count = e.mem.u16(node.wrapping_add(0x22)).wrapping_add(1);
    e.mem.set_u16(node.wrapping_add(0x22), count);
    e.mem.set_u32(node, x87_store32(total));
    let total = x87_add(f64::from(second), x87_m32(e.mem.u32(node.wrapping_add(4))));
    e.mem.set_u32(node.wrapping_add(4), x87_store32(total));
    Ptr::new(node)
}

// Translated from 00ddf170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::hkStreamStatisticsCollector` (engine map name), thiscall, two
/// stack words `(registry, buffer size)`: the base constructor (`00d06f50(registry)`), the
/// class's vtable, and the command stream: a block of `buffer size` bytes from the heap
/// allocator's slot `+0xc` (`allocate(&size)`; the size is in/out), whose granted size
/// becomes the stream's size word and capacity (a zero granted size, or no buffer size at
/// all, leaves the capacity word `0x80000000`); the object map is emptied and reserved
/// for 0 elements (`00cb1fe0(allocator, 0)`), then the stream's size is 0. (The exe also
/// has a growth step of the stream, `_reserve`, that is dead: the masked capacity is never
/// negative; it is kept below.) Returns `this`.
pub fn hk_stream_statistics_collector_hk_stream_statistics_collector(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    registry: u32,
    buffer_size: i32,
) -> Ptr<HkStreamStatisticsCollector> {
    e.call(BASE_CONSTRUCTOR, &args![this, registry]);
    e.mem.set_u32(this.addr(), COLLECTOR_VTABLE);
    e.set(this, HkStreamStatisticsCollector::m_data, Ptr::NULL);
    e.set(this, HkStreamStatisticsCollector::m_size, 0);
    e.set(
        this,
        HkStreamStatisticsCollector::m_capacityAndFlags,
        EMPTY_ARRAY_CAPACITY,
    );
    let allocator = heap_allocator(e);
    let (data, size, capacity) = if buffer_size == 0 {
        (0u32, 0i32, EMPTY_ARRAY_CAPACITY)
    } else {
        e.with_stack(4, |e, request| {
            e.mem.set_i32(request.addr(), buffer_size);
            let block = e
                .vcall(allocator, SLOT_ALLOCATE_BUFFER, &args![request])
                .u32();
            let granted = e.mem.i32(request.addr());
            let capacity = if granted == 0 {
                EMPTY_ARRAY_CAPACITY
            } else {
                granted as u32
            };
            (block, granted, capacity)
        })
    };
    e.set(this, HkStreamStatisticsCollector::m_size, size);
    e.set(this, HkStreamStatisticsCollector::m_data, Ptr::new(data));
    e.set(
        this,
        HkStreamStatisticsCollector::m_capacityAndFlags,
        capacity,
    );
    e.set(this, HkStreamStatisticsCollector::m_mapElem, 0);
    e.set(this, HkStreamStatisticsCollector::m_mapNumElems, 0);
    e.set(this, HkStreamStatisticsCollector::m_mapHashMod, -1);
    let map = this.addr().wrapping_add(0x20);
    let allocator = heap_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    let allocator = heap_allocator(e);
    let masked = e.mem.i32(this.addr() + 0x1c) & CAPACITY_MASK as i32;
    if masked < 0 {
        // Unreachable in the exe: the masked capacity is never negative.
        let doubled = masked.wrapping_add(masked);
        let count = if doubled <= 0 { 0 } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, this.addr().wrapping_add(0x14), count, 1i32],
        );
    }
    e.set(this, HkStreamStatisticsCollector::m_size, 0);
    this
}

// Translated from 00ddf240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::~hkStreamStatisticsCollector` (engine map name),
/// `__fastcall`: sets the class's vtable, empties the object map and gives its slots back
/// (`00cb1a90(allocator)` with the heap allocator, then the empty destructor `00a29680`),
/// sets the stream's size to 0, frees the stream's storage through the allocator's slot
/// `+0x10` when it owns it, resets its data and capacity word, and ends with the base
/// destructor (`00d06f70`).
pub fn hk_stream_statistics_collector_dtor(e: &mut Engine, this: Ptr<HkStreamStatisticsCollector>) {
    e.mem.set_u32(this.addr(), COLLECTOR_VTABLE);
    let map = this.addr().wrapping_add(0x20);
    let allocator = heap_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_DESTRUCTOR, &args![map]);
    let allocator = heap_allocator(e);
    let capacity_word = e.get(this, HkStreamStatisticsCollector::m_capacityAndFlags);
    e.set(this, HkStreamStatisticsCollector::m_size, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.get(this, HkStreamStatisticsCollector::m_data);
        e.vcall(
            allocator,
            SLOT_FREE_BUFFER,
            &args![data, capacity_word & CAPACITY_MASK],
        );
    }
    e.set(this, HkStreamStatisticsCollector::m_data, Ptr::NULL);
    e.set(
        this,
        HkStreamStatisticsCollector::m_capacityAndFlags,
        EMPTY_ARRAY_CAPACITY,
    );
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00ddf2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (replaces the command stream with a copy of `size` bytes at
/// `source`), thiscall, three stack words `(source, size, swap)`: when the capacity (masked) is
/// below `size` the stream grows to the larger of `size` and twice its capacity
/// (`_reserve`, heap allocator); the size becomes `size` and the bytes are copied
/// (`memCpy`). A non-zero `swap` (the low byte) then walks the commands (header 6 bytes plus
/// the payload size byte at `+1`) while the offset is below the stream's size and swaps the
/// bytes of the two 16-bit numbers at `+2` and `+4` of each header.
pub fn fn_00ddf2a0(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    source: Ptr,
    size: i32,
    swap: u8,
) {
    let array = this.addr().wrapping_add(0x14);
    let allocator = heap_allocator(e);
    let capacity = array_capacity(e, array);
    if capacity < size {
        let doubled = capacity.wrapping_add(capacity);
        let count = if size < doubled { doubled } else { size };
        e.call(ARRAY_RESERVE, &args![allocator, array, count, 1i32]);
    }
    e.mem.set_i32(array + 4, size);
    let data = e.mem.u32(array);
    e.call(MEM_CPY, &args![data, source, size]);
    if swap != 0 {
        let mut offset = 0i32;
        if e.get(this, HkStreamStatisticsCollector::m_size) > offset {
            loop {
                let data = e.mem.u32(array);
                let header = data.wrapping_add(offset as u32);
                let first = e.mem.u16(header.wrapping_add(2));
                e.mem.set_u16(header.wrapping_add(2), first.swap_bytes());
                let second = e.mem.u16(header.wrapping_add(4));
                e.mem.set_u16(header.wrapping_add(4), second.swap_bytes());
                let payload = i32::from(e.mem.u8(header.wrapping_add(1)));
                offset = offset.wrapping_add(payload).wrapping_add(HEADER_SIZE);
                if offset >= e.get(this, HkStreamStatisticsCollector::m_size) {
                    break;
                }
            }
        }
    }
}

// Translated from 00ddf370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_newCommand` (engine map name), cdecl with a register input: **the command stream (an
/// `hkArray<char>`) is in `ESI`**, the first parameter here, and the stack word is the payload
/// size. Returns the address where the next command starts (the stream's data plus its
/// size, taken before the stream grows, as the exe does) and extends the stream by `payload +
/// 6` bytes: an empty stream is first given room for one byte (the growth step `00c90940`
/// when full) and its size is reset to 0; then the capacity grows to the larger of the new size
/// and twice the capacity (`_reserve`) when it is below the new size.
pub fn new_command(e: &mut Engine, esi: Ptr, payload: i32) -> u32 {
    let array = esi.addr();
    let address = if e.mem.i32(array + 4) > 0 {
        e.mem.u32(array).wrapping_add(e.mem.u32(array + 4))
    } else {
        let allocator = heap_allocator(e);
        if e.mem.i32(array + 4) == array_capacity(e, array) {
            e.call(ARRAY_GROW, &args![allocator, array, 1i32]);
        }
        let size = e.mem.u32(array + 4);
        let start = e.mem.u32(array).wrapping_add(size);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        let allocator = heap_allocator(e);
        let masked = array_capacity(e, array);
        if masked < 0 {
            // Unreachable in the exe: the masked capacity is never negative.
            let doubled = masked.wrapping_add(masked);
            let count = if doubled <= 0 { 0 } else { doubled };
            e.call(ARRAY_RESERVE, &args![allocator, array, count, 1i32]);
        }
        e.mem.set_u32(array + 4, 0);
        start
    };
    let size = e.mem.i32(array + 4);
    let new_size = payload.wrapping_add(size).wrapping_add(HEADER_SIZE);
    let allocator = heap_allocator(e);
    let capacity = array_capacity(e, array);
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let count = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, count, 1i32]);
    }
    e.mem.set_i32(array + 4, new_size);
    address
}

/// The text of a command's name as the exe sizes it: `strLen + 1` bytes (`size`), and the
/// padding byte (`size % 2`, a signed remainder) that makes the payload even. A null name has
/// neither.
fn name_sizes(e: &mut Engine, name: Ptr) -> (i32, i32) {
    if name.is_null() {
        return (0, 0);
    }
    let length = e.call(STR_LEN, &args![name]).i32();
    let size = length.wrapping_add(1);
    (size, size % 2)
}

/// Writes a command's header and payload at `command`: the type character, the payload size
/// byte `(size + padding) & 0xff`, the two 16-bit numbers, then (when the name has bytes) the
/// name copied with `memCpy` and a zero padding byte when there is padding.
#[allow(clippy::too_many_arguments)] // the record header and payload, as the exe writes them
fn write_command(
    e: &mut Engine,
    command: u32,
    kind: u8,
    first: u16,
    second: u16,
    name: Ptr,
    size: i32,
    padding: i32,
) {
    e.mem.set_u8(command, kind);
    e.mem
        .set_u8(command.wrapping_add(1), (size.wrapping_add(padding)) as u8);
    e.mem.set_u16(command.wrapping_add(2), first);
    e.mem.set_u16(command.wrapping_add(4), second);
    if size > 0 {
        e.call(
            MEM_CPY,
            &args![command.wrapping_add(HEADER_SIZE as u32), name, size],
        );
        if padding != 0 {
            e.mem.set_u8(
                command
                    .wrapping_add(size as u32)
                    .wrapping_add(HEADER_SIZE as u32),
                0,
            );
        }
    }
}

/// Whether the stream has room for another command: its capacity (masked) is above its size
/// plus `0x46` (a signed compare).
fn has_room(e: &Engine, this: Ptr<HkStreamStatisticsCollector>) -> bool {
    let size = e.get(this, HkStreamStatisticsCollector::m_size);
    (e.get(this, HkStreamStatisticsCollector::m_capacityAndFlags) & CAPACITY_MASK) as i32
        > size.wrapping_add(COMMAND_ROOM)
}

// Translated from 00ddf420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::_startObject` (engine map name), thiscall, three stack words
/// `(name, address, size)`: records a push command (`'P'`) for the object: the payload is the
/// name with its terminator and a pad byte to an even size (a null name has none), the first
/// 16-bit number is `size` (low 16 bits) and the second is 0 for a zero `size` and otherwise
/// the low 16 bits of the heap allocator's slot `+0x2c` called with `(address, size)`.
pub fn hk_stream_statistics_collector_start_object(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    name: Ptr,
    address: u32,
    size: i32,
) {
    let (name_size, padding) = name_sizes(e, name);
    let command = new_command(
        e,
        Ptr::new(this.addr().wrapping_add(0x14)),
        padding.wrapping_add(name_size),
    );
    let second = if size == 0 {
        0
    } else {
        let allocator = heap_allocator(e);
        e.vcall(allocator, SLOT_OBJECT_ID, &args![address, size])
            .u16()
    };
    write_command(
        e,
        command,
        COMMAND_PUSH,
        size as u16,
        second,
        name,
        name_size,
        padding,
    );
}

// Translated from 00ddf4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::addChunk` (Xbox PDB name), thiscall, five stack words `(a1,
/// name, address, used, size)`; `a1` is not read. Nothing happens unless the stream has room
/// (`has_room`) and `used` or `size` is non-zero. Otherwise records a chunk command (`'C'`)
/// like `_startObject`: the payload is the name, the first 16-bit number is `used` (low 16
/// bits) and the second is 0 for a zero `size` and otherwise the low 16 bits of the heap
/// allocator's slot `+0x2c` called with `(address, size)`.
pub fn hk_stream_statistics_collector_add_chunk(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    _unused_0: u32,
    name: Ptr,
    address: u32,
    used: u32,
    size: i32,
) {
    if !has_room(e, this) || (used == 0 && size == 0) {
        return;
    }
    let (name_size, padding) = name_sizes(e, name);
    let command = new_command(
        e,
        Ptr::new(this.addr().wrapping_add(0x14)),
        padding.wrapping_add(name_size),
    );
    let second = if size == 0 {
        0
    } else {
        let allocator = heap_allocator(e);
        e.vcall(allocator, SLOT_OBJECT_ID, &args![address, size])
            .u16()
    };
    write_command(
        e,
        command,
        COMMAND_CHUNK,
        used as u16,
        second,
        name,
        name_size,
        padding,
    );
}

// Translated from 00ddf5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::pushDir` (Xbox PDB name), thiscall, one stack word `name`:
/// when the stream has room, records a push command (`'P'`) with the name (even-sized
/// payload) and both 16-bit numbers 0.
pub fn hk_stream_statistics_collector_push_dir(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    name: Ptr,
) {
    if !has_room(e, this) {
        return;
    }
    let (name_size, padding) = name_sizes(e, name);
    let command = new_command(
        e,
        Ptr::new(this.addr().wrapping_add(0x14)),
        padding.wrapping_add(name_size),
    );
    write_command(e, command, COMMAND_PUSH, 0, 0, name, name_size, padding);
}

// Translated from 00ddf640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::popDir` (Xbox PDB name), `__fastcall`: when the stream has
/// room, records a pop command (`'p'`) with no payload and both 16-bit numbers 0.
pub fn hk_stream_statistics_collector_pop_dir(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
) {
    if !has_room(e, this) {
        return;
    }
    let command = new_command(e, Ptr::new(this.addr().wrapping_add(0x14)), 0);
    e.mem.set_u8(command, COMMAND_POP);
    e.mem.set_u8(command.wrapping_add(1), 0);
    e.mem.set_u16(command.wrapping_add(2), 0);
    e.mem.set_u16(command.wrapping_add(4), 0);
}

// Translated from 00ddf680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_sumNodes` (engine map name), cdecl `(node, names)` (`names` is the `hkArray<hkStringPtr>`
/// `_newNode` appends to; it travels in `EBX`): does nothing for a null node or one with no
/// children. Otherwise it first does the same for every child. When the node's first total
/// (`+0`) is not 0.0 (a NaN counts as non-zero; the code's `FCOMP` + `TEST AH,0x44` + `JNP`)
/// it makes a child named by the word `010d8060` holding the node's own two totals truncated to
/// integers (`_ftol2` of `+4`, then of `+0`; `_newNode(names, node, int(+0), int(+4))`); when
/// that child is the node's last child it is removed from the end and inserted at index 0
/// (`00d10710(allocator, 0, &child)`, heap allocator). Then the node's two totals are cleared
/// and set to the sums of its children's (`FLD`, `FADD`, `FSTP`, in order).
pub fn sum_nodes(e: &mut Engine, node: Ptr, names: Ptr) {
    if node.is_null() {
        return;
    }
    let node_address = node.addr();
    let child_count = e.mem.i32(node_address + 0x34);
    if child_count == 0 {
        return;
    }
    let mut index = 0i32;
    if child_count > 0 {
        loop {
            let children = e.mem.u32(node_address + 0x30);
            let child = e
                .mem
                .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
            sum_nodes(e, Ptr::new(child), names);
            index += 1;
            if index >= e.mem.i32(node_address + 0x34) {
                break;
            }
        }
    }
    let first_bits = e.mem.u32(node_address);
    if f32::from_bits(first_bits) != 0.0 {
        let second_bits = e.mem.u32(node_address + 4);
        let second = truncate(e, second_bits);
        let first_bits = e.mem.u32(node_address);
        let first = truncate(e, first_bits);
        let own = new_node(e, Ptr::new(SELF_NODE_NAME), names, node, first, second);
        let size = e.mem.i32(node_address + 0x34);
        let children = e.mem.u32(node_address + 0x30);
        if e.mem.u32(
            children
                .wrapping_add((size as u32).wrapping_mul(4))
                .wrapping_sub(4),
        ) == own.addr()
        {
            e.mem.set_i32(node_address + 0x34, size.wrapping_sub(1));
            let allocator = heap_allocator(e);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), own.addr());
                e.call(
                    ARRAY_INSERT,
                    &args![node_address + 0x30, allocator, 0u32, slot],
                );
            });
        }
    }
    e.mem.set_u32(node_address, 0);
    e.mem.set_u32(node_address + 4, 0);
    let mut index = 0i32;
    if e.mem.i32(node_address + 0x34) > index {
        loop {
            let children = e.mem.u32(node_address + 0x30);
            let child = e
                .mem
                .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
            index += 1;
            let first = x87_add(
                x87_load32(e.mem.u32(child)),
                x87_m32(e.mem.u32(node_address)),
            );
            e.mem.set_u32(node_address, x87_store32(first));
            let second = x87_add(
                x87_load32(e.mem.u32(child + 4)),
                x87_m32(e.mem.u32(node_address + 4)),
            );
            e.mem.set_u32(node_address + 4, x87_store32(second));
            if index >= e.mem.i32(node_address + 0x34) {
                break;
            }
        }
    }
}

/// Frees a node: its destructor (`00e0d0a0`) and the `0x58`-byte block through the heap
/// allocator's slot `+8`; nothing for a null node.
fn destroy_node(e: &mut Engine, node: u32) {
    if node != 0 {
        e.call(NODE_DESTRUCTOR, &args![node]);
        let allocator = heap_allocator(e);
        e.vcall(allocator, SLOT_FREE, &args![node, NODE_SIZE]);
    }
}

/// Releases a local `hkArray<pointer>`: size 0 and, when it owns its storage, a free through
/// the heap allocator's slot `+0x10` of `capacity * 4` bytes.
fn release_pointer_array(e: &mut Engine, array: u32) {
    let allocator = heap_allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            SLOT_FREE_BUFFER,
            &args![data, (capacity_word & CAPACITY_MASK).wrapping_mul(4)],
        );
    }
}

// Translated from 00ddf740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::makeTree` (engine map name), thiscall, two stack words
/// `(roots, names)`: replays the command stream into nodes. `roots` is an `hkArray<Node*>`:
/// with no element a root node is made (`"/"`, parent 0, flag 1) and pushed; with elements
/// all but the first are destroyed from the end, and the first is the root. A local stack of
/// nodes starts with the root. Each command, in order (the next is at the offset plus the
/// payload size plus 6; the name is the payload address or null for an empty payload):
/// `'C'` adds a node under the top (`_newNode(name, names, top, number at +2, number at +4)`),
/// `'P'` does the same and pushes the result, `'p'` pops, and any other type stops the replay
/// at once, skipping the totals. Otherwise `sumNodes(root, names)` runs. The local stack's
/// storage is released.
pub fn hk_stream_statistics_collector_make_tree(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    roots: Ptr,
    names: Ptr,
) {
    let roots_address = roots.addr();
    e.with_stack(0xc, |e, stack| {
        let stack_address = stack.addr();
        e.mem.set_u32(stack_address, 0);
        e.mem.set_u32(stack_address + 4, 0);
        e.mem.set_u32(stack_address + 8, EMPTY_ARRAY_CAPACITY);
        let root = if e.mem.i32(roots_address + 4) <= 0 {
            let allocator = heap_allocator(e);
            let block = e.vcall(allocator, SLOT_ALLOCATE, &args![NODE_SIZE]).u32();
            let made = if block == 0 {
                0
            } else {
                e.call(NODE_CONSTRUCTOR, &args![block, 0u32, ROOT_NODE_NAME, 1u32])
                    .u32()
            };
            push_pointer(e, roots_address, made);
            made
        } else {
            let first = e.mem.u32(e.mem.u32(roots_address));
            if e.mem.i32(roots_address + 4) > 1 {
                loop {
                    let size = e.mem.u32(roots_address + 4);
                    let data = e.mem.u32(roots_address);
                    let last = e
                        .mem
                        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
                    destroy_node(e, last);
                    let size = e.mem.i32(roots_address + 4);
                    e.mem.set_i32(roots_address + 4, size.wrapping_sub(1));
                    if e.mem.i32(roots_address + 4) <= 1 {
                        break;
                    }
                }
            }
            first
        };
        push_pointer(e, stack_address, root);
        let mut offset = 0i32;
        let mut finished = true;
        if e.get(this, HkStreamStatisticsCollector::m_size) > offset {
            loop {
                let data = e.get(this, HkStreamStatisticsCollector::m_data).addr();
                let command = data.wrapping_add(offset as u32);
                let payload = e.mem.u8(command.wrapping_add(1));
                let name = if payload > 0 {
                    command.wrapping_add(HEADER_SIZE as u32)
                } else {
                    0
                };
                let advance = i32::from(payload) + HEADER_SIZE;
                let kind = e.mem.u8(command);
                let top_of_stack = |e: &Engine| {
                    let size = e.mem.u32(stack_address + 4);
                    let data = e.mem.u32(stack_address);
                    e.mem
                        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4))
                };
                match kind {
                    COMMAND_CHUNK => {
                        let parent = top_of_stack(&*e);
                        let first = i32::from(e.mem.u16(command.wrapping_add(2)));
                        let second = i32::from(e.mem.u16(command.wrapping_add(4)));
                        new_node(e, Ptr::new(name), names, Ptr::new(parent), first, second);
                    }
                    COMMAND_PUSH => {
                        let parent = top_of_stack(&*e);
                        let first = i32::from(e.mem.u16(command.wrapping_add(2)));
                        let second = i32::from(e.mem.u16(command.wrapping_add(4)));
                        let node =
                            new_node(e, Ptr::new(name), names, Ptr::new(parent), first, second);
                        push_pointer(e, stack_address, node.addr());
                    }
                    COMMAND_POP => {
                        let size = e.mem.i32(stack_address + 4);
                        e.mem.set_i32(stack_address + 4, size.wrapping_sub(1));
                    }
                    _ => {
                        finished = false;
                        break;
                    }
                }
                offset = offset.wrapping_add(advance);
                if offset >= e.get(this, HkStreamStatisticsCollector::m_size) {
                    break;
                }
            }
        }
        if finished {
            sum_nodes(e, Ptr::new(root), names);
        }
        release_pointer_array(e, stack_address);
    });
}

// Translated from 00ddf970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::writeStatistics` (engine map name), thiscall with two stack
/// words that are not read: builds the tree into a local `hkArray<Node*>` (`makeTree(roots,
/// 0)`), destroys every node left in that array from the end (`00e0d0a0` and a free through
/// the heap allocator's slot `+8`), and releases the array.
pub fn hk_stream_statistics_collector_write_statistics(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    _unused_0: u32,
    _unused_1: u32,
) {
    e.with_stack(0xc, |e, roots| {
        let roots_address = roots.addr();
        e.mem.set_u32(roots_address, 0);
        e.mem.set_u32(roots_address + 4, 0);
        e.mem.set_u32(roots_address + 8, EMPTY_ARRAY_CAPACITY);
        hk_stream_statistics_collector_make_tree(e, this, roots, Ptr::NULL);
        if e.mem.i32(roots_address + 4) != 0 {
            loop {
                let size = e.mem.u32(roots_address + 4);
                let data = e.mem.u32(roots_address);
                let node = e
                    .mem
                    .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
                destroy_node(e, node);
                let size = e.mem.i32(roots_address + 4).wrapping_sub(1);
                e.mem.set_i32(roots_address + 4, size);
                if size == 0 {
                    break;
                }
            }
        }
        release_pointer_array(e, roots_address);
    });
}

/// The shared tail of `addReferencedObject` and `addObject`: does nothing unless the stream
/// has room and `object` is not null and not yet in the object map
/// (`getWithDefault(object, 0)`); otherwise enters it (`insert(allocator, object, 1)`).
/// Returns whether it was entered.
fn enter_object(e: &mut Engine, this: Ptr<HkStreamStatisticsCollector>, object: u32) -> bool {
    if !has_room(e, this) || object == 0 {
        return false;
    }
    let map = this.addr().wrapping_add(0x20);
    if e.call(MAP_GET_WITH_DEFAULT, &args![map, object, 0u32])
        .u32()
        != 0
    {
        return false;
    }
    let allocator = heap_allocator(e);
    e.call(MAP_INSERT, &args![map, allocator, object, 1u32]);
    true
}

// Translated from 00ddfa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::addReferencedObject` (Xbox PDB name), thiscall, three stack
/// words `(name, object, flags)`: for an object that the stream has room for, is not null and
/// is not in the object map yet (it is entered): when bit 0 of `flags` is clear, records its
/// start (`_startObject(name, object, getReferencedObjectSize(object))`), asks the object
/// for its statistics through its own virtual slot `+8` with `(this, 00d06d50(object))` and
/// pops (`_addKnown`); when set, only asks the object (the same slot call, without the start
/// and the pop).
pub fn hk_stream_statistics_collector_add_referenced_object(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    name: Ptr,
    object: Ptr,
    flags: u8,
) {
    if !enter_object(e, this, object.addr()) {
        return;
    }
    if flags & 1 == 0 {
        let size = e
            .call(GET_REFERENCED_OBJECT_SIZE, &args![this, object])
            .i32();
        hk_stream_statistics_collector_start_object(e, this, name, object.addr(), size);
    }
    let vtable = e.mem.u32(object.addr());
    let contents = e
        .call(REFERENCED_OBJECT_CONTENTS, &args![this, object])
        .u32();
    let calculate = e.mem.u32(vtable.wrapping_add(8));
    e.call(calculate, &args![object, this, contents]);
    if flags & 1 == 0 {
        hk_stream_statistics_collector_add_known(e, Ptr::new(this.addr()));
    }
}

// Translated from 00ddfab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStreamStatisticsCollector::addObject` (Xbox PDB name), thiscall, four stack words
/// `(class, name, object, flags)`: for an object that the stream has room for, is not null
/// and is not in the object map yet (it is entered): when bit 0 of `flags` is clear, records
/// its start (`_startObject(name, object, hkClass::getObjectSize(class))`), adds the
/// class's contents (`hkStatisticsCollectorUtil::addClassContentsAll(object, class, this)`,
/// cdecl) and pops (`_addKnown`); when set, only adds the contents.
pub fn hk_stream_statistics_collector_add_object(
    e: &mut Engine,
    this: Ptr<HkStreamStatisticsCollector>,
    class: Ptr,
    name: Ptr,
    object: Ptr,
    flags: u8,
) {
    if !enter_object(e, this, object.addr()) {
        return;
    }
    if flags & 1 == 0 {
        let size = e.call(CLASS_GET_OBJECT_SIZE, &args![class]).i32();
        hk_stream_statistics_collector_start_object(e, this, name, object.addr(), size);
    }
    e.call(ADD_CLASS_CONTENTS_ALL, &args![object, class, this]);
    if flags & 1 == 0 {
        hk_stream_statistics_collector_add_known(e, Ptr::new(this.addr()));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_fb40, deleting_destructor_00ddfb40(Ptr, u8) -> Ptr),
        entry!(
            0x00dd_eff0,
            hk_monitor_stream_analyzer_node_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(0x00dd_f020, hk_stream_statistics_collector_add_known(Ptr)),
        entry!(0x00dd_f030, fn_00ddf030(Ptr, u32)),
        entry!(0x00dd_f040, fn_00ddf040(Ptr)),
        entry!(0x00dd_f050, fn_00ddf050(Ptr<HkStreamStatisticsCollector>)),
        entry!(0x00dd_f070, new_node(Ptr, Ptr, Ptr, i32, i32) -> Ptr),
        entry!(
            0x00dd_f170,
            hk_stream_statistics_collector_hk_stream_statistics_collector(
                Ptr<HkStreamStatisticsCollector>,
                u32,
                i32,
            ) -> Ptr<
                HkStreamStatisticsCollector,
            >
        ),
        entry!(
            0x00dd_f240,
            hk_stream_statistics_collector_dtor(Ptr<HkStreamStatisticsCollector>)
        ),
        entry!(
            0x00dd_f2a0,
            fn_00ddf2a0(Ptr<HkStreamStatisticsCollector>, Ptr, i32, u8)
        ),
        entry!(0x00dd_f370, new_command(Ptr, i32) -> u32),
        entry!(
            0x00dd_f420,
            hk_stream_statistics_collector_start_object(
                Ptr<HkStreamStatisticsCollector>,
                Ptr,
                u32,
                i32,
            )
        ),
        entry!(
            0x00dd_f4d0,
            hk_stream_statistics_collector_add_chunk(
                Ptr<HkStreamStatisticsCollector>,
                u32,
                Ptr,
                u32,
                u32,
                i32,
            )
        ),
        entry!(
            0x00dd_f5a0,
            hk_stream_statistics_collector_push_dir(Ptr<HkStreamStatisticsCollector>, Ptr)
        ),
        entry!(
            0x00dd_f640,
            hk_stream_statistics_collector_pop_dir(Ptr<HkStreamStatisticsCollector>)
        ),
        entry!(0x00dd_f680, sum_nodes(Ptr, Ptr)),
        entry!(
            0x00dd_f740,
            hk_stream_statistics_collector_make_tree(Ptr<HkStreamStatisticsCollector>, Ptr, Ptr)
        ),
        entry!(
            0x00dd_f970,
            hk_stream_statistics_collector_write_statistics(
                Ptr<HkStreamStatisticsCollector>,
                u32,
                u32,
            )
        ),
        entry!(
            0x00dd_fa10,
            hk_stream_statistics_collector_add_referenced_object(
                Ptr<HkStreamStatisticsCollector>,
                Ptr,
                Ptr,
                u8,
            )
        ),
        entry!(
            0x00dd_fab0,
            hk_stream_statistics_collector_add_object(
                Ptr<HkStreamStatisticsCollector>,
                Ptr,
                Ptr,
                Ptr,
                u8,
            )
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
    fn deleting_destructor_00ddfb40_frees_only_when_bit_zero_is_set() {
        check(0x00dd_fb40, 0x00dd_f240);
    }
}

#[cfg(test)]
mod tests_collector {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Keys = Rc<RefCell<Vec<u32>>>;

    const ALLOCATE_SLOT: u32 = 0x0900_0100;
    const FREE_SLOT: u32 = 0x0900_0101;
    const ALLOCATE_BUFFER_SLOT: u32 = 0x0900_0102;
    const FREE_BUFFER_SLOT: u32 = 0x0900_0103;
    const OBJECT_ID_SLOT: u32 = 0x0900_0104;
    const POP_DIR_SLOT: u32 = 0x0900_0105;

    struct Rig {
        e: Engine,
        log: Log,
        keys: Keys,
    }

    /// Registers a double that also logs its address and argument words.
    fn double(
        e: &mut Engine,
        log: &Log,
        address: u32,
        mut f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            f(e, a)
        });
    }

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Moves the `hkArray` at `array` into a fresh block of `count` elements of `element`
    /// bytes (capacity word `count`, no flags).
    fn regrow(e: &mut Engine, array: u32, count: u32, element: u32) {
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let block = e.mem.alloc((count * element).max(4));
        for i in 0..size.min(count) * element {
            let byte = e.mem.u8(old + i);
            e.mem.set_u8(block + i, byte);
        }
        e.mem.set_u32(array, block);
        e.mem.set_u32(array + 8, count);
    }

    /// Appends a pointer to the `hkArray` of pointers at `array` (test helper).
    fn push_word(e: &mut Engine, array: u32, value: u32) {
        let size = e.mem.u32(array + 4);
        if size == e.mem.u32(array + 8) & CAPACITY_MASK {
            regrow(e, array, (size * 2).max(1), 4);
        }
        let data = e.mem.u32(array);
        e.mem.set_u32(data + 4 * size, value);
        e.mem.set_u32(array + 4, size + 1);
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let keys: Keys = Rc::new(RefCell::new(Vec::new()));
        // The router and its allocator: slots 4, 8, 0xc, 0x10 and 0x2c.
        let vtable = e.mem.alloc(0x40);
        for (slot, address) in [
            (4, ALLOCATE_SLOT),
            (8, FREE_SLOT),
            (0xc, ALLOCATE_BUFFER_SLOT),
            (0x10, FREE_BUFFER_SLOT),
            (0x2c, OBJECT_ID_SLOT),
        ] {
            e.mem.set_u32(vtable + slot, address);
        }
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        double(&mut e, &log, MEMORY_ROUTER, move |_, _| returns(router));
        double(&mut e, &log, ALLOCATE_SLOT, |e, a| {
            returns(e.mem.alloc(a[1]))
        });
        double(&mut e, &log, FREE_SLOT, |_, _| Ret::default());
        double(&mut e, &log, ALLOCATE_BUFFER_SLOT, |e, a| {
            // The size word is in/out: the granted size is the request.
            let size = e.mem.u32(a[1]);
            returns(e.mem.alloc(size))
        });
        double(&mut e, &log, FREE_BUFFER_SLOT, |_, _| Ret::default());
        double(&mut e, &log, OBJECT_ID_SLOT, |_, a| {
            returns(0x1_0000 + a[2])
        });
        double(&mut e, &log, ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            regrow(e, a[1], if size == 0 { 1 } else { size * 2 }, a[2]);
            Ret::default()
        });
        double(&mut e, &log, ARRAY_RESERVE, |e, a| {
            regrow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        double(&mut e, &log, ARRAY_INSERT, |e, a| {
            let (array, index, value) = (a[0], a[2], e.mem.u32(a[3]));
            let size = e.mem.u32(array + 4);
            if size == e.mem.u32(array + 8) & CAPACITY_MASK {
                regrow(e, array, (size * 2).max(1), 4);
            }
            let data = e.mem.u32(array);
            for i in (index..size).rev() {
                let word = e.mem.u32(data + 4 * i);
                e.mem.set_u32(data + 4 * (i + 1), word);
            }
            e.mem.set_u32(data + 4 * index, value);
            e.mem.set_u32(array + 4, size + 1);
            Ret::default()
        });
        e.register(STR_LEN, |e, a| returns(e.mem.cstr(a[0]).len() as u32));
        e.register(STR_CMP, |e, a| {
            let left = e.mem.cstr(a[0]);
            let right = e.mem.cstr(a[1]);
            returns(match left.cmp(&right) {
                std::cmp::Ordering::Less => -1i32 as u32,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            })
        });
        e.register(MEM_CPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            returns(a[0])
        });
        // hkStringPtr: the text constructor keeps a copy (owned, pointer + 1).
        double(&mut e, &log, STRING_PTR_FROM_TEXT, |e, a| {
            let text = e.mem.cstr(a[1]);
            let copy = e.mem.alloc(text.len() as u32 + 1);
            e.mem.write(copy, &text);
            e.mem.set_u32(a[0], copy + 1);
            returns(a[0])
        });
        double(&mut e, &log, STRING_PTR_COPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            returns(a[0])
        });
        double(&mut e, &log, STRING_PTR_DESTRUCTOR, |_, _| Ret::default());
        // Nodes: the constructor empties the node, records parent and name, and registers the
        // node with its parent; the destructor only logs.
        double(&mut e, &log, NODE_CONSTRUCTOR, |e, a| {
            let (node, parent, name) = (a[0], a[1], a[2]);
            for offset in (0..NODE_SIZE).step_by(4) {
                e.mem.set_u32(node + offset, 0);
            }
            e.mem.set_u32(node + 0x38, EMPTY_ARRAY_CAPACITY);
            e.mem.set_u32(node + 0x48, parent);
            e.mem.set_u32(node + 0x4c, name);
            if parent != 0 {
                push_word(e, parent + 0x30, node);
            }
            returns(node)
        });
        double(&mut e, &log, NODE_DESTRUCTOR, |_, _| Ret::default());
        // The object map: the keys seen.
        let seen = Rc::clone(&keys);
        double(&mut e, &log, MAP_GET_WITH_DEFAULT, move |_, a| {
            if seen.borrow().contains(&a[1]) {
                returns(1)
            } else {
                returns(a[2])
            }
        });
        let seen = Rc::clone(&keys);
        double(&mut e, &log, MAP_INSERT, move |_, a| {
            seen.borrow_mut().push(a[2]);
            Ret::default()
        });
        let seen = Rc::clone(&keys);
        double(&mut e, &log, MAP_CLEAR, move |_, _| {
            seen.borrow_mut().clear();
            Ret::default()
        });
        // The base class, the map's heavier operations and the float truncation.
        double(&mut e, &log, BASE_CONSTRUCTOR, |_, _| Ret::default());
        double(&mut e, &log, BASE_DESTRUCTOR, |_, _| Ret::default());
        double(&mut e, &log, MAP_RESERVE, |_, _| Ret::default());
        double(&mut e, &log, MAP_CLEAR_AND_DEALLOCATE, |_, _| {
            Ret::default()
        });
        double(&mut e, &log, MAP_DESTRUCTOR, |_, _| Ret::default());
        double(&mut e, &log, FLOAT_TO_INT, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            returns(value as i32 as u32)
        });
        Rig { e, log, keys }
    }

    fn calls(rig: &Rig, address: u32) -> Vec<Vec<u32>> {
        rig.log
            .borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A collector record with a stream of `capacity` bytes holding `bytes`.
    fn collector(e: &mut Engine, bytes: &[u8], capacity: u32) -> u32 {
        let this = e.mem.alloc(0x30);
        let data = e.mem.alloc(capacity.max(1));
        e.mem.write(data, bytes);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, bytes.len() as u32);
        e.mem.set_u32(this + 0x1c, capacity);
        this
    }

    /// An empty `hkArray` record that does not own storage.
    fn empty_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array + 8, EMPTY_ARRAY_CAPACITY);
        array
    }

    /// A node with a name and the two totals.
    fn node(e: &mut Engine, name: &[u8], first: f32, second: f32) -> u32 {
        let node = e.mem.alloc(NODE_SIZE);
        e.mem.set_u32(node + 0x38, EMPTY_ARRAY_CAPACITY);
        let text = e.mem.alloc(name.len() as u32 + 1);
        e.mem.write(text, name);
        e.mem.set_u32(node + 0x4c, text);
        e.mem.set_f32(node, first);
        e.mem.set_f32(node + 4, second);
        node
    }

    fn children(e: &Engine, node: u32) -> Vec<u32> {
        let data = e.mem.u32(node + 0x30);
        (0..e.mem.u32(node + 0x34))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn name_of(e: &Engine, node: u32) -> String {
        String::from_utf8(e.mem.cstr(e.mem.u32(node + 0x4c))).unwrap()
    }

    /// One command as the stream stores it.
    fn command(kind: u8, first: u16, second: u16, name: &[u8]) -> Vec<u8> {
        let mut payload = name.to_vec();
        if !name.is_empty() {
            payload.push(0);
            if payload.len() % 2 == 1 {
                payload.push(0);
            }
        }
        let mut bytes = vec![kind, payload.len() as u8];
        bytes.extend_from_slice(&first.to_le_bytes());
        bytes.extend_from_slice(&second.to_le_bytes());
        bytes.extend_from_slice(&payload);
        bytes
    }

    fn stream_of(e: &Engine, this: u32) -> Vec<u8> {
        e.mem.bytes(e.mem.u32(this + 0x14), e.mem.u32(this + 0x18))
    }

    #[test]
    fn node_deleting_destructor_frees_the_block_only_when_asked() {
        let mut rig = rig();
        assert_eq!(
            rig.e.call(0x00dd_eff0, &args![0x5000u32, 1u8]).u32(),
            0x5000
        );
        assert_eq!(calls(&rig, NODE_DESTRUCTOR), vec![vec![0x5000]]);
        assert_eq!(calls(&rig, FREE_SLOT).len(), 1);
        assert_eq!(calls(&rig, FREE_SLOT)[0][1..], [0x5000, 0x58]);
        // Bit 0 clear: no free. A null object: the destructor still runs, nothing is freed.
        rig.log.borrow_mut().clear();
        rig.e.call(0x00dd_eff0, &args![0x5000u32, 0xfeu8]);
        assert!(calls(&rig, FREE_SLOT).is_empty());
        rig.e.call(0x00dd_eff0, &args![0u32, 1u8]);
        assert_eq!(calls(&rig, NODE_DESTRUCTOR).len(), 2);
        assert!(calls(&rig, FREE_SLOT).is_empty());
    }

    #[test]
    fn add_known_calls_the_pop_slot_of_the_collector() {
        let mut rig = rig();
        let vtable = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(vtable + 0x1c, POP_DIR_SLOT);
        let this = rig.e.mem.alloc(0x30);
        rig.e.mem.set_u32(this, vtable);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, POP_DIR_SLOT, |_, _| Ret::default());
        rig.e.call(0x00dd_f020, &args![this]);
        assert_eq!(calls(&rig, POP_DIR_SLOT), vec![vec![this]]);
    }

    #[test]
    fn the_map_clearing_functions_clear_the_map_at_plus_0x20() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[1, 2, 3], 0x100);
        rig.keys.borrow_mut().push(5);
        rig.e.call(0x00dd_f030, &args![this, 0xdead_u32]);
        assert!(rig.keys.borrow().is_empty());
        rig.keys.borrow_mut().push(5);
        rig.e.call(0x00dd_f040, &args![this]);
        assert!(rig.keys.borrow().is_empty());
        assert_eq!(
            calls(&rig, MAP_CLEAR),
            vec![vec![this + 0x20], vec![this + 0x20]]
        );
        // The third also resets the stream's size.
        rig.e.call(0x00dd_f050, &args![this]);
        assert_eq!(rig.e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&rig, MAP_CLEAR).len(), 3);
    }

    /// `hkArray<hkStringPtr>` for the names.
    fn names_array(e: &mut Engine) -> u32 {
        empty_array(e)
    }

    #[test]
    fn new_node_finds_an_existing_child_and_adds_to_its_totals() {
        let mut rig = rig();
        let parent = node(&mut rig.e, b"root", 0.0, 0.0);
        let other = node(&mut rig.e, b"other", 0.0, 0.0);
        let wanted = node(&mut rig.e, b"wanted", 1.5, 10.0);
        push_word(&mut rig.e, parent + 0x30, other);
        push_word(&mut rig.e, parent + 0x30, wanted);
        let text = rig.e.mem.alloc(8);
        rig.e.mem.write(text, b"wanted\0");
        let got = rig
            .e
            .call(0x00dd_f070, &args![text, 0u32, parent, 2i32, 3i32])
            .u32();
        assert_eq!(got, wanted);
        assert_eq!(rig.e.mem.f32(wanted), 3.5);
        assert_eq!(rig.e.mem.f32(wanted + 4), 13.0);
        assert_eq!(rig.e.mem.u16(wanted + 0x20), 1);
        assert_eq!(rig.e.mem.u16(wanted + 0x22), 1);
        assert!(calls(&rig, NODE_CONSTRUCTOR).is_empty());
        // A zero second count is the first count.
        rig.e
            .call(0x00dd_f070, &args![text, 0u32, parent, 4i32, 0i32]);
        assert_eq!(rig.e.mem.f32(wanted), 7.5);
        assert_eq!(rig.e.mem.f32(wanted + 4), 17.0);
        assert_eq!(rig.e.mem.u16(wanted + 0x20), 2);
    }

    #[test]
    fn new_node_makes_a_child_and_remembers_its_name() {
        let mut rig = rig();
        let parent = node(&mut rig.e, b"root", 0.0, 0.0);
        let names = names_array(&mut rig.e);
        let text = rig.e.mem.alloc(8);
        rig.e.mem.write(text, b"fresh\0");
        let got = rig
            .e
            .call(0x00dd_f070, &args![text, names, parent, 5i32, 6i32])
            .u32();
        assert_eq!(children(&rig.e, parent), vec![got]);
        assert_eq!(rig.e.mem.f32(got), 5.0);
        assert_eq!(rig.e.mem.f32(got + 4), 6.0);
        assert_eq!(rig.e.mem.u16(got + 0x20), 1);
        // The node is allocated with 0x58 bytes and constructed with (parent, name, 1); the
        // name it holds is the stored copy (the names array's element without bit 0).
        assert_eq!(calls(&rig, ALLOCATE_SLOT).len(), 1);
        assert_eq!(calls(&rig, ALLOCATE_SLOT)[0][1], 0x58);
        assert_eq!(rig.e.mem.u32(names + 4), 1);
        let stored = rig.e.mem.u32(rig.e.mem.u32(names)) & !1;
        assert_eq!(name_of(&rig.e, got), "fresh");
        assert_eq!(rig.e.mem.u32(got + 0x4c), stored);
        let constructed = calls(&rig, NODE_CONSTRUCTOR);
        assert_eq!(constructed[0][1..], [parent, stored, 1]);
        // The names array grew (it owned nothing), and the temporary was destroyed.
        assert_eq!(calls(&rig, ARRAY_GROW).len(), 1);
        assert_eq!(calls(&rig, STRING_PTR_DESTRUCTOR).len(), 1);
        // A second new name appends a second element; the array is full at one element, so
        // it grows again.
        let again = rig.e.mem.alloc(8);
        rig.e.mem.write(again, b"second\0");
        rig.e
            .call(0x00dd_f070, &args![again, names, parent, 1i32, 1i32]);
        assert_eq!(rig.e.mem.u32(names + 4), 2);
        assert_eq!(children(&rig.e, parent).len(), 2);
    }

    #[test]
    fn new_node_without_a_names_array_uses_the_name_pointer_and_defaults_a_null_name() {
        let mut rig = rig();
        let parent = node(&mut rig.e, b"root", 0.0, 0.0);
        rig.e.mem.set_cstr(DEFAULT_NODE_NAME, b"self");
        let got = rig
            .e
            .call(0x00dd_f070, &args![0u32, 0u32, parent, 1i32, 1i32])
            .u32();
        assert_eq!(rig.e.mem.u32(got + 0x4c), DEFAULT_NODE_NAME);
        // The next request for a null name finds the same child.
        let again = rig
            .e
            .call(0x00dd_f070, &args![0u32, 0u32, parent, 1i32, 1i32])
            .u32();
        assert_eq!(again, got);
        assert_eq!(rig.e.mem.f32(got), 2.0);
        assert_eq!(rig.e.mem.u16(got + 0x20), 2);
        // With no parent there is nothing to search: a node is made with no parent.
        let orphan = rig
            .e
            .call(0x00dd_f070, &args![0u32, 0u32, 0u32, 1i32, 1i32])
            .u32();
        assert_ne!(orphan, got);
        assert_eq!(calls(&rig, NODE_CONSTRUCTOR).last().unwrap()[1], 0);
    }

    #[test]
    fn constructor_takes_the_granted_buffer_and_empties_the_stream() {
        let mut rig = rig();
        let this = rig.e.mem.alloc(0x30);
        assert_eq!(
            rig.e
                .call(0x00dd_f170, &args![this, 0x77u32, 0x100i32])
                .u32(),
            this
        );
        assert_eq!(calls(&rig, BASE_CONSTRUCTOR), vec![vec![this, 0x77]]);
        assert_eq!(rig.e.mem.u32(this), COLLECTOR_VTABLE);
        // The granted size is the capacity; the size is reset to 0.
        assert_eq!(rig.e.mem.u32(this + 0x1c), 0x100);
        assert_eq!(rig.e.mem.u32(this + 0x18), 0);
        assert_ne!(rig.e.mem.u32(this + 0x14), 0);
        // The map is empty and reserved for 0 elements through the heap allocator.
        assert_eq!(rig.e.mem.u32(this + 0x20), 0);
        assert_eq!(rig.e.mem.u32(this + 0x24), 0);
        assert_eq!(rig.e.mem.i32(this + 0x28), -1);
        let reserve = calls(&rig, MAP_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][0], this + 0x20);
        assert_eq!(reserve[0][2], 0);
    }

    #[test]
    fn constructor_without_a_buffer_leaves_a_borrowed_empty_stream() {
        let mut rig = rig();
        let this = rig.e.mem.alloc(0x30);
        rig.e.call(0x00dd_f170, &args![this, 0x77u32, 0i32]);
        assert_eq!(rig.e.mem.u32(this + 0x14), 0);
        assert_eq!(rig.e.mem.u32(this + 0x18), 0);
        assert_eq!(rig.e.mem.u32(this + 0x1c), 0x8000_0000);
        assert!(calls(&rig, ALLOCATE_BUFFER_SLOT).is_empty());
    }

    #[test]
    fn constructor_gives_a_zero_granted_size_the_borrowed_capacity_word() {
        let mut rig = rig();
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, ALLOCATE_BUFFER_SLOT, |e, a| {
            e.mem.set_u32(a[1], 0);
            returns(0x7000)
        });
        let this = rig.e.mem.alloc(0x30);
        rig.e.call(0x00dd_f170, &args![this, 0x77u32, 0x100i32]);
        assert_eq!(rig.e.mem.u32(this + 0x14), 0x7000);
        assert_eq!(rig.e.mem.u32(this + 0x1c), 0x8000_0000);
    }

    #[test]
    fn destructor_releases_the_map_and_the_owned_stream() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[1, 2, 3], 0x40);
        let data = rig.e.mem.u32(this + 0x14);
        rig.e.call(0x00dd_f240, &args![this]);
        assert_eq!(rig.e.mem.u32(this), COLLECTOR_VTABLE);
        assert_eq!(calls(&rig, MAP_CLEAR_AND_DEALLOCATE).len(), 1);
        assert_eq!(calls(&rig, MAP_CLEAR_AND_DEALLOCATE)[0][0], this + 0x20);
        assert_eq!(calls(&rig, MAP_DESTRUCTOR), vec![vec![this + 0x20]]);
        assert_eq!(calls(&rig, FREE_BUFFER_SLOT).len(), 1);
        assert_eq!(calls(&rig, FREE_BUFFER_SLOT)[0][1..], [data, 0x40]);
        assert_eq!(rig.e.mem.u32(this + 0x14), 0);
        assert_eq!(rig.e.mem.u32(this + 0x1c), 0x8000_0000);
        assert_eq!(rig.e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&rig, BASE_DESTRUCTOR), vec![vec![this]]);
        // Borrowed storage is not freed.
        let borrowed = collector(&mut rig.e, &[], 0);
        rig.e.mem.set_u32(borrowed + 0x1c, 0x8000_0040);
        rig.log.borrow_mut().clear();
        rig.e.call(0x00dd_f240, &args![borrowed]);
        assert!(calls(&rig, FREE_BUFFER_SLOT).is_empty());
    }

    #[test]
    fn fn_00ddf2a0_copies_and_swaps_the_header_numbers() {
        let mut rig = rig();
        let mut bytes = command(COMMAND_PUSH, 0x1234, 0x5678, b"x");
        bytes.extend(command(COMMAND_POP, 0xabcd, 0, b""));
        let source = rig.e.mem.alloc(bytes.len() as u32);
        rig.e.mem.write(source, &bytes);
        let this = collector(&mut rig.e, &[], 0x100);
        rig.e
            .call(0x00dd_f2a0, &args![this, source, bytes.len() as i32, 1u8]);
        let copied = stream_of(&rig.e, this);
        assert_eq!(copied.len(), bytes.len());
        // The headers' 16-bit numbers are byte swapped; everything else is as copied.
        let mut want = bytes.clone();
        want.swap(2, 3);
        want.swap(4, 5);
        let second = 6 + 2;
        want.swap(second + 2, second + 3);
        want.swap(second + 4, second + 5);
        assert_eq!(copied, want);
        assert!(calls(&rig, ARRAY_RESERVE).is_empty());
        // Without the swap flag the bytes are as given; a larger source grows the stream.
        let this = collector(&mut rig.e, &[], 4);
        rig.e
            .call(0x00dd_f2a0, &args![this, source, bytes.len() as i32, 0u8]);
        assert_eq!(stream_of(&rig.e, this), bytes);
        let reserve = calls(&rig, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        // max(size, 2 * capacity) = max(14, 8).
        assert_eq!(reserve[0][2..], [bytes.len() as u32, 1]);
    }

    #[test]
    fn new_command_extends_the_stream_and_returns_the_old_end() {
        let mut rig = rig();
        let array = empty_array(&mut rig.e);
        // An empty array first gets room for one byte (the growth step), then its size is
        // reset; the command starts at the data pointer.
        let first = rig.e.call(0x00dd_f370, &args![array, 4i32]).u32();
        assert_eq!(calls(&rig, ARRAY_GROW).len(), 1);
        assert_ne!(first, 0);
        assert_eq!(rig.e.mem.u32(array + 4), 10);
        // The capacity (1) was below the size 10: reserved to the larger of 10 and 2.
        assert_eq!(calls(&rig, ARRAY_RESERVE)[0][2..], [10, 1]);
        // The pointer is the one from before the second reserve moved the data.
        assert_ne!(rig.e.mem.u32(array), first);
        // A non-empty stream: the command starts at data + size.
        let this = collector(&mut rig.e, &[0; 4], 0x100);
        let second = rig.e.call(0x00dd_f370, &args![this + 0x14, 2i32]).u32();
        assert_eq!(second, rig.e.mem.u32(this + 0x14) + 4);
        assert_eq!(rig.e.mem.u32(this + 0x18), 12);
        // Enough capacity: no further reserve.
        assert_eq!(calls(&rig, ARRAY_RESERVE).len(), 1);
    }

    #[test]
    fn new_command_grows_to_twice_the_capacity_when_that_is_larger() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[0; 4], 8);
        rig.e.call(0x00dd_f370, &args![this + 0x14, 6i32]);
        // 4 + 6 + 6 = 16 against 2 * 8 = 16: the new size.
        assert_eq!(calls(&rig, ARRAY_RESERVE)[0][2..], [16, 1]);
        let that = collector(&mut rig.e, &[0; 4], 10);
        rig.e.call(0x00dd_f370, &args![that + 0x14, 2i32]);
        // 12 against 20: twice the capacity wins.
        assert_eq!(calls(&rig, ARRAY_RESERVE)[1][2..], [20, 1]);
    }

    #[test]
    fn start_object_records_a_push_with_the_name_and_the_object_id() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[], 0x100);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"ab\0");
        rig.e
            .call(0x00dd_f420, &args![this, name, 0x9000u32, 0x1_2345i32]);
        // "ab" and its zero are 3 bytes: padded to 4. The first number is the size's low 16
        // bits, the second the low 16 bits of the allocator's answer 0x10000 + size.
        let mut want = vec![COMMAND_PUSH, 4];
        want.extend_from_slice(&0x2345u16.to_le_bytes());
        want.extend_from_slice(&0x2345u16.to_le_bytes());
        want.extend_from_slice(b"ab\0\0");
        assert_eq!(stream_of(&rig.e, this), want);
        assert_eq!(calls(&rig, OBJECT_ID_SLOT)[0][1..], [0x9000, 0x1_2345]);
    }

    #[test]
    fn start_object_without_a_name_or_a_size() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[], 0x100);
        rig.e.call(0x00dd_f420, &args![this, 0u32, 0x9000u32, 0i32]);
        assert_eq!(stream_of(&rig.e, this), vec![COMMAND_PUSH, 0, 0, 0, 0, 0]);
        assert!(calls(&rig, OBJECT_ID_SLOT).is_empty());
        // An even-sized name (with its zero) needs no pad byte.
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"abc\0");
        let that = collector(&mut rig.e, &[], 0x100);
        rig.e.call(0x00dd_f420, &args![that, name, 1u32, 1i32]);
        let bytes = stream_of(&rig.e, that);
        assert_eq!(bytes[1], 4);
        assert_eq!(&bytes[6..], b"abc\0");
    }

    #[test]
    fn add_chunk_records_a_chunk_when_there_is_room_and_something_to_say() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[], 0x100);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"x\0");
        // `used` and `size` both zero: nothing.
        rig.e
            .call(0x00dd_f4d0, &args![this, 0u32, name, 0x9000u32, 0u32, 0i32]);
        assert!(stream_of(&rig.e, this).is_empty());
        rig.e
            .call(0x00dd_f4d0, &args![this, 0u32, name, 0x9000u32, 7u32, 0i32]);
        let mut want = vec![COMMAND_CHUNK, 2];
        want.extend_from_slice(&7u16.to_le_bytes());
        want.extend_from_slice(&0u16.to_le_bytes());
        want.extend_from_slice(b"x\0");
        assert_eq!(stream_of(&rig.e, this), want);
        // A size asks the allocator for the second number.
        rig.e
            .call(0x00dd_f4d0, &args![this, 0u32, name, 0x9000u32, 0u32, 3i32]);
        let bytes = stream_of(&rig.e, this);
        assert_eq!(bytes[want.len()..want.len() + 2], [COMMAND_CHUNK, 2]);
        assert_eq!(bytes[want.len() + 4..want.len() + 6], 3u16.to_le_bytes());
        assert_eq!(calls(&rig, OBJECT_ID_SLOT)[0][1..], [0x9000, 3]);
    }

    #[test]
    fn commands_need_room_for_more_than_0x46_bytes() {
        let mut rig = rig();
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"x\0");
        // Capacity 0x46 + size is not enough (it must be larger).
        let full = collector(&mut rig.e, &[0; 4], 0x4a);
        rig.e.call(0x00dd_f5a0, &args![full, name]);
        rig.e.call(0x00dd_f640, &args![full]);
        rig.e
            .call(0x00dd_f4d0, &args![full, 0u32, name, 0u32, 1u32, 1i32]);
        assert_eq!(rig.e.mem.u32(full + 0x18), 4);
        // One byte more is enough.
        let room = collector(&mut rig.e, &[0; 4], 0x4b);
        rig.e.call(0x00dd_f640, &args![room]);
        assert_eq!(rig.e.mem.u32(room + 0x18), 4 + 6);
    }

    #[test]
    fn push_dir_and_pop_dir_record_their_commands() {
        let mut rig = rig();
        let this = collector(&mut rig.e, &[], 0x100);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"dir\0");
        rig.e.call(0x00dd_f5a0, &args![this, name]);
        rig.e.call(0x00dd_f640, &args![this]);
        let mut want = command(COMMAND_PUSH, 0, 0, b"dir");
        want.extend(command(COMMAND_POP, 0, 0, b""));
        assert_eq!(stream_of(&rig.e, this), want);
        // A null name is a push with no payload.
        let that = collector(&mut rig.e, &[], 0x100);
        rig.e.call(0x00dd_f5a0, &args![that, 0u32]);
        assert_eq!(stream_of(&rig.e, that), vec![COMMAND_PUSH, 0, 0, 0, 0, 0]);
    }

    /// The names and map for node tests: `SELF_NODE_NAME` holds `"self"`.
    fn prepare_self_name(e: &mut Engine) {
        e.mem.set_cstr(SELF_NODE_NAME, b"self");
    }

    #[test]
    fn sum_nodes_adds_a_self_child_and_sums_the_children() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        let parent = node(&mut rig.e, b"parent", 10.0, 20.0);
        let first = node(&mut rig.e, b"first", 1.0, 2.0);
        let second = node(&mut rig.e, b"second", 3.0, 4.0);
        push_word(&mut rig.e, parent + 0x30, first);
        push_word(&mut rig.e, parent + 0x30, second);
        rig.e.call(0x00dd_f680, &args![parent, 0u32]);
        // A "self" child holding 10 and 20 was made, moved to the front; the parent's totals
        // are the sums over its children, 10 + 1 + 3 and 20 + 2 + 4.
        let kids = children(&rig.e, parent);
        assert_eq!(kids.len(), 3);
        assert_eq!(kids[1..], [first, second]);
        assert_eq!(name_of(&rig.e, kids[0]), "self");
        assert_eq!(rig.e.mem.f32(kids[0]), 10.0);
        assert_eq!(rig.e.mem.f32(kids[0] + 4), 20.0);
        assert_eq!(rig.e.mem.f32(parent), 14.0);
        assert_eq!(rig.e.mem.f32(parent + 4), 26.0);
        // The totals are truncated to integers for the new child: `_ftol2` of +4, then of +0.
        let conversions = calls(&rig, FLOAT_TO_INT);
        assert_eq!(conversions.len(), 2);
        assert_eq!(
            f64::from_bits(u64::from(conversions[0][0]) | u64::from(conversions[0][1]) << 32),
            20.0
        );
        assert_eq!(
            f64::from_bits(u64::from(conversions[1][0]) | u64::from(conversions[1][1]) << 32),
            10.0
        );
    }

    #[test]
    fn sum_nodes_skips_a_zero_total_and_leaves_childless_nodes_alone() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        rig.e.register(FLOAT_TO_INT, |_, _| returns(0));
        // A node without children is untouched, a null node does nothing.
        let leaf = node(&mut rig.e, b"leaf", 5.0, 6.0);
        rig.e.call(0x00dd_f680, &args![leaf, 0u32]);
        rig.e.call(0x00dd_f680, &args![0u32, 0u32]);
        assert_eq!(rig.e.mem.f32(leaf), 5.0);
        // A zero first total: no self child; the totals are recomputed from the children.
        let parent = node(&mut rig.e, b"parent", 0.0, 99.0);
        let child = node(&mut rig.e, b"child", 2.0, 3.0);
        push_word(&mut rig.e, parent + 0x30, child);
        rig.e.call(0x00dd_f680, &args![parent, 0u32]);
        assert_eq!(children(&rig.e, parent), vec![child]);
        assert_eq!(rig.e.mem.f32(parent), 2.0);
        assert_eq!(rig.e.mem.f32(parent + 4), 3.0);
        // A NaN first total counts as non-zero: the self child is made.
        let odd = node(&mut rig.e, b"odd", f32::NAN, 1.0);
        let kid = node(&mut rig.e, b"kid", 1.0, 1.0);
        push_word(&mut rig.e, odd + 0x30, kid);
        rig.e.call(0x00dd_f680, &args![odd, 0u32]);
        assert_eq!(children(&rig.e, odd).len(), 2);
    }

    #[test]
    fn make_tree_replays_the_commands_into_nodes_and_sums_them() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        let mut stream = command(COMMAND_PUSH, 5, 7, b"a");
        stream.extend(command(COMMAND_CHUNK, 3, 4, b"b"));
        stream.extend(command(COMMAND_POP, 0, 0, b""));
        stream.extend(command(COMMAND_CHUNK, 1, 0, b"c"));
        let this = collector(&mut rig.e, &stream, 0x200);
        let roots = empty_array(&mut rig.e);
        let names = names_array(&mut rig.e);
        rig.e.call(0x00dd_f740, &args![this, roots, names]);
        // One root, "/" with no parent, created with the 0x58-byte block.
        assert_eq!(rig.e.mem.u32(roots + 4), 1);
        let root = rig.e.mem.u32(rig.e.mem.u32(roots));
        assert_eq!(
            calls(&rig, NODE_CONSTRUCTOR)[0][1..],
            [0, ROOT_NODE_NAME, 1]
        );
        let kids = children(&rig.e, root);
        assert_eq!(kids.len(), 2);
        let (a, c) = (kids[0], kids[1]);
        assert_eq!(name_of(&rig.e, a), "a");
        assert_eq!(name_of(&rig.e, c), "c");
        // "a" got a self child in front of "b": totals 5 + 3 and 7 + 4; "c" counted 1 twice.
        let under_a = children(&rig.e, a);
        assert_eq!(under_a.len(), 2);
        assert_eq!(name_of(&rig.e, under_a[0]), "self");
        assert_eq!(name_of(&rig.e, under_a[1]), "b");
        assert_eq!(rig.e.mem.f32(a), 8.0);
        assert_eq!(rig.e.mem.f32(a + 4), 11.0);
        assert_eq!(rig.e.mem.f32(c), 1.0);
        assert_eq!(rig.e.mem.f32(c + 4), 1.0);
        // The root had no total of its own: the sums of its children.
        assert_eq!(rig.e.mem.f32(root), 9.0);
        assert_eq!(rig.e.mem.f32(root + 4), 12.0);
        // The names array holds the copies of "a", "b", "c" and the self names.
        assert!(rig.e.mem.u32(names + 4) >= 3);
    }

    #[test]
    fn make_tree_reuses_the_first_root_and_destroys_the_others() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        let this = collector(&mut rig.e, &command(COMMAND_CHUNK, 2, 2, b"x"), 0x200);
        let roots = empty_array(&mut rig.e);
        let first = node(&mut rig.e, b"first", 0.0, 0.0);
        let second = node(&mut rig.e, b"second", 0.0, 0.0);
        let third = node(&mut rig.e, b"third", 0.0, 0.0);
        for root in [first, second, third] {
            push_word(&mut rig.e, roots, root);
        }
        rig.e.call(0x00dd_f740, &args![this, roots, 0u32]);
        assert_eq!(rig.e.mem.u32(roots + 4), 1);
        // The extra roots were destroyed from the end, each with its destructor and a free.
        let destroyed: Vec<u32> = calls(&rig, NODE_DESTRUCTOR).iter().map(|c| c[0]).collect();
        assert_eq!(destroyed, vec![third, second]);
        assert_eq!(calls(&rig, FREE_SLOT).len(), 2);
        // The command went under the first root.
        assert_eq!(children(&rig.e, first).len(), 1);
    }

    #[test]
    fn make_tree_stops_at_an_unknown_command_without_summing() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        let mut stream = command(COMMAND_CHUNK, 2, 2, b"x");
        stream.extend(command(b'Z', 0, 0, b""));
        stream.extend(command(COMMAND_CHUNK, 9, 9, b"never"));
        let this = collector(&mut rig.e, &stream, 0x200);
        let roots = empty_array(&mut rig.e);
        rig.e.call(0x00dd_f740, &args![this, roots, 0u32]);
        let root = rig.e.mem.u32(rig.e.mem.u32(roots));
        assert_eq!(children(&rig.e, root).len(), 1);
        // No sums: the root's totals are untouched.
        assert_eq!(rig.e.mem.f32(root), 0.0);
        assert!(calls(&rig, FLOAT_TO_INT).is_empty());
    }

    #[test]
    fn write_statistics_builds_the_tree_and_destroys_it() {
        let mut rig = rig();
        prepare_self_name(&mut rig.e);
        let this = collector(&mut rig.e, &command(COMMAND_CHUNK, 2, 2, b"x"), 0x200);
        rig.e.call(0x00dd_f970, &args![this, 0u32, 0u32]);
        // The root was made and, being the only node left in the local array, destroyed
        // with its destructor and a free of its 0x58 bytes (the child stays: only the roots
        // are destroyed here).
        let made = calls(&rig, NODE_CONSTRUCTOR);
        let root = made.iter().find(|c| c[1] == 0).map(|c| c[0]).unwrap();
        assert_eq!(calls(&rig, NODE_DESTRUCTOR), vec![vec![root]]);
        let frees = calls(&rig, FREE_SLOT);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [root, 0x58]);
        // The local array's storage (owned after growing) is freed as `capacity * 4` bytes.
        // Two arrays released: makeTree's local stack and the roots array, each with `capacity * 4`
        // bytes.
        let released = calls(&rig, FREE_BUFFER_SLOT);
        assert_eq!(released.len(), 2);
        assert!(released.iter().all(|c| c[2] == 4));
    }

    /// An object with a vtable whose slot 8 logs and a reference to the collector.
    fn counted_object(rig: &mut Rig) -> u32 {
        let vtable = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(vtable + 8, 0x0900_0110);
        let object = rig.e.mem.alloc(8);
        rig.e.mem.set_u32(object, vtable);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, 0x0900_0110, |_, _| Ret::default());
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, GET_REFERENCED_OBJECT_SIZE, |_, _| {
            returns(0x40)
        });
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, REFERENCED_OBJECT_CONTENTS, |_, _| {
            returns(0x77)
        });
        object
    }

    #[test]
    fn add_referenced_object_records_the_object_once() {
        let mut rig = rig();
        let object = counted_object(&mut rig);
        // A collector whose vtable slot 0x1c is popDir (for `_addKnown`).
        let pop = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(pop + 0x1c, POP_DIR_SLOT);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, POP_DIR_SLOT, |_, _| Ret::default());
        let this = collector(&mut rig.e, &[], 0x100);
        rig.e.mem.set_u32(this, pop);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"obj\0");
        rig.e.call(0x00dd_fa10, &args![this, name, object, 0u8]);
        // Entered in the map with value 1; the start recorded with the object's size; the
        // object asked for its statistics with (collector, 0x77); the pop slot called.
        assert_eq!(*rig.keys.borrow(), vec![object]);
        let mut want = vec![COMMAND_PUSH, 4];
        want.extend_from_slice(&0x40u16.to_le_bytes());
        want.extend_from_slice(&(0x40u16).to_le_bytes());
        want.extend_from_slice(b"obj\0");
        assert_eq!(stream_of(&rig.e, this), want);
        assert_eq!(calls(&rig, 0x0900_0110), vec![vec![object, this, 0x77]]);
        assert_eq!(calls(&rig, POP_DIR_SLOT), vec![vec![this]]);
        // The same object again, a null one, and a stream without room: nothing more happens.
        rig.log.borrow_mut().clear();
        rig.e.call(0x00dd_fa10, &args![this, name, object, 0u8]);
        rig.e.call(0x00dd_fa10, &args![this, name, 0u32, 0u8]);
        assert!(calls(&rig, 0x0900_0110).is_empty());
    }

    #[test]
    fn add_referenced_object_with_bit_zero_only_asks_the_object() {
        let mut rig = rig();
        let object = counted_object(&mut rig);
        let this = collector(&mut rig.e, &[], 0x100);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"obj\0");
        rig.e.call(0x00dd_fa10, &args![this, name, object, 1u8]);
        assert!(stream_of(&rig.e, this).is_empty());
        assert_eq!(calls(&rig, 0x0900_0110), vec![vec![object, this, 0x77]]);
        assert!(calls(&rig, POP_DIR_SLOT).is_empty());
        // No room: not even entered.
        let tight = collector(&mut rig.e, &[0; 8], 0x20);
        let other = counted_object(&mut rig);
        rig.e.call(0x00dd_fa10, &args![tight, name, other, 1u8]);
        assert!(!rig.keys.borrow().contains(&other));
    }

    #[test]
    fn add_object_records_the_class_contents() {
        let mut rig = rig();
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, CLASS_GET_OBJECT_SIZE, |_, _| {
            returns(0x30)
        });
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, ADD_CLASS_CONTENTS_ALL, |_, _| {
            Ret::default()
        });
        let pop = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(pop + 0x1c, POP_DIR_SLOT);
        let log = Rc::clone(&rig.log);
        double(&mut rig.e, &log, POP_DIR_SLOT, |_, _| Ret::default());
        let this = collector(&mut rig.e, &[], 0x100);
        rig.e.mem.set_u32(this, pop);
        let name = rig.e.mem.alloc(8);
        rig.e.mem.write(name, b"obj\0");
        rig.e
            .call(0x00dd_fab0, &args![this, 0xc1a5u32, name, 0x9000u32, 0u8]);
        assert_eq!(*rig.keys.borrow(), vec![0x9000]);
        assert_eq!(
            calls(&rig, ADD_CLASS_CONTENTS_ALL),
            vec![vec![0x9000, 0xc1a5, this]]
        );
        assert_eq!(calls(&rig, POP_DIR_SLOT).len(), 1);
        let mut want = vec![COMMAND_PUSH, 4];
        want.extend_from_slice(&0x30u16.to_le_bytes());
        want.extend_from_slice(&0x30u16.to_le_bytes());
        want.extend_from_slice(b"obj\0");
        assert_eq!(stream_of(&rig.e, this), want);
        // Bit 0: contents only; an object already seen: nothing.
        rig.log.borrow_mut().clear();
        rig.e
            .call(0x00dd_fab0, &args![this, 0xc1a5u32, name, 0x9100u32, 1u8]);
        assert_eq!(calls(&rig, ADD_CLASS_CONTENTS_ALL).len(), 1);
        assert!(calls(&rig, POP_DIR_SLOT).is_empty());
        rig.e
            .call(0x00dd_fab0, &args![this, 0xc1a5u32, name, 0x9100u32, 1u8]);
        assert_eq!(calls(&rig, ADD_CLASS_CONTENTS_ALL).len(), 1);
    }
}
