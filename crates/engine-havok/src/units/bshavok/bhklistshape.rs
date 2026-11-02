//! `BSHavok/bhklistshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkListShape` (0x14 bytes: the Havok shape at `+8`, `pInfo` at `+0xC`, vtable
//! `010c485c`, RTTI-confirmed). The Havok shape's container sub-object at `+0x10` has a
//! vtable whose slot `4` gives the child count and whose slots `8`, `0xC` and `0x10` begin
//! the key walk, step it and give the per-child word for a key. The children themselves are
//! the 0x10-byte entries of the array at `+0x18`; the first word of an entry is a Havok
//! shape whose `+8` is the `bhkShape` wrapper. `pInfo` is a 0x1C-byte block holding two
//! `hkArray`s (data, size, capacity-and-flags): the Havok shapes at `+4` and the per-child
//! words at `+0x10`.
//!
//! The functions marked `Translation incomplete` have exception-unwinding frames
//! (`FS:[0]` registrations) or a stack-cookie check (`00ec408c`), which are not modeled.
//! Four functions raise the per-thread allocation word at `TLS + 0x2b4` to 0x10 for their
//! duration (`enter_scope`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhklimitedhingeconstraint::{
    allocate_sized, allocator, enter_scope, increment, push_viewer_string, ALLOCATE, MEMORY_ROUTER,
    POOL_ALLOCATE, POOL_OBJECT, REMOVE_REFERENCE,
};

/// The vtables of `bhkShape`, `bhkShapeCollection` and `bhkListShape`, with the instance
/// counters of the three.
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_COLLECTION: u32 = 0x010c_44e4;
const VTABLE_LIST_SHAPE: u32 = 0x010c_485c;
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_COLLECTION: u32 = 0x0126_86d0;
const COUNT_LIST_SHAPE: u32 = 0x0126_81a0;
/// `bhkSerializable::Kill` (`00c876a0`), `CloneCritLock` (`00c876f0`) and
/// `CloneCritUnlock` (`00c87700`); the `bhkRefObject` constructor (`00c87c30`).
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_CRIT_LOCK: u32 = 0x00c8_76f0;
const CLONE_CRIT_UNLOCK: u32 = 0x00c8_7700;
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkShapeCollection`'s destructor (`00cd2de0`) and `GetViewerStrings` (`00cd2fd0`).
const COLLECTION_DESTRUCT: u32 = 0x00cd_2de0;
const COLLECTION_GET_VIEWER_STRINGS: u32 = 0x00cd_2fd0;
/// `bhkShape`'s `CopyMembers` (`00c84df0`), `LoadBinary` (`00c84d00`), `SaveBinary`
/// (`00c84d50`), `Link` (`00c84e50`), `LinkObject` (`00c84cc0`), `CopyCinfo` (`00c84e20`)
/// and `CanShare` (`00c84ec0`).
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_LOAD_BINARY: u32 = 0x00c8_4d00;
const SHAPE_SAVE_BINARY: u32 = 0x00c8_4d50;
const SHAPE_LINK: u32 = 0x00c8_4e50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
const SHAPE_COPY_CINFO: u32 = 0x00c8_4e20;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
/// `hkpListShape`'s constructor (`00cf5640`, `(block, children, count, 1)`) and its
/// add-child function (`00cf50a0`, `(list, key, shape)`).
const LIST_CONSTRUCT: u32 = 0x00cf_5640;
const LIST_ADD_CHILD: u32 = 0x00cf_50a0;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`), `GetNumberOfLinkIDs` (`00a63460`) and
/// `GetObjectFromLinkID` (`00a63430`).
const READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `bhkStream::SwapHKArray` (`00cf0930`, cdecl `(info, count, array)`), `LoadBinary_ov2`
/// (`00cf10f0`, cdecl `(stream, array)`) and `SaveBinary_ov3` (`00cf0e90`, cdecl
/// `(stream, array)`).
const SWAP_HK_ARRAY: u32 = 0x00cf_0930;
const STREAM_LOAD_ARRAY: u32 = 0x00cf_10f0;
const STREAM_SAVE_ARRAY: u32 = 0x00cf_0e90;
/// `00aa0ad0`: a jump to `00a81fe0`, called with the stream by `RegisterStreamables`.
const REGISTER_STREAMABLES_BASE: u32 = 0x00aa_0ad0;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, 4)`) and
/// the make-room-for-one-more function (`00c90940`, cdecl `(allocator, array, 4)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The `bhkMoppBvTreeShape` constructor (`00cca4e0`, `(block, shape)`), the RTTI records
/// `LinkObject` tests, `IsKindOf`-style `00653290` (`(object, rtti)`) and the formatting
/// function `00406d00` (cdecl `(buffer, size, format, ...)`).
const MOPP_CONSTRUCT: u32 = 0x00cc_a4e0;
const RTTI_WRAPPED_KIND: u32 = 0x0126_82e8;
const RTTI_WRAPPED_KIND_OTHER: u32 = 0x0126_82dc;
const RTTI_CONVEX_LIST: u32 = 0x0126_86a0;
const RTTI_CONVEX_LIST_OTHER: u32 = 0x0126_8264;
const IS_KIND_OF: u32 = 0x0065_3290;
const FORMAT: u32 = 0x0040_6d00;
/// The `NiGetViewerString` overloads for a name (`00a7d330`) and a name with an integer
/// (`00a7d4d0`), the global holding the first name (`012681a8`), the second name's text
/// (`010c41b0`) and the child format (`010c6040`).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_NAME_INT: u32 = 0x00a7_d4d0;
const NAME_GLOBAL: u32 = 0x0126_81a8;
const NAME_COUNT: u32 = 0x010c_41b0;
const FORMAT_CHILD: u32 = 0x010c_6040;
/// The old-version warning's format (`010c5fe0`).
const FORMAT_OLD_VERSION: u32 = 0x010c_5fe0;
/// The pool-owned `pInfo` block's destructor (`00c71120`) and the pool's free (`00aa4060`).
const INFO_DESTRUCT: u32 = 0x00c7_1120;
const POOL_FREE: u32 = 0x00aa_4060;
/// Size of the `pInfo` block and of the Havok list-shape block.
const INFO_SIZE: u32 = 0x1c;
const LIST_SIZE: u32 = 0x70;
/// `CopyCinfo` of this unit (`00c8b8a0`), called by `CreateCinfo`.
const COPY_CINFO: u32 = 0x00c8_b8a0;
/// An empty `hkArray`'s capacity word: no data of its own.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The container sub-object of the Havok shape at `+8` of the wrapper, 0 when there is none.
fn container_of(e: &mut Engine, this: u32) -> u32 {
    if this != 0 {
        let shape = e.mem.u32(this + 8);
        if shape != 0 {
            return shape + 0x10;
        }
    }
    0
}

/// The child count: slot `4` of the container, 0 when the wrapper has no Havok shape.
fn child_count(e: &mut Engine, this: u32) -> u32 {
    let container = container_of(e, this);
    if container == 0 {
        return 0;
    }
    e.vcall(container, 4, &args![]).u32()
}

/// The child entry at byte offset `offset` of the child array (`shape + 0x18`), 0 when the
/// wrapper has no Havok shape.
fn child_entry(e: &mut Engine, this: u32, offset: u32) -> u32 {
    if this == 0 {
        return 0;
    }
    let shape = e.mem.u32(this + 8);
    if shape == 0 {
        return 0;
    }
    let children = e.mem.u32(shape + 0x18);
    e.mem.u32(children.wrapping_add(offset))
}

/// The `bhkShape` wrapper of the child at byte offset `offset`: 0 when the wrapper has no
/// Havok shape, the entry is empty or its shape has no wrapper.
fn child_wrapper(e: &mut Engine, this: u32, offset: u32) -> u32 {
    let entry = child_entry(e, this, offset);
    if entry == 0 {
        return 0;
    }
    e.mem.u32(entry + 8)
}

/// Whether the RTTI chain of `object` (slot `8`, then the parent word at `+4`) passes
/// through `rtti`.
fn derives_from(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut node = e.vcall(object, 8, &args![]).u32();
    while node != 0 {
        if node == rtti {
            return true;
        }
        node = e.mem.u32(node + 4);
    }
    false
}

/// Releases the data of the `hkArray` at `array` through the allocator's slot `0x10`
/// `(data, bytes)` unless its capacity word is flagged as not owning it.
fn release_array(e: &mut Engine, heap: u32, array: u32) {
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(heap, 0x10, &args![data, (capacity & CAPACITY_MASK) * 4]);
    }
}

/// Makes the `hkArray` at `array` hold at least `wanted` words (`00c908c0` with the larger
/// of the wanted count and twice the capacity) when it holds fewer.
fn reserve_array(e: &mut Engine, array: u32, wanted: i32) {
    let heap = allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![heap, array, new_capacity as u32, 4u32],
        );
    }
}

/// `hkArray` push-back of a word: room is made by `00c90940` when it is full.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let size = e.mem.u32(array + 4);
    if size == e.mem.u32(array + 8) & CAPACITY_MASK {
        let heap = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![heap, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + size * 4, value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Builds a `bhkListShape` in the block: the `bhkRefObject` constructor, then the three
/// vtables in turn with their counters raised, `+0xC` and `+0x10` cleared.
fn construct(e: &mut Engine, object: u32) {
    e.call(REF_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_SHAPE);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    increment(e, COUNT_SHAPE);
    e.mem.set_u32(object, VTABLE_COLLECTION);
    increment(e, COUNT_COLLECTION);
    e.mem.set_u32(object, VTABLE_LIST_SHAPE);
    increment(e, COUNT_LIST_SHAPE);
}

// Translated from 00c8adf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8adf0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::~bhkListShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill` (`00c876a0`), takes one off the instance counter `012681a0`
/// and runs the `bhkShapeCollection` destructor (`00cd2de0`).
pub fn bhk_list_shape_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_LIST_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(COUNT_LIST_SHAPE);
    e.mem.set_u32(COUNT_LIST_SHAPE, count.wrapping_sub(1));
    e.call(COLLECTION_DESTRUCT, &args![this]);
}

// Translated from 00c8ae60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8ae60 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: allocates a 0x14-byte `bhkListShape` (`00aa13e0`) and, when that
/// succeeded, constructs it: the `bhkRefObject` constructor (`00c87c30`), the
/// `bhkShape`, `bhkShapeCollection` and `bhkListShape` vtables in turn, `+0xC` and `+0x10`
/// cleared, the three instance counters (`012680f8`, `012686d0`, `012681a0`) up by one.
/// Returns the object, or 0.
pub fn fn_00c8ae60(e: &mut Engine) -> u32 {
    let object = e.call(ALLOCATE, &args![0x14u32]).u32();
    if object == 0 {
        return 0;
    }
    construct(e, object);
    object
}

// Translated from 00c8af10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8af10 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::CreateClone` (Xbox PDB), vtable slot `0x48`: inside the clone lock
/// (`00c876f0` / `00c87700`) allocates and constructs a `bhkListShape` as `00c8ae60`
/// does, then calls slot `0xC4` (`CopyMembers`) of `this` with `(clone, process_info)`.
/// Answers the clone (0 when the allocation failed). The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_list_shape_create_clone(e: &mut Engine, this: Ptr, process_info: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_CRIT_LOCK, &args![]);
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let clone = if block == 0 {
        0
    } else {
        construct(e, block);
        block
    };
    e.vcall(this.addr(), 0xc4, &args![clone, process_info]);
    e.call(CLONE_CRIT_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00c8b000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::CopyMembers` (Xbox PDB), vtable slot `0xC4`: takes `pInfo` through slot
/// `0xB4` (`CreateCinfo`) and, for each Havok shape in its array at `+4` whose `bhkShape`
/// wrapper (`+8`) exists and cannot be shared (slot `0xD0` with `process_info`), replaces
/// the entry with the Havok shape (`+8`) of the wrapper's clone (slot `0x48` with
/// `process_info`; 0 when there is none). Then `bhkShape::CopyMembers` (`00c84df0`) runs
/// with `(clone, process_info)`. The count is re-read each time round, as signed.
pub fn bhk_list_shape_copy_members(e: &mut Engine, this: Ptr, clone: u32, process_info: u32) {
    let me = this.addr();
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(info + 8) as i32) {
        let entries = e.mem.u32(info + 4);
        let entry = e.mem.u32(entries + index * 4);
        if entry != 0 {
            let wrapper = e.mem.u32(entry + 8);
            if wrapper != 0 && !e.vcall(wrapper, 0xd0, &args![process_info]).bool() {
                let copy = e.vcall(wrapper, 0x48, &args![process_info]).u32();
                let shape = if copy != 0 { e.mem.u32(copy + 8) } else { 0 };
                let entries = e.mem.u32(info + 4);
                e.mem.set_u32(entries + index * 4, shape);
            }
        }
        index += 1;
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, process_info]);
}

// Translated from 00c8b090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8b090 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::Init` (Xbox PDB), vtable slot `0xB0`, with the cinfo `info`: builds the
/// Havok list shape (a 0x70-byte block, size word at `+4`, constructed by `00cf5640` with
/// `info`'s `+4` and `+8` and 1) and adds, through `00cf50a0` and the container's key
/// walk (slots `8` and `0xC`), the first `min(+8, +0x14)` words of the array at `info +
/// 0x10`. The wrapper takes it through slot `0x8C` and releases its own reference
/// (`00c905b0`); then slot `0xBC` (`PostInit`) runs with `info`. The allocation-scope word
/// is 0x10 meanwhile.
pub fn bhk_list_shape_init(e: &mut Engine, this: Ptr, info: u32) {
    let (word, saved) = enter_scope(e);
    let children = e.mem.u32(info + 4);
    let wanted = e.mem.u32(info + 8);
    let block = allocate_sized(e, LIST_SIZE);
    let list = e
        .call(LIST_CONSTRUCT, &args![block, children, wanted, 1u32])
        .u32();
    let key = e.vcall(list + 0x10, 8, &args![]).u32();
    let count = wanted.min(e.mem.u32(info + 0x14));
    for index in 0..count {
        let shapes = e.mem.u32(info + 0x10);
        let shape = e.mem.u32(shapes + index * 4);
        e.call(LIST_ADD_CHILD, &args![list, key, shape]);
        e.vcall(list + 0x10, 0xc, &args![key]);
    }
    e.vcall(this.addr(), 0x8c, &args![list]);
    e.call(REMOVE_REFERENCE, &args![list]);
    e.vcall(this.addr(), 0xbc, &args![info]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c8b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::LoadBinary` (Xbox PDB), vtable slot `0x4C`: reads the link ids
/// (`00a64670`), runs `bhkShape::LoadBinary` (`00c84d00`), takes `pInfo` through slot
/// `0xB4`, byte-swaps its two arrays (`00cf0930` with the size from slot `0xA8`, the
/// array at `+0x10` first) when the stream's byte at `+0x1e5` is not 1, and loads the
/// array at `+0x10` from the stream (`00cf10f0`).
pub fn bhk_list_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.call(READ_MULTIPLE_LINK_IDS, &args![stream]);
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    if e.mem.u8(stream + 0x1e5) != 1 {
        let size = e.vcall(me, 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, info + 0x10]);
        e.call(SWAP_HK_ARRAY, &args![info, size, info + 4]);
    }
    e.call(STREAM_LOAD_ARRAY, &args![stream, info + 0x10]);
}

// Translated from 00c8b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::RegisterStreamables` (Xbox PDB), vtable slot `0x54`: calls slot `0x54`
/// of the `bhkShape` wrapper of each child with `stream`, then `00aa0ad0` with
/// `(this, stream)`. Children are counted once, at the start, and looked up afresh each
/// time (a missing Havok shape, entry or wrapper is skipped). The answer is `00aa0ad0`'s: the exe
/// ends on that call and keeps its `AL` (the PDB's `bool`).
pub fn bhk_list_shape_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    let count = child_count(e, this.addr());
    for index in 0..count {
        let wrapper = child_wrapper(e, this.addr(), index * 0x10);
        if wrapper != 0 {
            e.vcall(wrapper, 0x54, &args![stream]);
        }
    }
    e.call(REGISTER_STREAMABLES_BASE, &args![this, stream])
        .bool()
}

// Translated from 00c8b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: takes a reference (`InterlockedIncrement` of `+4`, slot `00fdf1c0`) on the
/// `bhkShape` wrapper of each child, from the last to the first. Nothing happens when
/// `this` or its Havok shape is missing; an empty entry or wrapper is skipped.
pub fn fn_00c8b2b0(e: &mut Engine, this: Ptr) {
    let mut index = child_count(e, this.addr());
    while index != 0 {
        index -= 1;
        let wrapper = child_wrapper(e, this.addr(), index * 0x10);
        if wrapper != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![wrapper + 4]);
        }
    }
}

// Translated from 00c8b310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8b310 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name: drops a reference (`InterlockedDecrement` of `+4`, slot `00fdf1c4`) on the
/// `bhkShape` wrapper of each child, from the last to the first, and calls the wrapper's
/// slot `4` (its delete) when the count reaches 0. The allocation-scope word is 0x10
/// meanwhile; nothing else happens when `this` or its Havok shape is missing.
pub fn fn_00c8b310(e: &mut Engine, this: Ptr) {
    let (word, saved) = enter_scope(e);
    let mut index = child_count(e, this.addr());
    while index != 0 {
        index -= 1;
        let wrapper = child_wrapper(e, this.addr(), index * 0x10);
        if wrapper != 0 && e.call(INTERLOCKED_DECREMENT, &args![wrapper + 4]).u32() == 0 {
            e.vcall(wrapper, 4, &args![]);
        }
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c8b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::Link` (Xbox PDB), vtable slot `0x90`: takes the children's references
/// (`00c8b2b0`) when the low byte of `flag` is not 0, else drops them (`00c8b310`); then
/// `bhkShape::Link` (`00c84e50`) runs with the whole word.
pub fn bhk_list_shape_link(e: &mut Engine, this: Ptr, flag: u32) {
    if flag as u8 != 0 {
        fn_00c8b2b0(e, this);
    } else {
        fn_00c8b310(e, this);
    }
    e.call(SHAPE_LINK, &args![this, flag]);
}

// Translated from 00c8b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::CanShare` (Xbox PDB), vtable slot `0xD0`: `bhkShape::CanShare`
/// (`00c84ec0`) first; while its byte result stays non-zero, ANDs in slot `0xD0` of each
/// child's wrapper with `process_info` (signed child count; an empty entry is skipped).
/// Answers the byte.
pub fn bhk_list_shape_can_share(e: &mut Engine, this: Ptr, process_info: u32) -> u8 {
    let mut result = e.call(SHAPE_CAN_SHARE, &args![this, process_info]).u8();
    if result != 0 {
        let count = child_count(e, this.addr()) as i32;
        let mut index = 0;
        while index < count {
            let wrapper = child_wrapper(e, this.addr(), index as u32 * 0x10);
            if wrapper != 0 {
                result &= e.vcall(wrapper, 0xd0, &args![process_info]).u8();
            }
            index += 1;
            if result == 0 {
                break;
            }
        }
    }
    result
}

// Translated from 00c8b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::BuildDisplayGeometry` (Xbox PDB), vtable slot `0xD4`: calls slot `0xD4`
/// of each child's wrapper with the two arguments (signed child count; a missing
/// entry or wrapper is skipped).
pub fn bhk_list_shape_build_display_geometry(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let count = child_count(e, this.addr()) as i32;
    for index in 0..count.max(0) {
        let wrapper = child_wrapper(e, this.addr(), index as u32 * 0x10);
        if wrapper != 0 {
            e.vcall(wrapper, 0xd4, &args![first, second]);
        }
    }
}

// Translated from 00c8b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::QSubMaterial` (Xbox PDB), vtable slot `0xE0`: walks the children while
/// the result is still 0x24: the child's wrapper (it must exist: the exe reads its vtable
/// unconditionally, which faults on 0) gives an object through slot `0xCC`, and when that
/// exists its slot `0xE0` with `key` becomes the result. When every child left it at
/// 0x24 the answer is the low five bits of the word at `this + 0x10`.
pub fn bhk_list_shape_q_sub_material(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut result = 0x24;
    let count = child_count(e, this.addr());
    for index in 0..count {
        if result != 0x24 {
            return result;
        }
        let wrapper = child_wrapper(e, this.addr(), index * 0x10);
        let material = e.vcall(wrapper, 0xcc, &args![]).u32();
        if material != 0 {
            result = e.vcall(material, 0xe0, &args![key]).u32();
        }
    }
    if result != 0x24 {
        return result;
    }
    e.mem.u32(this.addr() + 0x10) & 0x1f
}

// Translated from 00c8b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8b5e0 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkListShape::LinkObject` (Xbox PDB), vtable slot `0x50`, with the stream: takes
/// `pInfo` through slot `0xB4`, empties the Havok-shape array at `+4` (data released
/// through the allocator when it owns it) and reserves room for the stream's link-id
/// count (`00a63460`). Each link id (`00a63430`) that resolves is replaced by a new
/// `bhkMoppBvTreeShape` wrapper (0x14 bytes, `00cca4e0`) when its RTTI chain passes
/// through `012682e8` or `00653290` accepts it with `012682dc`; the object's Havok shape
/// (`+8`, 0 for none) is appended. When the stream's version (`+4`) is below 0x11 and
/// every shape in the array is a `012686a0` or `01268264` kind (checking at most the
/// arrays's entries that have wrappers) the warning text naming `stream + 0xE0` and
/// the version is formatted (`00406d00`, 1000-byte buffer, 1000 limit); then
/// `bhkShape::LinkObject` (`00c84cc0`) runs. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_list_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let info = e.with_stack(4, |e, created| e.vcall(me, 0xb4, &args![created]).u32());
    let links = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let array = info + 4;
    let heap = allocator(e);
    e.mem.set_u32(array + 4, 0);
    release_array(e, heap, array);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, EMPTY_CAPACITY);
    reserve_array(e, array, links as i32);
    for _ in 0..links {
        let mut object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if object == 0 {
            continue;
        }
        let wrapped = derives_from(e, object, RTTI_WRAPPED_KIND);
        if wrapped
            || e.call(IS_KIND_OF, &args![object, RTTI_WRAPPED_KIND_OTHER])
                .u32()
                != 0
        {
            let block = e.call(ALLOCATE, &args![0x14u32]).u32();
            let wrapper = if block != 0 {
                e.call(MOPP_CONSTRUCT, &args![block, object]).u32()
            } else {
                0
            };
            if wrapper != 0 {
                object = wrapper;
            }
        }
        let shape = e.mem.u32(object + 8);
        push_word(e, array, shape);
    }
    if e.mem.u32(stream + 4) < 0x11 {
        let count = e.mem.u32(info + 8);
        let mut stopped = false;
        for index in 0..count {
            let entries = e.mem.u32(array);
            let entry = e.mem.u32(entries + index * 4);
            if entry == 0 {
                continue;
            }
            let wrapper = e.mem.u32(entry + 8);
            if wrapper == 0 {
                continue;
            }
            if !derives_from(e, wrapper, RTTI_CONVEX_LIST)
                && !derives_from(e, wrapper, RTTI_CONVEX_LIST_OTHER)
            {
                stopped = true;
                break;
            }
        }
        if !stopped {
            let version = e.mem.u32(stream + 4);
            e.with_stack(1000, |e, buffer| {
                e.call(
                    FORMAT,
                    &args![buffer, 1000u32, FORMAT_OLD_VERSION, stream + 0xe0, version],
                );
            });
        }
    }
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
    e.mem.set_u32(word, saved);
}

// Translated from 00c8b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::CopyCinfo` (Xbox PDB; called by `CreateCinfo`, `00c8b9e0`), with the
/// cinfo `info`: `bhkShape::CopyCinfo` (`00c84e20`) runs, then both
/// arrays of `info` are given the child count (the array at `+4` emptied first, each
/// reserved through `00c908c0` when its capacity is smaller, signed) and filled: the
/// child entry's first word into the array at `+4` and, walking the container's keys
/// (slot `8` first, then slot `0xC`), slot `0x10` of the container for the key into the
/// array at `+0x10` (0 for both when the wrapper has no Havok shape).
pub fn bhk_list_shape_copy_cinfo(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    e.call(SHAPE_COPY_CINFO, &args![this, info]);
    let count = child_count(e, me);
    let shapes = info + 4;
    e.mem.set_u32(info + 8, 0);
    reserve_array(e, shapes, count as i32);
    e.mem.set_u32(info + 8, count);
    let words = info + 0x10;
    reserve_array(e, words, count as i32);
    e.mem.set_u32(info + 0x14, count);
    let container = container_of(e, me);
    let mut key = if container != 0 {
        e.vcall(container, 8, &args![]).u32()
    } else {
        0
    };
    for index in 0..count {
        let entry = child_entry(e, me, index * 0x10);
        let data = e.mem.u32(info + 4);
        e.mem.set_u32(data + index * 4, entry);
        let container = container_of(e, me);
        let word = if container != 0 {
            e.vcall(container, 0x10, &args![key]).u32()
        } else {
            0
        };
        let data = e.mem.u32(info + 0x10);
        e.mem.set_u32(data + index * 4, word);
        let container = container_of(e, me);
        key = if container != 0 {
            e.vcall(container, 0xc, &args![key]).u32()
        } else {
            0
        };
    }
}

// Translated from 00c8b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8b9e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::CreateCinfo` (Xbox PDB), vtable slot `0xB4`: the byte at `created` is set
/// to whether `pInfo` (`+0xC`) was empty. If so, a 0x1C-byte block is taken from the pool
/// (`00aa3e40`) with two empty arrays (`+4` and `+0x10`, capacity word `0x80000000`) and
/// becomes `pInfo` (0 when the allocation failed); when the Havok shape (`+8`) exists
/// `CopyCinfo` (`00c8b8a0`) fills it. Answers `pInfo`. The allocation-scope word is 0x10
/// meanwhile.
pub fn bhk_list_shape_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let me = this.addr();
    let was_empty = e.mem.u32(me + 0xc) == 0;
    e.mem.set_u8(created, was_empty as u8);
    if was_empty {
        let block = e.call(POOL_ALLOCATE, &args![POOL_OBJECT, INFO_SIZE]).u32();
        if block != 0 {
            e.mem.set_u32(block, 0);
            e.mem.set_u32(block + 4, 0);
            e.mem.set_u32(block + 8, 0);
            e.mem.set_u32(block + 0xc, EMPTY_CAPACITY);
            e.mem.set_u32(block + 0x10, 0);
            e.mem.set_u32(block + 0x14, 0);
            e.mem.set_u32(block + 0x18, EMPTY_CAPACITY);
        }
        e.mem.set_u32(me + 0xc, block);
        if e.mem.u32(me + 8) != 0 {
            e.call(COPY_CINFO, &args![this, block]);
        }
    }
    e.mem.set_u32(word, saved);
    e.mem.u32(me + 0xc)
}

// Translated from 00c8baa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8baa0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::SaveBinary` (Xbox PDB), vtable slot `0x58`: takes `pInfo` through slot
/// `0xB4` and moves both its arrays out into locals, leaving them empty. When the stream's
/// byte at `+0x1e4` is not 1 the (now empty) arrays are swapped (`00cf0930`, size from
/// slot `0xA8`, the array at `+0x10` first) before and after. The shape count is written
/// through the writer at `stream + 0x250` (slot `0xC`, 4 bytes, counted), each shape's
/// `+8` (0 for an empty entry) goes to stream slot `0x30`, then `bhkShape::SaveBinary`
/// (`00c84d50`) and the local word array (`00cf0e90`) are written. Both local arrays are
/// released through the allocator, the word array first.
pub fn bhk_list_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    // The exe's frame (exception record, `SUB ESP,0x30`, four pushes and the cookie): 0x50
    // bytes. The created byte at +0x17 (00c8bad2), the count word at +0x1c and its size word
    // at +0x24 (00c8bb90, 00c8bba1), the local word array at +0x2c (00c8bc12).
    e.with_stack(0x50, |e, frame| {
        save_binary_in_frame(e, this, stream, frame.addr())
    });
}

/// [`bhk_list_shape_save_binary`] in the exe's frame (`frame` is its lowest address).
fn save_binary_in_frame(e: &mut Engine, this: Ptr, stream: u32, frame: u32) {
    let me = this.addr();
    let info = e.vcall(me, 0xb4, &args![frame + 0x17]).u32();
    let shapes = info + 4;
    let words = info + 0x10;
    let shapes_data = e.mem.u32(info + 4);
    e.mem.set_u32(info + 4, 0);
    let shapes_size = e.mem.u32(info + 8);
    e.mem.set_u32(info + 8, 0);
    let shapes_capacity = e.mem.u32(info + 0xc);
    e.mem.set_u32(info + 0xc, EMPTY_CAPACITY);
    let local = frame + 0x2c;
    let words_data = e.mem.u32(info + 0x10);
    e.mem.set_u32(info + 0x10, 0);
    let words_size = e.mem.u32(info + 0x14);
    e.mem.set_u32(info + 0x14, 0);
    let words_capacity = e.mem.u32(info + 0x18);
    e.mem.set_u32(info + 0x18, EMPTY_CAPACITY);
    e.mem.set_u32(local, words_data);
    e.mem.set_u32(local + 4, words_size);
    e.mem.set_u32(local + 8, words_capacity);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let size = e.vcall(me, 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, words]);
        e.call(SWAP_HK_ARRAY, &args![info, size, shapes]);
    }
    let io = e.mem.u32(stream + 0x250);
    let count = frame + 0x1c;
    e.mem.set_u32(count, shapes_size);
    e.mem.set_u32(frame + 0x18, io);
    let size_slot = frame + 0x24;
    e.mem.set_u32(size_slot, 4);
    let writer = e.mem.u32(io + 0xc);
    let done = e
        .call(writer, &args![io, count, 4u32, size_slot, 1u32])
        .u32();
    let position = e.mem.u32(io + 4);
    e.mem.set_u32(io + 4, position.wrapping_add(done));
    for index in 0..shapes_size {
        let entry = e.mem.u32(shapes_data + index * 4);
        let shape = if entry != 0 { e.mem.u32(entry + 8) } else { 0 };
        e.vcall(stream, 0x30, &args![shape]);
    }
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    e.call(STREAM_SAVE_ARRAY, &args![stream, local]);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let size = e.vcall(me, 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![info, size, words]);
        e.call(SWAP_HK_ARRAY, &args![info, size, shapes]);
    }
    let heap = allocator(e);
    e.mem.set_u32(local + 4, 0);
    release_array(e, heap, local);
    e.mem.set_u32(local, 0);
    e.mem.set_u32(local + 8, EMPTY_CAPACITY);
    let heap = allocator(e);
    if (shapes_capacity as i32) >= 0 {
        e.vcall(
            heap,
            0x10,
            &args![shapes_data, (shapes_capacity & CAPACITY_MASK) * 4],
        );
    }
}

// Translated from 00c8bcd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8bcd0 - the FS:[0] exception-unwinding frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkListShape::GetViewerStrings` (Xbox PDB), vtable slot `0x60`: runs
/// `bhkShapeCollection::GetViewerStrings` (`00cd2fd0`), then adds to `strings` the name
/// at the global `012681a8` (`NiGetViewerString`, `00a7d330`) and the name `010c41b0`
/// with the child count (`00a7d4d0`), and formats a 32-byte label (`00406d00`, format
/// `010c6040`) for each child index. The allocation-scope word is 0x10 meanwhile.
pub fn bhk_list_shape_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    let (word, saved) = enter_scope(e);
    e.call(COLLECTION_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NAME_GLOBAL);
    let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let count = child_count(e, this.addr());
    let text = e
        .call(VIEWER_STRING_NAME_INT, &args![NAME_COUNT, count])
        .u32();
    push_viewer_string(e, strings, text);
    for index in 0..count as i32 {
        e.with_stack(0x20, |e, buffer| {
            e.call(FORMAT, &args![buffer, 0x20u32, FORMAT_CHILD, index as u32]);
        });
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c8be30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c8be30 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkListShape::KillCinfo` (Xbox PDB), vtable slot `0xA4`: when the low byte of `flag`
/// is not 0 and `pInfo` (`+0xC`) exists, runs the block's destructor (`00c71120`), gives
/// it back to the pool (`00aa4060` on the pool object `011f6238`) and clears `+0xC`. The
/// allocation-scope word is 0x10 meanwhile.
pub fn bhk_list_shape_kill_cinfo(e: &mut Engine, this: Ptr, flag: u32) {
    let (word, saved) = enter_scope(e);
    if flag as u8 != 0 {
        let info = e.mem.u32(this.addr() + 0xc);
        if info != 0 {
            e.call(INFO_DESTRUCT, &args![info]);
            e.call(POOL_FREE, &args![POOL_OBJECT, info]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8adf0, bhk_list_shape_destruct(Ptr)),
        entry!(0x00c8ae60, fn_00c8ae60() -> u32),
        entry!(0x00c8af10, bhk_list_shape_create_clone(Ptr, u32) -> u32),
        entry!(0x00c8b000, bhk_list_shape_copy_members(Ptr, u32, u32)),
        entry!(0x00c8b090, bhk_list_shape_init(Ptr, u32)),
        entry!(0x00c8b1d0, bhk_list_shape_load_binary(Ptr, u32)),
        entry!(0x00c8b240, bhk_list_shape_register_streamables(Ptr, u32) -> bool),
        entry!(0x00c8b2b0, fn_00c8b2b0(Ptr)),
        entry!(0x00c8b310, fn_00c8b310(Ptr)),
        entry!(0x00c8b410, bhk_list_shape_link(Ptr, u32)),
        entry!(0x00c8b440, bhk_list_shape_can_share(Ptr, u32) -> u8),
        entry!(
            0x00c8b4d0,
            bhk_list_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00c8b540, bhk_list_shape_q_sub_material(Ptr, u32) -> u32),
        entry!(0x00c8b5e0, bhk_list_shape_link_object(Ptr, u32)),
        entry!(0x00c8b8a0, bhk_list_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00c8b9e0, bhk_list_shape_create_cinfo(Ptr, u32) -> u32),
        entry!(0x00c8baa0, bhk_list_shape_save_binary(Ptr, u32)),
        entry!(0x00c8bcd0, bhk_list_shape_get_viewer_strings(Ptr, u32)),
        entry!(0x00c8be30, bhk_list_shape_kill_cinfo(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::rc::Rc;

    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    const SUB_FIRST_KEY: u32 = 0x0900_0108;
    const SUB_NEXT_KEY: u32 = 0x0900_010c;
    const SUB_WORD: u32 = 0x0900_0110;
    const WRAPPER_RELEASE_DELETE: u32 = 0x0900_0204;

    /// A list shape (wrapper, Havok shape, container table, children) whose children are
    /// the given wrappers; the container's count double answers the number of wrappers.
    /// Returns the wrapper and the Havok shape.
    fn list(e: &mut Engine, wrappers: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x20);
        let shape = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        let count_function = shape | 0x4000_0000;
        e.mem.set_u32(table + 4, count_function);
        e.mem.set_u32(table + 8, SUB_FIRST_KEY);
        e.mem.set_u32(table + 0xc, SUB_NEXT_KEY);
        e.mem.set_u32(table + 0x10, SUB_WORD);
        e.mem.set_u32(shape + 0x10, table);
        let children = e.mem.alloc(0x10 * wrappers.len() as u32 + 0x10);
        for (i, wrapper) in wrappers.iter().enumerate() {
            let child = e.mem.alloc(0x10);
            e.mem.set_u32(child + 8, *wrapper);
            e.mem.set_u32(children + 0x10 * i as u32, child);
        }
        e.mem.set_u32(shape + 0x18, children);
        e.mem.set_u32(this + 8, shape);
        let count = wrappers.len() as u32;
        e.register_double(count_function, move |_, _| ret(count));
        (this, shape)
    }

    /// The vtable slots the list shape tests dispatch through, as doubles.
    fn recording_wrapper(e: &mut Engine, slots: &[(u32, u32)], log: &Calls) -> u32 {
        for (_, target) in slots {
            record(e, *target, 1, log);
        }
        vobject(e, 0x20, slots)
    }

    #[test]
    fn destruct_sets_the_vtable_kills_and_counts_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_LIST_SHAPE, 3u32);
        let log: Calls = Rc::default();
        record(&mut e, SERIALIZABLE_KILL, 0, &log);
        record(&mut e, COLLECTION_DESTRUCT, 0, &log);
        let this = e.mem.alloc(0x14);
        e.call(0x00c8_adf0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_LIST_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_LIST_SHAPE), 2);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, [SERIALIZABLE_KILL, COLLECTION_DESTRUCT]);
        assert_eq!(calls_to(&log, COLLECTION_DESTRUCT), [[this]]);
    }

    #[test]
    fn fn_00c8ae60_builds_the_object_and_counts_it() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0xc, 5);
        e.mem.set_u32(object + 0x10, 6);
        let log: Calls = Rc::default();
        record(&mut e, ALLOCATE, object, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        assert_eq!(e.call(0x00c8_ae60, &args![]).u32(), object);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x14]]);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_LIST_SHAPE);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (0, 0));
        assert_eq!(e.global::<u32>(COUNT_SHAPE), 1);
        assert_eq!(e.global::<u32>(COUNT_COLLECTION), 1);
        assert_eq!(e.global::<u32>(COUNT_LIST_SHAPE), 1);
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00c8_ae60, &args![]).u32(), 0);
        assert_eq!(calls_to(&log, REF_OBJECT_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_copies_the_members_inside_the_lock() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let clone = e.mem.alloc(0x14);
        let log: Calls = Rc::default();
        record(&mut e, CLONE_CRIT_LOCK, 0, &log);
        record(&mut e, CLONE_CRIT_UNLOCK, 0, &log);
        record(&mut e, ALLOCATE, clone, &log);
        record(&mut e, REF_OBJECT_CONSTRUCT, 0, &log);
        record(&mut e, 0x0900_0300, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0xc4, 0x0900_0300)]);
        assert_eq!(e.call(0x00c8_af10, &args![this, 0x55u32]).u32(), clone);
        assert_eq!(calls_to(&log, 0x0900_0300), [[this, clone, 0x55]]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                CLONE_CRIT_LOCK,
                ALLOCATE,
                REF_OBJECT_CONSTRUCT,
                0x0900_0300,
                CLONE_CRIT_UNLOCK
            ]
        );
        assert_eq!(e.mem.u32(word), 0x77);
        // A failed allocation hands 0 to CopyMembers.
        record(&mut e, ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00c8_af10, &args![this, 0x56u32]).u32(), 0);
        assert_eq!(calls_to(&log, 0x0900_0300)[1], [this, 0, 0x56]);
    }

    #[test]
    fn copy_members_replaces_the_shapes_that_cannot_be_shared() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_COPY_MEMBERS, 0, &log);
        // Wrapper 1 can be shared; wrapper 2 cannot and clones to an object with a Havok
        // shape; wrapper 3 cannot and has no clone.
        let shared = vobject(&mut e, 0x20, &[(0xd0, 0x0900_0310)]);
        record(&mut e, 0x0900_0310, 1, &log);
        let clone_hk = e.mem.alloc(0x10);
        let clone = e.mem.alloc(0x10);
        e.mem.set_u32(clone + 8, clone_hk);
        let second = vobject(&mut e, 0x20, &[(0xd0, 0x0900_0314), (0x48, 0x0900_0318)]);
        record(&mut e, 0x0900_0314, 0, &log);
        record(&mut e, 0x0900_0318, clone, &log);
        let third = vobject(&mut e, 0x20, &[(0xd0, 0x0900_0314), (0x48, 0x0900_031c)]);
        record(&mut e, 0x0900_031c, 0, &log);
        let mut entries = Vec::new();
        for wrapper in [shared, second, third, 0] {
            let entry = e.mem.alloc(0x10);
            e.mem.set_u32(entry + 8, wrapper);
            entries.push(entry);
        }
        entries.push(0);
        let data = e.mem.alloc(0x20);
        for (i, entry) in entries.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *entry);
        }
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 8, 5);
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0320)]);
        e.register_double(0x0900_0320, move |_, _| ret(info));
        e.call(0x00c8_b000, &args![this, 0x1000u32, 0x77u32]);
        assert_eq!(e.mem.u32(data), entries[0]);
        assert_eq!(e.mem.u32(data + 4), clone_hk);
        assert_eq!(e.mem.u32(data + 8), 0);
        assert_eq!(e.mem.u32(data + 12), entries[3]);
        assert_eq!(calls_to(&log, 0x0900_0318), [[second, 0x77]]);
        assert_eq!(calls_to(&log, SHAPE_COPY_MEMBERS), [[this, 0x1000, 0x77]]);
        // No entries: nothing is asked of any wrapper.
        e.mem.set_u32(info + 8, 0);
        e.call(0x00c8_b000, &args![this, 0x1000u32, 0x78u32]);
        assert_eq!(calls_to(&log, 0x0900_0318).len(), 1);
    }

    #[test]
    fn init_builds_the_havok_list_and_adds_the_shapes() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        let list_object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 8, 0x0900_0330);
        e.mem.set_u32(table + 0xc, 0x0900_0334);
        e.mem.set_u32(list_object + 0x10, table);
        record(&mut e, 0x0900_0330, 0x31, &log);
        record(&mut e, 0x0900_0334, 0, &log);
        record(&mut e, LIST_CONSTRUCT, list_object, &log);
        record(&mut e, LIST_ADD_CHILD, 0, &log);
        record(&mut e, REMOVE_REFERENCE, 0, &log);
        record(&mut e, 0x0900_0338, 0, &log);
        record(&mut e, 0x0900_033c, 0, &log);
        let this = vobject(&mut e, 0x14, &[(0x8c, 0x0900_0338), (0xbc, 0x0900_033c)]);
        let shapes = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_u32(shapes + 4 * i, 0x4000 + i);
        }
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 0x7000);
        e.mem.set_u32(info + 8, 3);
        e.mem.set_u32(info + 0x10, shapes);
        e.mem.set_u32(info + 0x14, 2);
        e.call(0x00c8_b090, &args![this, info]);
        assert_eq!(calls_to(&log, LIST_CONSTRUCT), [[block, 0x7000, 3, 1]]);
        assert_eq!(e.mem.u16(block + 4), 0x70);
        // Only min(3, 2) children are added, each with the key from the first-key slot.
        assert_eq!(
            calls_to(&log, LIST_ADD_CHILD),
            [[list_object, 0x31, 0x4000], [list_object, 0x31, 0x4001]]
        );
        assert_eq!(calls_to(&log, 0x0900_0334), [[list_object + 0x10, 0x31]; 2]);
        assert_eq!(calls_to(&log, 0x0900_0338), [[this, list_object]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[list_object]]);
        assert_eq!(calls_to(&log, 0x0900_033c), [[this, info]]);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn load_binary_reads_the_ids_and_swaps_a_foreign_stream() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, READ_MULTIPLE_LINK_IDS, 0, &log);
        record(&mut e, SHAPE_LOAD_BINARY, 0, &log);
        record(&mut e, SWAP_HK_ARRAY, 0, &log);
        record(&mut e, STREAM_LOAD_ARRAY, 0, &log);
        record(&mut e, 0x0900_0340, 0x60, &log);
        let info = e.mem.alloc(0x20);
        e.register_double(0x0900_0344, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xa8, 0x0900_0340), (0xb4, 0x0900_0344)]);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call(0x00c8_b1d0, &args![this, stream]);
        assert_eq!(calls_to(&log, READ_MULTIPLE_LINK_IDS), [[stream]]);
        assert_eq!(calls_to(&log, SHAPE_LOAD_BINARY), [[this, stream]]);
        assert_eq!(calls_to(&log, STREAM_LOAD_ARRAY), [[stream, info + 0x10]]);
        assert!(calls_to(&log, SWAP_HK_ARRAY).is_empty());
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call(0x00c8_b1d0, &args![this, stream]);
        assert_eq!(
            calls_to(&log, SWAP_HK_ARRAY),
            [[info, 0x60, info + 0x10], [info, 0x60, info + 4]]
        );
    }

    #[test]
    fn register_streamables_asks_every_wrapper_then_the_base() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, REGISTER_STREAMABLES_BASE, 0, &log);
        let first = recording_wrapper(&mut e, &[(0x54, 0x0900_0350)], &log);
        let second = recording_wrapper(&mut e, &[(0x54, 0x0900_0350)], &log);
        let (this, shape) = list(&mut e, &[first, 0, second]);
        e.call(0x00c8_b240, &args![this, 0x99u32]);
        assert_eq!(calls_to(&log, 0x0900_0350), [[first, 0x99], [second, 0x99]]);
        assert_eq!(calls_to(&log, REGISTER_STREAMABLES_BASE), [[this, 0x99]]);
        // Without a Havok shape only the base runs.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_b240, &args![this, 0x9au32]);
        assert_eq!(calls_to(&log, 0x0900_0350).len(), 2);
        assert_eq!(calls_to(&log, REGISTER_STREAMABLES_BASE).len(), 2);
        assert_ne!(shape, 0);
    }

    #[test]
    fn register_streamables_answers_the_base() {
        let mut e = Engine::new();
        let (this, _) = list(&mut e, &[]);
        e.mem.set_u32(this + 8, 0);
        e.register(REGISTER_STREAMABLES_BASE, |_, _| 1u32.into_ret());
        assert!(e.call(0x00c8_b240, &args![this, 0x99u32]).bool());
        e.register(REGISTER_STREAMABLES_BASE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00c8_b240, &args![this, 0x99u32]).bool());
    }

    #[test]
    fn fn_00c8b2b0_adds_a_reference_to_each_wrapper_last_first() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, INTERLOCKED_INCREMENT, 2, &log);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x20);
        let (this, _) = list(&mut e, &[first, 0, second]);
        e.call(0x00c8_b2b0, &args![this]);
        assert_eq!(
            calls_to(&log, INTERLOCKED_INCREMENT),
            [[second + 4], [first + 4]]
        );
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_b2b0, &args![this]);
        assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT).len(), 2);
    }

    #[test]
    fn fn_00c8b310_drops_references_and_deletes_at_zero() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, WRAPPER_RELEASE_DELETE, 0, &log);
        let kept = vobject(&mut e, 0x20, &[(4, WRAPPER_RELEASE_DELETE)]);
        let dying = vobject(&mut e, 0x20, &[(4, WRAPPER_RELEASE_DELETE)]);
        e.register_double(INTERLOCKED_DECREMENT, move |_, a| {
            ret(if a[0] == dying + 4 { 0 } else { 1 })
        });
        let (this, _) = list(&mut e, &[kept, dying, 0]);
        e.call(0x00c8_b310, &args![this]);
        assert_eq!(calls_to(&log, WRAPPER_RELEASE_DELETE), [[dying]]);
        assert_eq!(e.mem.u32(word), 0x77);
        // No Havok shape: nothing is dropped and the scope word is still put back.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_b310, &args![this]);
        assert_eq!(calls_to(&log, WRAPPER_RELEASE_DELETE).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn link_takes_or_drops_the_references_then_links_the_shape() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, INTERLOCKED_INCREMENT, 2, &log);
        record(&mut e, INTERLOCKED_DECREMENT, 1, &log);
        record(&mut e, SHAPE_LINK, 0, &log);
        let wrapper = e.mem.alloc(0x20);
        let (this, _) = list(&mut e, &[wrapper]);
        e.call(0x00c8_b410, &args![this, 0x0100u32]);
        assert!(calls_to(&log, INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(calls_to(&log, INTERLOCKED_DECREMENT), [[wrapper + 4]]);
        e.call(0x00c8_b410, &args![this, 0xff01u32]);
        assert_eq!(calls_to(&log, INTERLOCKED_INCREMENT), [[wrapper + 4]]);
        assert_eq!(calls_to(&log, SHAPE_LINK), [[this, 0x0100], [this, 0xff01]]);
    }

    #[test]
    fn can_share_stops_at_the_first_refusal() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        record(&mut e, SHAPE_CAN_SHARE, 1, &log);
        let yes = recording_wrapper(&mut e, &[(0xd0, 0x0900_0360)], &log);
        let no = vobject(&mut e, 0x20, &[(0xd0, 0x0900_0364)]);
        record(&mut e, 0x0900_0364, 0, &log);
        let (this, _) = list(&mut e, &[yes, no, yes]);
        assert_eq!(e.call(0x00c8_b440, &args![this, 0x42u32]).u32() & 0xff, 0);
        assert_eq!(calls_to(&log, 0x0900_0360), [[yes, 0x42]]);
        assert_eq!(calls_to(&log, 0x0900_0364), [[no, 0x42]]);
        let (all, _) = list(&mut e, &[yes, yes]);
        assert_eq!(e.call(0x00c8_b440, &args![all, 0x42u32]).u32() & 0xff, 1);
        // The base class's refusal skips the children.
        record(&mut e, SHAPE_CAN_SHARE, 0, &log);
        let before = calls_to(&log, 0x0900_0360).len();
        assert_eq!(e.call(0x00c8_b440, &args![all, 0x42u32]).u32() & 0xff, 0);
        assert_eq!(calls_to(&log, 0x0900_0360).len(), before);
    }

    #[test]
    fn build_display_geometry_passes_both_words_to_each_wrapper() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let first = recording_wrapper(&mut e, &[(0xd4, 0x0900_0370)], &log);
        let second = recording_wrapper(&mut e, &[(0xd4, 0x0900_0370)], &log);
        let (this, _) = list(&mut e, &[first, second, 0]);
        e.call(0x00c8_b4d0, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(
            calls_to(&log, 0x0900_0370),
            [[first, 0x11, 0x22], [second, 0x11, 0x22]]
        );
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c8_b4d0, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&log, 0x0900_0370).len(), 2);
    }

    #[test]
    fn q_sub_material_takes_the_first_child_that_answers() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let none = vobject(&mut e, 0x20, &[(0xcc, 0x0900_0380)]);
        record(&mut e, 0x0900_0380, 0, &log);
        let source = vobject(&mut e, 0x20, &[(0xe0, 0x0900_0384)]);
        record(&mut e, 0x0900_0384, 7, &log);
        let some = vobject(&mut e, 0x20, &[(0xcc, 0x0900_0388)]);
        record(&mut e, 0x0900_0388, source, &log);
        let (this, _) = list(&mut e, &[none, some, none]);
        e.mem.set_u32(this + 0x10, 0x3f);
        assert_eq!(e.call(0x00c8_b540, &args![this, 0x1234u32]).u32(), 7);
        assert_eq!(calls_to(&log, 0x0900_0384), [[source, 0x1234]]);
        // The third child is never asked.
        assert_eq!(calls_to(&log, 0x0900_0380), [[none]]);
        // No answers: the low five bits of +0x10.
        let (blank, _) = list(&mut e, &[none]);
        e.mem.set_u32(blank + 0x10, 0x3f);
        assert_eq!(e.call(0x00c8_b540, &args![blank, 1u32]).u32(), 0x1f);
        let (empty, _) = list(&mut e, &[]);
        e.mem.set_u32(empty + 0x10, 0x25);
        assert_eq!(e.call(0x00c8_b540, &args![empty, 1u32]).u32(), 0x05);
        // An answer of exactly 0x24 keeps the walk going.
        record(&mut e, 0x0900_0384, 0x24, &log);
        assert_eq!(e.call(0x00c8_b540, &args![this, 1u32]).u32(), 0x1f);
    }

    /// An `hkArray` reserve double that gives the array a fresh 64-word buffer.
    fn array_doubles(e: &mut Engine, log: &Calls) {
        let reserve_log = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let buffer = e.mem.alloc(0x400);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let more_log = Rc::clone(log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            more_log.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let buffer = e.mem.alloc(0x400);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 0x40);
            Ret::default()
        });
    }

    /// An object whose RTTI chain (slot 8) is `chain`, each record's parent at `+4`.
    fn rtti_object(e: &mut Engine, chain: &[u32], hk: u32) -> u32 {
        for pair in chain.windows(2) {
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        let first = chain.first().copied().unwrap_or(0);
        let object = vobject(e, 0x20, &[]);
        let target = object | 0x4000_0000;
        let table = e.mem.u32(object);
        e.mem.set_u32(table + 8, target);
        e.register_double(target, move |_, _| ret(first));
        e.mem.set_u32(object + 8, hk);
        object
    }

    #[test]
    fn link_object_collects_the_havok_shapes_and_wraps_the_mopp_kinds() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let log: Calls = Rc::default();
        array_doubles(&mut e, &log);
        record(&mut e, FREE_TARGET, 0, &log);
        record(&mut e, SHAPE_LINK_OBJECT, 0, &log);
        record(&mut e, FORMAT, 0, &log);
        record(&mut e, GET_NUMBER_OF_LINK_IDS, 3, &log);
        let plain_hk = e.mem.alloc(0x10);
        let plain = rtti_object(&mut e, &[0x0126_86a0], plain_hk);
        let moppable_hk = e.mem.alloc(0x10);
        let moppable = rtti_object(&mut e, &[0x0126_82e8], moppable_hk);
        let wrapper_hk = e.mem.alloc(0x10);
        let wrapper = e.mem.alloc(0x10);
        e.mem.set_u32(wrapper + 8, wrapper_hk);
        record(&mut e, ALLOCATE, wrapper, &log);
        record(&mut e, MOPP_CONSTRUCT, wrapper, &log);
        record(&mut e, IS_KIND_OF, 0, &log);
        let ids = Rc::new(std::cell::RefCell::new(vec![moppable, 0, plain]));
        let remaining = Rc::clone(&ids);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| {
            ret(remaining.borrow_mut().remove(0))
        });
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0xc, 0x8000_0000);
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0390)]);
        e.register_double(0x0900_0390, move |_, _| ret(info));
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 4, 0x20);
        e.call(0x00c8_b5e0, &args![this, stream]);
        let data = e.mem.u32(info + 4);
        assert_eq!(e.mem.u32(info + 8), 2);
        assert_eq!(e.mem.u32(data), wrapper_hk);
        assert_eq!(e.mem.u32(data + 4), plain_hk);
        assert_eq!(calls_to(&log, ALLOCATE), [[0x14]]);
        assert_eq!(calls_to(&log, MOPP_CONSTRUCT), [[wrapper, moppable]]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[0][2..], [3, 4]);
        assert_eq!(calls_to(&log, SHAPE_LINK_OBJECT), [[this, stream]]);
        assert!(calls_to(&log, FORMAT).is_empty());
        assert_eq!(e.mem.u32(word), 0x77);
        // The kind test of 00653290 also wraps; a failed allocation keeps the object.
        let other = rtti_object(&mut e, &[], 0x5555);
        *ids.borrow_mut() = vec![other];
        record(&mut e, GET_NUMBER_OF_LINK_IDS, 1, &log);
        record(&mut e, IS_KIND_OF, 1, &log);
        record(&mut e, ALLOCATE, 0, &log);
        e.mem.set_u32(info + 8, 7);
        e.call(0x00c8_b5e0, &args![this, stream]);
        assert_eq!(
            calls_to(&log, IS_KIND_OF).last().unwrap()[..],
            [other, 0x0126_82dc]
        );
        assert_eq!(e.mem.u32(info + 8), 1);
        assert_eq!(e.mem.u32(e.mem.u32(info + 4)), 0x5555);
    }

    #[test]
    fn link_object_warns_about_old_streams_with_only_convex_lists() {
        let (mut e, _, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        let log: Calls = Rc::default();
        array_doubles(&mut e, &log);
        record(&mut e, FREE_TARGET, 0, &log);
        record(&mut e, SHAPE_LINK_OBJECT, 0, &log);
        record(&mut e, FORMAT, 0, &log);
        record(&mut e, GET_NUMBER_OF_LINK_IDS, 1, &log);
        let convex = rtti_object(&mut e, &[0x0126_8264], 0);
        let convex_hk = e.mem.alloc(0x10);
        e.mem.set_u32(convex_hk + 8, convex);
        let linked = rtti_object(&mut e, &[], convex_hk);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| ret(linked));
        record(&mut e, IS_KIND_OF, 0, &log);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0xc, 0x8000_0000);
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_0390)]);
        e.register_double(0x0900_0390, move |_, _| ret(info));
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 4, 0x10);
        e.call(0x00c8_b5e0, &args![this, stream]);
        let formats = calls_to(&log, FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(
            formats[0][1..],
            [1000, FORMAT_OLD_VERSION, stream + 0xe0, 0x10]
        );
        // A shape of another kind ends the walk without the warning.
        let node = e.mem.alloc(8);
        let other = rtti_object(&mut e, &[node], 0);
        let other_hk = e.mem.alloc(0x10);
        e.mem.set_u32(other_hk + 8, other);
        let linked = rtti_object(&mut e, &[], other_hk);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| ret(linked));
        e.call(0x00c8_b5e0, &args![this, stream]);
        assert_eq!(calls_to(&log, FORMAT).len(), 1);
        // A current stream version never warns.
        e.mem.set_u32(stream + 4, 0x11);
        let linked = rtti_object(&mut e, &[], convex_hk);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| ret(linked));
        e.call(0x00c8_b5e0, &args![this, stream]);
        assert_eq!(calls_to(&log, FORMAT).len(), 1);
    }

    #[test]
    fn copy_cinfo_fills_both_arrays_from_the_children() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        array_doubles(&mut e, &log);
        record(&mut e, SHAPE_COPY_CINFO, 0, &log);
        let (this, shape) = list(&mut e, &[0x10, 0x20]);
        let children = e.mem.u32(shape + 0x18);
        let first = e.mem.u32(children);
        let second = e.mem.u32(children + 0x10);
        e.register_double(SUB_FIRST_KEY, |_, _| ret(100));
        e.register_double(SUB_NEXT_KEY, move |_, a| ret(a[1] + 1));
        e.register_double(SUB_WORD, |_, a| ret(a[1] * 2));
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0xc, 0x8000_0000);
        e.mem.set_u32(info + 0x18, 0x8000_0000);
        e.call(0x00c8_b8a0, &args![this, info]);
        assert_eq!(calls_to(&log, SHAPE_COPY_CINFO), [[this, info]]);
        assert_eq!((e.mem.u32(info + 8), e.mem.u32(info + 0x14)), (2, 2));
        let shapes = e.mem.u32(info + 4);
        let words = e.mem.u32(info + 0x10);
        assert_eq!((e.mem.u32(shapes), e.mem.u32(shapes + 4)), (first, second));
        assert_eq!((e.mem.u32(words), e.mem.u32(words + 4)), (200, 202));
        let reserves = calls_to(&log, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(
            (reserves[0][1], reserves[0][2], reserves[0][3]),
            (info + 4, 2, 4)
        );
        assert_eq!(reserves[1][1], info + 0x10);
        // No Havok shape: nothing to copy and no reserve is needed.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(info + 0xc, 0x4);
        e.mem.set_u32(info + 0x18, 0x4);
        e.call(0x00c8_b8a0, &args![this, info]);
        assert_eq!((e.mem.u32(info + 8), e.mem.u32(info + 0x14)), (0, 0));
        assert_eq!(calls_to(&log, ARRAY_RESERVE).len(), 2);
    }

    #[test]
    fn create_cinfo_builds_the_block_once_and_fills_it() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let block = e.mem.alloc(0x1c);
        for offset in 0..7 {
            e.mem.set_u32(block + 4 * offset, 0xdead);
        }
        record(&mut e, POOL_ALLOCATE, block, &log);
        record(&mut e, COPY_CINFO, 0, &log);
        let (this, shape) = list(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0);
        let created = e.mem.alloc(4);
        assert_eq!(e.call(0x00c8_b9e0, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&log, POOL_ALLOCATE), [[POOL_OBJECT, 0x1c]]);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(words, [0, 0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
        assert_eq!(e.mem.u32(this + 0xc), block);
        assert_eq!(calls_to(&log, COPY_CINFO), [[this, block]]);
        // Second time: reuse, flag 0.
        assert_eq!(e.call(0x00c8_b9e0, &args![this, created]).u32(), block);
        assert_eq!(e.mem.u8(created), 0);
        assert_eq!(calls_to(&log, POOL_ALLOCATE).len(), 1);
        // No Havok shape: no fill; a failed allocation leaves pInfo 0.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0);
        record(&mut e, POOL_ALLOCATE, 0, &log);
        assert_eq!(e.call(0x00c8_b9e0, &args![this, created]).u32(), 0);
        assert_eq!(calls_to(&log, COPY_CINFO).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
        assert_ne!(shape, 0);
    }

    #[test]
    fn save_binary_moves_the_arrays_out_and_writes_the_shapes() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x250);
        record(&mut e, SHAPE_SAVE_BINARY, 0, &log);
        record(&mut e, STREAM_SAVE_ARRAY, 0, &log);
        record(&mut e, SWAP_HK_ARRAY, 0, &log);
        record(&mut e, FREE_TARGET, 0, &log);
        record(&mut e, 0x0900_03a0, 0, &log);
        let stream_table = e.mem.alloc(0x100);
        e.mem.set_u32(stream_table + 0x30, 0x0900_03a0);
        e.mem.set_u32(stream, stream_table);
        let hk_a = e.mem.alloc(0x10);
        let hk_b = e.mem.alloc(0x10);
        let shape_a = e.mem.alloc(0x10);
        e.mem.set_u32(shape_a + 8, hk_a);
        let shape_b = e.mem.alloc(0x10);
        e.mem.set_u32(shape_b + 8, hk_b);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, shape_a);
        e.mem.set_u32(data + 4, 0);
        e.mem.set_u32(data + 8, shape_b);
        let words = e.mem.alloc(0x10);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 8, 3);
        e.mem.set_u32(info + 0xc, 4);
        e.mem.set_u32(info + 0x10, words);
        e.mem.set_u32(info + 0x14, 3);
        e.mem.set_u32(info + 0x18, 0x8000_0003);
        e.register_double(0x0900_03a4, move |_, _| ret(info));
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_03a4), (0xa8, 0x0900_03a8)]);
        record(&mut e, 0x0900_03a8, 0x60, &log);
        let local = Rc::new(std::cell::Cell::new((0u32, 0u32, 0u32)));
        let seen = Rc::clone(&local);
        e.register_double(STREAM_SAVE_ARRAY, move |e, a| {
            seen.set((e.mem.u32(a[1]), e.mem.u32(a[1] + 4), e.mem.u32(a[1] + 8)));
            Ret::default()
        });
        e.call(0x00c8_baa0, &args![this, stream]);
        // The cinfo's arrays are emptied; the count goes out through the writer.
        assert_eq!(
            [0x4, 0x8, 0xc, 0x10, 0x14, 0x18].map(|o| e.mem.u32(info + o)),
            [0, 0, 0x8000_0000, 0, 0, 0x8000_0000]
        );
        let writes = calls_to(&log, 0x0900_000c);
        assert_eq!(writes.len(), 1);
        assert_eq!((writes[0][0], writes[0][2]), (io, 4));
        assert_eq!(
            calls_to(&log, 0x0900_03a0),
            [[stream, hk_a], [stream, 0], [stream, hk_b]]
        );
        assert_eq!(calls_to(&log, SHAPE_SAVE_BINARY), [[this, stream]]);
        assert_eq!(local.get(), (words, 3, 0x8000_0003));
        assert!(calls_to(&log, SWAP_HK_ARRAY).is_empty());
        // Only the shape array (capacity 4, owned) is released; the word array was not.
        assert_eq!(calls_to(&log, FREE_TARGET).len(), 1);
        assert_eq!(calls_to(&log, FREE_TARGET)[0][1..], [data, 16]);
        // A foreign byte order swaps before and after.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 8, 0);
        e.mem.set_u32(info + 0xc, 0x8000_0000);
        e.call(0x00c8_baa0, &args![this, stream]);
        assert_eq!(calls_to(&log, SWAP_HK_ARRAY).len(), 4);
    }

    #[test]
    fn save_binary_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut e, _, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        let stream = stream_fixture(&mut e, &log);
        let io = e.mem.u32(stream + 0x250);
        record(&mut e, SHAPE_SAVE_BINARY, 0, &log);
        record(&mut e, STREAM_SAVE_ARRAY, 0, &log);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0xc, 0x8000_0000);
        e.mem.set_u32(info + 0x18, 0x8000_0000);
        record(&mut e, 0x0900_03a4, info, &log);
        let this = vobject(&mut e, 0x14, &[(0xb4, 0x0900_03a4)]);
        let frame = e.next_stack_local(0x50);
        e.call(0x00c8_baa0, &args![this, stream]);
        // The created byte at +0x17, the count at +0x1c with its size word at +0x24, the
        // local word array at +0x2c.
        assert_eq!(calls_to(&log, 0x0900_03a4), [[this, frame + 0x17]]);
        assert_eq!(
            calls_to(&log, 0x0900_000c),
            [[io, frame + 0x1c, 4, frame + 0x24, 1]]
        );
        assert_eq!(calls_to(&log, STREAM_SAVE_ARRAY), [[stream, frame + 0x2c]]);
    }

    #[test]
    fn get_viewer_strings_adds_both_names_and_formats_each_child() {
        let (mut e, word, _) = engine_with_allocator();
        e.map(0x0126_8000, 0x1000);
        e.set_global(NAME_GLOBAL, 0x9100u32);
        let log: Calls = Rc::default();
        record(&mut e, COLLECTION_GET_VIEWER_STRINGS, 0, &log);
        record(&mut e, VIEWER_STRING_NAME, 0x3000, &log);
        record(&mut e, VIEWER_STRING_NAME_INT, 0x3001, &log);
        record(&mut e, FORMAT, 0, &log);
        e.register_double(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(0x0900_1000 + 4 * a[1], text);
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.map(0x0900_1000, 0x100);
        let (this, _) = list(&mut e, &[0, 0, 0]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00c8_bcd0, &args![this, strings]);
        assert_eq!(
            calls_to(&log, COLLECTION_GET_VIEWER_STRINGS),
            [[this, strings]]
        );
        assert_eq!(calls_to(&log, VIEWER_STRING_NAME), [[0x9100]]);
        assert_eq!(calls_to(&log, VIEWER_STRING_NAME_INT), [[NAME_COUNT, 3]]);
        assert_eq!(
            (e.mem.u32(0x0900_1000), e.mem.u32(0x0900_1004)),
            (0x3000, 0x3001)
        );
        let formats = calls_to(&log, FORMAT);
        assert_eq!(formats.len(), 3);
        assert_eq!(
            formats
                .iter()
                .map(|f| (f[1], f[2], f[3]))
                .collect::<Vec<_>>(),
            [
                (0x20, FORMAT_CHILD, 0),
                (0x20, FORMAT_CHILD, 1),
                (0x20, FORMAT_CHILD, 2)
            ]
        );
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn kill_cinfo_frees_the_block_only_when_asked() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, INFO_DESTRUCT, 0, &log);
        record(&mut e, POOL_FREE, 0, &log);
        let (this, _) = list(&mut e, &[]);
        e.mem.set_u32(this + 0xc, 0x6000);
        e.call(0x00c8_be30, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x6000);
        assert!(calls_to(&log, INFO_DESTRUCT).is_empty());
        e.call(0x00c8_be30, &args![this, 0x0101u32]);
        assert_eq!(calls_to(&log, INFO_DESTRUCT), [[0x6000]]);
        assert_eq!(calls_to(&log, POOL_FREE), [[POOL_OBJECT, 0x6000]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // Nothing to free: only +0xC is cleared again.
        e.call(0x00c8_be30, &args![this, 1u32]);
        assert_eq!(calls_to(&log, INFO_DESTRUCT).len(), 1);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
