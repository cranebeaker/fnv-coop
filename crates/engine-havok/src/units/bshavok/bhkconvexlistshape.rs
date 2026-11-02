//! `BSHavok/bhkconvexlistshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkConvexListShape` (0x14 bytes, like `bhkShape`) keeps its Havok list shape at `+8`
//! (`phkObject`), its construction info at `+0xC` (`pInfo`) and its material at `+0x10`. The
//! Havok list shape holds its child shapes through an embedded object at `+0x14` (its virtual
//! slot `+4` answers the child count) and a child array at `+0x44`; each child is a `bhk`
//! wrapper whose Havok object is at `+8`. The construction info this class makes
//! (`CreateCinfo`) is a 0x1C-byte block: radius at `+4`, a flag byte at `+8`, a float at
//! `+0xC` and an `hkArray` of child pointers at `+0x10` (data), `+0x14` (size), `+0x18`
//! (capacity in the low 30 bits).
//!
//! The functions that allocate or release Havok objects raise the per-thread allocation
//! word at `TLS + 0x2b4` to 0x10 for their duration. The exception-unwinding frames
//! (`FS:[0]`) and `/GS` cookie checks of the functions marked `Translation incomplete` are
//! not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bshavok::bhkconstraint::{
    add_viewer_string, allocate_havok, enter_scope, leave_scope, stream_transfer,
};

/// `hkMemoryRouter` lookup (`00c85750`): its `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The pooled allocator object at `011f6238` and its allocate `(size)` (`00aa3e40`) and
/// release `(block)` (`00aa4060`) methods.
const POOL: u32 = 0x011f_6238;
const POOL_ALLOCATE: u32 = 0x00aa_3e40;
const POOL_RELEASE: u32 = 0x00aa_4060;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`)
/// and the grow-by-one helper (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW_ONE: u32 = 0x00c9_0940;
/// `bhkStream::SwapHKArray` (`00cf0930`, cdecl `(cinfo, size, array)`).
const SWAP_HK_ARRAY: u32 = 0x00cf_0930;
/// The base constructor the object constructor starts with (`00c87c30`).
const BASE_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable` methods: `Kill`, the clone lock and unlock.
const SERIALIZABLE_KILL: u32 = 0x00c8_76a0;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkConvexShape` methods: destructor body, `CopyCinfo`, `LoadBinary`, `SaveBinary`,
/// `GetViewerStrings`.
const CONVEX_SHAPE_DESTRUCT: u32 = 0x00c9_fb00;
const CONVEX_SHAPE_COPY_CINFO: u32 = 0x00c9_fb70;
const CONVEX_SHAPE_LOAD_BINARY: u32 = 0x00c9_fbc0;
const CONVEX_SHAPE_SAVE_BINARY: u32 = 0x00c9_fc00;
const CONVEX_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c9_fc50;
/// `bhkShape` methods: `CopyMembers` `(source, process)`, `CanShare`, `Link`, `LinkObject`.
const SHAPE_COPY_MEMBERS: u32 = 0x00c8_4df0;
const SHAPE_CAN_SHARE: u32 = 0x00c8_4ec0;
const SHAPE_LINK: u32 = 0x00c8_4e50;
const SHAPE_LINK_OBJECT: u32 = 0x00c8_4cc0;
/// `NiObject::RegisterStreamables` reached through `00aa0ad0`, a jump to `00a81fe0`.
const REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
/// `NiStream` helpers: `00a63430` answers the next linked object, `00a63460` the number of
/// links, `00a64670` reads the link ids.
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_GET_LINK_COUNT: u32 = 0x00a6_3460;
const STREAM_READ_LINK_IDS: u32 = 0x00a6_4670;
/// The Havok list shape's constructor (`00d5e750`, `(array, count)`), and its flag getter
/// (`00d5e0c0`, a byte at `+0x40`).
const HK_LIST_CONSTRUCT: u32 = 0x00d5_e750;
const HK_LIST_FLAG: u32 = 0x00d5_e0c0;
/// `hkReferencedObject::removeReference` (`00c905b0`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Imports: `InterlockedIncrement` and `InterlockedDecrement` (stdcall, one argument).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` overloads (cdecl) and `sprintf_s` (`00406d00`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_INT: u32 = 0x00a7_d4d0;
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// The vtables of the classes the constructor passes through (RTTI-confirmed).
const VTABLE_SHAPE: u32 = 0x0103_0bf4;
const VTABLE_SPHERE_REP_SHAPE: u32 = 0x0103_0b0c;
const VTABLE_CONVEX_SHAPE: u32 = 0x0103_0a24;
const VTABLE_CONVEX_LIST_SHAPE: u32 = 0x010c_abdc;
/// The live-object counters of those classes, in the same order.
const COUNT_SHAPE: u32 = 0x0126_80f8;
const COUNT_SPHERE_REP_SHAPE: u32 = 0x0126_8274;
const COUNT_CONVEX_SHAPE: u32 = 0x0126_8250;
const COUNT_CONVEX_LIST_SHAPE: u32 = 0x0126_86c0;
/// The global holding the text `GetViewerStrings` starts with (read as a word).
const CLASS_TEXT: u32 = 0x0126_86c4;
/// Bits of an `hkArray`'s capacity word: the capacity, and the flag that the storage is not
/// owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

layout! {
    /// `bhkConvexListShape` (Xbox PDB layout of `bhkShape`), 0x14 bytes.
    pub struct BhkConvexListShape: 0x14 {
        /// `phkObject` (Xbox PDB): the Havok list shape.
        0x08 phkObject: Ptr,
        /// `pInfo` (Xbox PDB): the construction info.
        0x0C pInfo: Ptr,
        /// `eMaterial` (Xbox PDB).
        0x10 eMaterial: u32,
    }
}

fn increment_count(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// The Havok list shape of `this`, or 0 when `this` or the shape is null.
fn list_object(e: &Engine, this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        e.mem.u32(this.addr() + 8)
    }
}

/// The child count: the embedded object at the Havok list shape's `+0x14`, virtual slot 1
/// (`+4`); 0 without a list shape.
fn child_count(e: &mut Engine, this: Ptr) -> u32 {
    let list = list_object(e, this);
    if list == 0 {
        0
    } else {
        e.vcall(list + 0x14, 4, &args![]).u32()
    }
}

/// The Havok object of child `index` (the child array is at the list shape's `+0x44`, each
/// entry a wrapper whose `+8` is its Havok object), or 0 when any link is missing.
fn child_object(e: &Engine, this: Ptr, index: u32) -> u32 {
    let list = list_object(e, this);
    if list == 0 {
        return 0;
    }
    let children = e.mem.u32(list + 0x44);
    let wrapper = e.mem.u32(children.wrapping_add(index.wrapping_mul(4)));
    if wrapper == 0 {
        return 0;
    }
    e.mem.u32(wrapper + 8)
}

/// The hkArray allocator: the memory router's allocator (`router + 0x10`).
fn array_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Frees an `hkArray`'s storage the way the code does: when the capacity word's top bit is
/// clear, the allocator's virtual slot `0x10` takes `(data, capacity * 4)`.
fn free_array_storage(e: &mut Engine, allocator: u32, data: u32, capacity_word: u32) {
    if (capacity_word as i32) >= 0 {
        let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Reserves room in the cinfo's `hkArray` for `wanted` elements of 4 bytes when its capacity
/// is smaller: the larger of twice the capacity and `wanted` (signed compares).
fn reserve_array(e: &mut Engine, allocator: u32, array: u32, wanted: u32) {
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let grow_to = if (wanted as i32) < doubled {
            doubled
        } else {
            wanted as i32
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, grow_to, 4u32]);
    }
}

// Translated from 00cc7d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkListShape::QCinfoSize` (engine map name, taken from a vtable shared by identical
/// code): takes no arguments and answers 0x1C, the size of the construction info this class
/// builds.
pub fn fn_00cc7d80(_e: &mut Engine) -> u32 {
    0x1c
}

// Translated from 00ccf740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf740 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::~bhkConvexListShape` (Xbox PDB): sets the class vtable, runs
/// `bhkSerializable::Kill`, decrements the live-object count at `012686c0` and runs the
/// `bhkConvexShape` destructor body (`00c9fb00`).
pub fn bhk_convex_list_shape_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CONVEX_LIST_SHAPE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.global::<u32>(COUNT_CONVEX_LIST_SHAPE);
    e.set_global(COUNT_CONVEX_LIST_SHAPE, count.wrapping_sub(1));
    e.call(CONVEX_SHAPE_DESTRUCT, &args![this]);
}

// Translated from 00ccf7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape` constructor (unnamed in the engine map): the base constructor
/// (`00c87c30`), `+0xC` and `+0x10` cleared, then the vtable of each class on the way down
/// (`bhkShape`, `bhkSphereRepShape`, `bhkConvexShape`, `bhkConvexListShape`), each with its
/// live-object count raised. Answers `this`.
pub fn fn_00ccf7c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    for (vtable, counter) in [
        (VTABLE_SHAPE, COUNT_SHAPE),
        (VTABLE_SPHERE_REP_SHAPE, COUNT_SPHERE_REP_SHAPE),
        (VTABLE_CONVEX_SHAPE, COUNT_CONVEX_SHAPE),
        (VTABLE_CONVEX_LIST_SHAPE, COUNT_CONVEX_LIST_SHAPE),
    ] {
        e.mem.set_u32(this.addr(), vtable);
        increment_count(e, counter);
    }
    this
}

// Translated from 00ccf810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::GetSupportingVertex` (Xbox PDB): with a Havok list shape, a tail
/// jump to its virtual slot `0x38` with the two stack words (its result is returned);
/// otherwise nothing.
pub fn bhk_convex_list_shape_get_supporting_vertex(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) -> Ret {
    let list = list_object(e, this);
    if list == 0 {
        return Ret::default();
    }
    e.vcall(list, 0x38, &args![first, second])
}

// Translated from 00ccf830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::ConvertVertexIdsToVertices` (Xbox PDB): a tail jump to the Havok
/// list shape's virtual slot `0x3C` with the three stack words, when it exists.
pub fn bhk_convex_list_shape_convert_vertex_ids_to_vertices(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
) -> Ret {
    let list = list_object(e, this);
    if list == 0 {
        return Ret::default();
    }
    e.vcall(list, 0x3c, &args![first, second, third])
}

// Translated from 00ccf850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::GetFirstVertex` (Xbox PDB): a tail jump to the Havok list shape's
/// virtual slot `0x34` with the stack word, when it exists.
pub fn bhk_convex_list_shape_get_first_vertex(e: &mut Engine, this: Ptr, first: u32) -> Ret {
    let list = list_object(e, this);
    if list == 0 {
        return Ret::default();
    }
    e.vcall(list, 0x34, &args![first])
}

// Translated from 00ccf890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccf890 - the FS:[0] exception-unwinding frame is not modeled.
/// Allocates a 0x14-byte block (`operator new`, `00aa13e0`) and constructs a
/// `bhkConvexListShape` in it (`00ccf7c0`); 0 when the allocation fails.
pub fn fn_00ccf890(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00ccf7c0(e, Ptr::new(block))
}

// Translated from 00ccf900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::CopyMembers` (Xbox PDB), `(this, source, process)`: for each child
/// pointer in the cinfo's array (virtual slot `0xB4` answers the cinfo; size `+0x14`, data
/// `+0x10`) whose object's virtual slot `0xD0` (with `process`) answers false, replaces it
/// with the Havok object (`+8`) of the clone that object's slot `0x48` makes (0 when there
/// is no clone); then `bhkShape::CopyMembers`.
pub fn bhk_convex_list_shape_copy_members(e: &mut Engine, this: Ptr, source: u32, process: u32) {
    let cinfo = e
        .with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32();
    let count = e.mem.u32(cinfo + 0x14) as i32;
    let mut index = 0;
    while index < count {
        let data = e.mem.u32(cinfo + 0x10);
        let child = e.mem.u32(data + 4 * index as u32);
        if child != 0 {
            let object = e.mem.u32(child + 8);
            if object != 0 && !e.vcall(object, 0xd0, &args![process]).bool() {
                let clone = e.vcall(object, 0x48, &args![process]).u32();
                let replacement = if clone != 0 { e.mem.u32(clone + 8) } else { 0 };
                let data = e.mem.u32(cinfo + 0x10);
                e.mem.set_u32(data + 4 * index as u32, replacement);
            }
        }
        index += 1;
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, source, process]);
}

// Translated from 00ccf990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::RegisterStreamables` (Xbox PDB): registers each child's Havok
/// object through its virtual slot `0x54` with the stream (children counted unsigned), then
/// `NiObject::RegisterStreamables` (through `00aa0ad0`), whose `AL` it answers: the exe ends on
/// that call, and `RegisterStreamables` is `bool` in the Xbox PDB.
pub fn bhk_convex_list_shape_register_streamables(e: &mut Engine, this: Ptr, stream: u32) -> bool {
    let count = child_count(e, this);
    let mut index = 0;
    while index < count {
        let object = child_object(e, this, index);
        if object != 0 {
            e.vcall(object, 0x54, &args![stream]);
        }
        index += 1;
    }
    e.call(REGISTER_STREAMABLES, &args![this, stream]).bool()
}

// Translated from 00ccfa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Raises the reference count (`+4`, `InterlockedIncrement`) of the Havok object of every
/// child, last to first (unnamed in the engine map).
pub fn fn_00ccfa00(e: &mut Engine, this: Ptr) {
    if list_object(e, this) == 0 {
        return;
    }
    let mut remaining = child_count(e, this);
    while remaining != 0 {
        remaining = remaining.wrapping_sub(1);
        let object = child_object(e, this, remaining);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
        }
    }
}

// Translated from 00ccfa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::CanShare` (Xbox PDB): `bhkShape::CanShare`, and while that holds,
/// every child's Havok object's virtual slot `0xD0` with `other`, the answers ANDed in a
/// byte (children counted signed).
pub fn bhk_convex_list_shape_can_share(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let mut result = e.call(SHAPE_CAN_SHARE, &args![this, other]).u8();
    if result == 0 {
        return false;
    }
    let count = child_count(e, this) as i32;
    let mut index = 0;
    while index < count {
        let object = child_object(e, this, index as u32);
        if object != 0 {
            result &= e.vcall(object, 0xd0, &args![other]).u8();
        }
        index += 1;
        if result == 0 {
            break;
        }
    }
    result != 0
}

// Translated from 00ccfad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::BuildDisplayGeometry` (Xbox PDB): calls each child's Havok object's
/// virtual slot `0xD4` with the two stack words (children counted signed).
pub fn bhk_convex_list_shape_build_display_geometry(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) {
    let count = child_count(e, this) as i32;
    let mut index = 0;
    while index < count {
        let object = child_object(e, this, index as u32);
        if object != 0 {
            e.vcall(object, 0xd4, &args![first, second]);
        }
        index += 1;
    }
}

// Translated from 00ccfb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::QSubMaterial` (Xbox PDB): for an index other than -1 that is not
/// above the child count and has a child object, that object's material (`+0x10`) in the
/// low five bits; otherwise this shape's own material (`+0x10`) in the low five bits.
pub fn bhk_convex_list_shape_q_sub_material(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index != 0xffff_ffff {
        let count = child_count(e, this);
        if index <= count {
            let object = child_object(e, this, index);
            if object != 0 {
                return e.mem.u32(object + 0x10) & 0x1f;
            }
        }
    }
    e.mem.u32(this.addr() + 0x10) & 0x1f
}

// Translated from 00ccfba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccfba0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::CreateClone` (Xbox PDB): in the Havok allocation scope and between
/// the clone lock and unlock (`00c876f0`, `00c87700`), makes a new 0x14-byte shape
/// (`operator new` and the constructor `00ccf7c0`; null stays null) and calls this shape's
/// virtual slot `0xC4` with `(clone, process)`. Answers the clone.
pub fn bhk_convex_list_shape_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let old = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let clone = if block != 0 {
        fn_00ccf7c0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    e.vcall(this.addr(), 0xc4, &args![clone, process]);
    e.call(CLONE_UNLOCK, &args![]);
    leave_scope(e, old);
    clone
}

// Translated from 00ccfc60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccfc60 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::Init` (Xbox PDB): in the Havok allocation scope, builds a 0x50-byte
/// Havok list shape from the cinfo's child array (`+0x10`) and size (`+0x14`) (`00d5e750`),
/// hands it to virtual slot `0x8C`, releases it, then calls virtual slot `0xBC` with the cinfo.
pub fn bhk_convex_list_shape_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    let size = e.mem.u32(cinfo + 0x14);
    let array = e.mem.u32(cinfo + 0x10);
    let block = allocate_havok(e, 0x50);
    let shape = e.call(HK_LIST_CONSTRUCT, &args![block, array, size]).u32();
    e.vcall(this.addr(), 0x8c, &args![shape]);
    e.call(REMOVE_REFERENCE, &args![shape]);
    e.vcall(this.addr(), 0xbc, &args![cinfo]);
    leave_scope(e, old);
}

// Translated from 00ccfd40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccfd40 - the FS:[0] exception-unwinding frame is not modeled.
/// In the Havok allocation scope, lowers the reference count (`+4`,
/// `InterlockedDecrement`) of the Havok object of every child, last to first, and runs that
/// object's virtual slot 1 (`+4`) when it reaches zero (unnamed in the engine map).
pub fn fn_00ccfd40(e: &mut Engine, this: Ptr) {
    let old = enter_scope(e);
    if list_object(e, this) != 0 {
        let mut remaining = child_count(e, this);
        while remaining != 0 {
            remaining = remaining.wrapping_sub(1);
            let object = child_object(e, this, remaining);
            if object != 0 {
                let left = e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32();
                if left == 0 {
                    e.vcall(object, 4, &args![]);
                }
            }
        }
    }
    leave_scope(e, old);
}

// Translated from 00ccfe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::Link` (Xbox PDB): when the low byte of `flag` is set references
/// every child (`00ccfa00`), otherwise releases them (`00ccfd40`); then `bhkShape::Link` with
/// the same word.
pub fn bhk_convex_list_shape_link(e: &mut Engine, this: Ptr, flag: u32) {
    if flag as u8 != 0 {
        fn_00ccfa00(e, this);
    } else {
        fn_00ccfd40(e, this);
    }
    e.call(SHAPE_LINK, &args![this, flag]);
}

// Translated from 00ccfe50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccfe50 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::CopyCinfo` (Xbox PDB): in the Havok allocation scope, `bhkConvexShape::
/// CopyCinfo`, then fills the cinfo: the float at the Havok list shape's `+0x18` into `+0xC`
/// (a float copy through the x87 stack, 0 without a list shape), the byte from `00d5e0c0`
/// into `+8` (0 without one), and the child array (`+0x10`, size `+0x14`) rebuilt: grown if
/// its capacity is smaller than the child count (the larger of twice the capacity and the
/// count, through `hkArrayUtil::_reserve`), then each child pointer copied from the list
/// shape's array while the embedded object's slots 2 and 3 (`+8`, `+0xC`) step an iteration
/// token.
pub fn bhk_convex_list_shape_copy_cinfo(e: &mut Engine, this: Ptr, cinfo: u32) {
    let old = enter_scope(e);
    e.call(CONVEX_SHAPE_COPY_CINFO, &args![this, cinfo]);
    let list = list_object(e, this);
    let radius = if list == 0 {
        0
    } else {
        x87_quiet_bits(e.mem.u32(list + 0x18))
    };
    e.mem.set_u32(cinfo + 0xc, radius);
    let flag = if list == 0 {
        0
    } else {
        e.call(HK_LIST_FLAG, &args![list]).u8()
    };
    e.mem.set_u8(cinfo + 8, flag);
    let count = child_count(e, this);
    e.mem.set_u32(cinfo + 0x14, 0);
    let allocator = array_allocator(e);
    reserve_array(e, allocator, cinfo + 0x10, count);
    e.mem.set_u32(cinfo + 0x14, count);
    let list = list_object(e, this);
    let mut token = if list == 0 {
        0
    } else {
        e.vcall(list + 0x14, 8, &args![]).u32()
    };
    let mut index = 0;
    while index < count {
        let list = list_object(e, this);
        let child = if list == 0 {
            0
        } else {
            let children = e.mem.u32(list + 0x44);
            e.mem.u32(children.wrapping_add(4 * index))
        };
        let data = e.mem.u32(cinfo + 0x10);
        e.mem.set_u32(data.wrapping_add(4 * index), child);
        let list = list_object(e, this);
        token = if list == 0 {
            0
        } else {
            e.vcall(list + 0x14, 0xc, &args![token]).u32()
        };
        index += 1;
    }
    leave_scope(e, old);
}

// Translated from 00ccffd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ccffd0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::CreateCinfo` (Xbox PDB), `(this, created)`: in the Havok allocation
/// scope stores into the byte at `created` whether the shape had no cinfo (`+0xC` null); in
/// that case allocates a 0x1C-byte block from the pool at `011f6238` (null stays null),
/// initialises it (`+0` and `+0x10`, `+0x14` zero, `+4` the float at `010c72bc` through the
/// x87 stack, `+0x18` the not-owned empty capacity), stores it at `+0xC` and, when the shape
/// has a Havok list shape, fills it with `CopyCinfo`. Answers the cinfo at `+0xC`.
pub fn bhk_convex_list_shape_create_cinfo(e: &mut Engine, this: Ptr, created: Ptr) -> u32 {
    let old = enter_scope(e);
    let missing = e.mem.u32(this.addr() + 0xc) == 0;
    e.mem.set_u8(created.addr(), missing as u8);
    if missing {
        let cinfo = e.call(POOL_ALLOCATE, &args![POOL, 0x1cu32]).u32();
        if cinfo != 0 {
            e.mem.set_u32(cinfo, 0);
            let radius = x87_quiet_bits(e.mem.u32(0x010c_72bc));
            e.mem.set_u32(cinfo + 4, radius);
            e.mem.set_u32(cinfo + 0x10, 0);
            e.mem.set_u32(cinfo + 0x14, 0);
            e.mem.set_u32(cinfo + 0x18, NOT_OWNED);
        }
        let has_list = e.mem.u32(this.addr() + 8) != 0;
        e.mem.set_u32(this.addr() + 0xc, cinfo);
        if has_list {
            bhk_convex_list_shape_copy_cinfo(e, this, cinfo);
        }
    }
    let cinfo = e.mem.u32(this.addr() + 0xc);
    leave_scope(e, old);
    cinfo
}

// Translated from 00cd00a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::LoadBinary` (Xbox PDB): reads the link ids (`00a64670`), loads the
/// `bhkConvexShape` part, takes the cinfo (virtual slot `0xB4`), swaps its array
/// (`bhkStream::SwapHKArray` with the size from slot `0xA8`) when the stream's byte at
/// `+0x1E5` is not 1, reads the cinfo's byte at `+8` and dword at `+0xC` through the reader
/// (`+0x24C`), and swaps the array again when that byte is not 1.
pub fn bhk_convex_list_shape_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(STREAM_READ_LINK_IDS, &args![stream]);
    e.call(CONVEX_SHAPE_LOAD_BINARY, &args![this, stream]);
    let cinfo = e
        .with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32();
    if e.mem.u8(stream + 0x1e5) != 1 {
        let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 0x10]);
    }
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, cinfo + 8, 1);
    let reader = e.mem.u32(stream + 0x24c);
    stream_transfer(e, reader, 8, cinfo + 0xc, 4);
    if e.mem.u8(stream + 0x1e5) != 1 {
        let size = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![cinfo, size, cinfo + 0x10]);
    }
}

// Translated from 00cd0170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd0170 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::LinkObject` (Xbox PDB): in the Havok allocation scope, takes the
/// cinfo (slot `0xB4`) and the link count of the stream (`00a63460`), frees the cinfo's child
/// array and resets it, reserves room for the count, then for each link takes the linked
/// object from the stream and, when there is one, appends its Havok object (`+8`) to the
/// array (growing by one when full); then `bhkShape::LinkObject`.
pub fn bhk_convex_list_shape_link_object(e: &mut Engine, this: Ptr, stream: u32) {
    let old = enter_scope(e);
    let cinfo = e
        .with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32();
    let count = e.call(STREAM_GET_LINK_COUNT, &args![stream]).u32();
    let array = cinfo + 0x10;
    let allocator = array_allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    let data = e.mem.u32(array);
    free_array_storage(e, allocator, data, capacity_word);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    let allocator = array_allocator(e);
    reserve_array(e, allocator, array, count);
    let mut remaining = count;
    while remaining != 0 {
        let linked = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if linked != 0 {
            let object = e.mem.u32(linked + 8);
            let allocator = array_allocator(e);
            let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
            if e.mem.u32(array + 4) == capacity {
                e.call(ARRAY_GROW_ONE, &args![allocator, array, 4u32]);
            }
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            e.mem.set_u32(data + 4 * size, object);
            e.mem.set_u32(array + 4, size.wrapping_add(1));
        }
        remaining -= 1;
    }
    e.call(SHAPE_LINK_OBJECT, &args![this, stream]);
    leave_scope(e, old);
}

// Translated from 00cd02d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd02d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::SaveBinary` (Xbox PDB): takes the cinfo (slot `0xB4`) and moves its
/// child array into locals, leaving the cinfo's array empty (not owned). When the stream's byte
/// at `+0x1E4` is not 1 it swaps the (now empty) array. Through the writer (`+0x250`) it
/// writes the array's size (4 bytes), then, for each of the Havok list shape's children
/// (counted again each time, signed), the stream's virtual slot `0x30` with the saved
/// pointer's `+8` (0 for a null pointer, advancing the index either way); then
/// `bhkConvexShape::SaveBinary`, the cinfo's byte at `+8` (1 byte) and dword at `+0xC` (4
/// bytes), and finally frees the saved array storage.
pub fn bhk_convex_list_shape_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let cinfo = e
        .with_stack(4, |e, flag| e.vcall(this.addr(), 0xb4, &args![flag]))
        .u32();
    let size = e.mem.u32(cinfo + 0x14);
    let array = cinfo + 0x10;
    let data = e.mem.u32(array);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    if e.mem.u8(stream + 0x1e4) != 1 {
        let swap_size = e.vcall(this.addr(), 0xa8, &args![]).u32();
        e.call(SWAP_HK_ARRAY, &args![cinfo, swap_size, array]);
    }
    let writer = e.mem.u32(stream + 0x250);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), size);
        stream_transfer(e, writer, 0xc, local.addr(), 4);
    });
    let mut index = 0u32;
    loop {
        let hk = e.mem.u32(this.addr() + 8);
        let total = if hk == 0 {
            0
        } else {
            e.vcall(hk + 0x14, 4, &args![]).u32()
        };
        if (index as i32) >= (total as i32) {
            break;
        }
        let item = e.mem.u32(data.wrapping_add(4 * index));
        let link = if item != 0 { e.mem.u32(item + 8) } else { 0 };
        e.vcall(stream, 0x30, &args![link]);
        index += 1;
    }
    e.call(CONVEX_SHAPE_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, cinfo + 8, 1);
    let writer = e.mem.u32(stream + 0x250);
    stream_transfer(e, writer, 0xc, cinfo + 0xc, 4);
    let allocator = array_allocator(e);
    free_array_storage(e, allocator, data, capacity_word);
}

// Translated from 00cd0480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd0480 - the FS:[0] exception-unwinding frame and the /GS cookie check are not modeled.
/// `bhkConvexListShape::GetViewerStrings` (Xbox PDB): in the Havok allocation scope,
/// `bhkConvexShape::GetViewerStrings`, then appends the viewer string of the text in the
/// global at `012686c4` and the one for the child count under the name at `010c41b0`; then,
/// once per child (signed count), formats the index into a 0x20-byte buffer with the format
/// at `010c6040` (the formatted text is not used).
pub fn bhk_convex_list_shape_get_viewer_strings(e: &mut Engine, this: Ptr, list: u32) {
    let old = enter_scope(e);
    e.call(CONVEX_SHAPE_GET_VIEWER_STRINGS, &args![this, list]);
    let class_text = e.global::<u32>(CLASS_TEXT);
    let text = e.call(VIEWER_STRING, &args![class_text]).u32();
    add_viewer_string(e, list, text);
    let count = child_count(e, this);
    let text = e
        .call(VIEWER_STRING_INT, &args![0x010c_41b0u32, count])
        .u32();
    add_viewer_string(e, list, text);
    let mut index = 0;
    while index < count as i32 {
        e.with_stack(0x20, |e, buffer| {
            e.call(FORMAT_TEXT, &args![buffer, 0x20u32, 0x010c_6040u32, index]);
        });
        index += 1;
    }
    leave_scope(e, old);
}

// Translated from 00cd05e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cd05e0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkConvexListShape::KillCinfo` (Xbox PDB): when the low byte of `kill` is set, in the
/// Havok allocation scope frees the cinfo's child array storage, empties it, releases the
/// cinfo block to the pool (`00aa4060`) and clears `+0xC`.
pub fn bhk_convex_list_shape_kill_cinfo(e: &mut Engine, this: Ptr, kill: u32) {
    let old = enter_scope(e);
    if kill as u8 != 0 {
        let cinfo = e.mem.u32(this.addr() + 0xc);
        if cinfo != 0 {
            let allocator = array_allocator(e);
            let capacity_word = e.mem.u32(cinfo + 0x18);
            e.mem.set_u32(cinfo + 0x14, 0);
            let data = e.mem.u32(cinfo + 0x10);
            free_array_storage(e, allocator, data, capacity_word);
            e.mem.set_u32(cinfo + 0x10, 0);
            e.mem.set_u32(cinfo + 0x18, NOT_OWNED);
            e.call(POOL_RELEASE, &args![POOL, cinfo]);
        }
        e.mem.set_u32(this.addr() + 0xc, 0);
    }
    leave_scope(e, old);
}

// Translated from 00ccf870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ccf740`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_convex_list_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_f740;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00ccf7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkConvexListShape::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_86c4.
pub fn bhk_convex_list_shape_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_86c4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc7d80, fn_00cc7d80() -> u32),
        entry!(0x00ccf740, bhk_convex_list_shape_destructor(Ptr)),
        entry!(0x00ccf7b0, bhk_convex_list_shape_get_r_t_t_i() -> u32),
        entry!(0x00ccf7c0, fn_00ccf7c0(Ptr) -> Ptr),
        (
            0x00ccf810,
            (|e: &mut Engine, a: &[u32]| {
                bhk_convex_list_shape_get_supporting_vertex(e, Ptr::new(a[0]), a[1], a[2])
            }) as AbiFn,
        ),
        (
            0x00ccf830,
            (|e: &mut Engine, a: &[u32]| {
                bhk_convex_list_shape_convert_vertex_ids_to_vertices(
                    e,
                    Ptr::new(a[0]),
                    a[1],
                    a[2],
                    a[3],
                )
            }) as AbiFn,
        ),
        (
            0x00ccf850,
            (|e: &mut Engine, a: &[u32]| {
                bhk_convex_list_shape_get_first_vertex(e, Ptr::new(a[0]), a[1])
            }) as AbiFn,
        ),
        entry!(0x00ccf870, bhk_convex_list_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ccf890, fn_00ccf890() -> Ptr),
        entry!(
            0x00ccf900,
            bhk_convex_list_shape_copy_members(Ptr, u32, u32)
        ),
        entry!(
            0x00ccf990,
            bhk_convex_list_shape_register_streamables(Ptr, u32) -> bool
        ),
        entry!(0x00ccfa00, fn_00ccfa00(Ptr)),
        entry!(0x00ccfa50, bhk_convex_list_shape_can_share(Ptr, u32) -> bool),
        entry!(
            0x00ccfad0,
            bhk_convex_list_shape_build_display_geometry(Ptr, u32, u32)
        ),
        entry!(0x00ccfb40, bhk_convex_list_shape_q_sub_material(Ptr, u32) -> u32),
        entry!(0x00ccfba0, bhk_convex_list_shape_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00ccfc60, bhk_convex_list_shape_init(Ptr, u32)),
        entry!(0x00ccfd40, fn_00ccfd40(Ptr)),
        entry!(0x00ccfe20, bhk_convex_list_shape_link(Ptr, u32)),
        entry!(0x00ccfe50, bhk_convex_list_shape_copy_cinfo(Ptr, u32)),
        entry!(0x00ccffd0, bhk_convex_list_shape_create_cinfo(Ptr, Ptr) -> u32),
        entry!(0x00cd00a0, bhk_convex_list_shape_load_binary(Ptr, u32)),
        entry!(0x00cd0170, bhk_convex_list_shape_link_object(Ptr, u32)),
        entry!(0x00cd02d0, bhk_convex_list_shape_save_binary(Ptr, u32)),
        entry!(
            0x00cd0480,
            bhk_convex_list_shape_get_viewer_strings(Ptr, u32)
        ),
        entry!(0x00cd05e0, bhk_convex_list_shape_kill_cinfo(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_convex_list_shape_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_f740, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ccf870, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ccf870, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x14]]);
        assert_eq!(calls_to(&e, 0x00cc_f740), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_convex_list_shape_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_f7b0, &args![]).u32(), 0x0126_86c4);
    }
    type Calls = Vec<Vec<u32>>;

    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0u32; 0x80];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        let vtable = e.mem.alloc(0x200);
        for (i, word) in table.iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *word);
        }
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    fn scope(e: &mut Engine) -> u32 {
        let word = e.tls() + 0x2b4;
        e.mem.u32(word)
    }

    /// Starts the scope word at a recognisable value, so its restore is visible.
    fn mark_scope(e: &mut Engine) {
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x77);
    }

    /// The memory router: its allocator (slot 4 allocates, slot `0x10` records a free).
    fn install_allocator(e: &mut Engine) -> u32 {
        let allocator = object(e, &[(4, 0x0900_0200), (0x10, 0x0900_0201)]);
        e.register(0x0900_0200, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        quiet(e, &[0x0900_0201]);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router + 0x10, allocator);
        e.map(0x0200_3000, 0x100);
        e.mem.set_u32(0x0200_3000, router);
        e.register(MEMORY_ROUTER, |e, _| Ret {
            eax: e.mem.u32(0x0200_3000),
            ..Ret::default()
        });
        allocator
    }

    /// A convex list shape with `count` children. Returns `(this, list, hk children)`.
    /// The list's embedded object (at `list + 0x14`) answers `count` (kept at
    /// `list + 0x34`) from slot 4, and steps a token in slots 8 and `0xC`; each child has
    /// slots `0xD0`, `0xD4`, `0x54`, `0x48` and `0x04` set to recording doubles.
    fn shape(e: &mut Engine, count: u32) -> (u32, u32, Vec<u32>) {
        let this = object(
            e,
            &[
                (0xb4, 0x0900_0100),
                (0xa8, 0x0900_0101),
                (0xc4, 0x0900_0102),
                (0x8c, 0x0900_0103),
                (0xbc, 0x0900_0104),
            ],
        );
        let list = e.mem.alloc(0x80);
        let embedded = object(e, &[(4, 0x0900_0110), (8, 0x0900_0111), (0xc, 0x0900_0112)]);
        let embedded_table = e.mem.u32(embedded);
        e.mem.set_u32(list + 0x14, embedded_table);
        e.mem.set_u32(list + 0x34, count);
        e.mem.set_u32(this + 8, list);
        let mut children = vec![];
        let wrappers = e.mem.alloc(4 * count.max(1));
        for i in 0..count {
            let child = object(
                e,
                &[
                    (4, 0x0900_0120),
                    (0x48, 0x0900_0121),
                    (0x54, 0x0900_0122),
                    (0xd0, 0x0900_0123),
                    (0xd4, 0x0900_0124),
                ],
            );
            e.mem.set_u32(child + 0x10, 0x100 + i);
            let wrapper = e.mem.alloc(0x20);
            e.mem.set_u32(wrapper + 8, child);
            e.mem.set_u32(wrappers + 4 * i, wrapper);
            children.push(child);
        }
        e.mem.set_u32(list + 0x44, wrappers);
        // The slot-4 count of the embedded object; slot 8/0xC step a token by one.
        e.register(0x0900_0110, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x20),
            ..Ret::default()
        });
        e.register(0x0900_0111, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.register(0x0900_0112, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        quiet(e, &[0x0900_0120, 0x0900_0122, 0x0900_0124]);
        e.register(0x0900_0121, |e, a| Ret {
            // A child's clone is a wrapper whose +8 is the child's own Havok twin.
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        e.register(0x0900_0123, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        (this, list, children)
    }

    #[test]
    fn fn_00cc7d80_answers_0x1c() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc7d80, &args![]).u32(), 0x1c);
    }

    #[test]
    fn destructor_sets_the_vtable_kills_counts_down_and_destructs() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(COUNT_CONVEX_LIST_SHAPE, 9u32);
        quiet(&mut e, &[SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.call(0x00ccf740, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_LIST_SHAPE);
        assert_eq!(e.global::<u32>(COUNT_CONVEX_LIST_SHAPE), 8);
        assert_eq!(
            order(&e),
            vec![0x00ccf740, SERIALIZABLE_KILL, CONVEX_SHAPE_DESTRUCT]
        );
    }

    #[test]
    fn constructor_counts_every_class_on_the_way_down() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x10, 6);
        assert_eq!(e.call(0x00ccf7c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CONVEX_LIST_SHAPE);
        assert_eq!((e.mem.u32(this + 0xc), e.mem.u32(this + 0x10)), (0, 0));
        for counter in [
            COUNT_SHAPE,
            COUNT_SPHERE_REP_SHAPE,
            COUNT_CONVEX_SHAPE,
            COUNT_CONVEX_LIST_SHAPE,
        ] {
            assert_eq!(e.global::<u32>(counter), 1);
        }
    }

    #[test]
    fn list_shape_tail_calls_forward_their_words_when_the_shape_exists() {
        let mut e = Engine::new();
        let (this, list, _) = shape(&mut e, 0);
        let owner = object(
            &mut e,
            &[
                (0x34, 0x0900_0130),
                (0x38, 0x0900_0131),
                (0x3c, 0x0900_0132),
            ],
        );
        let table = e.mem.u32(owner);
        e.mem.set_u32(list, table);
        for target in [0x0900_0130, 0x0900_0131, 0x0900_0132] {
            e.register(target, |_, a| Ret {
                eax: a.iter().sum(),
                ..Ret::default()
            });
        }
        assert_eq!(e.call(0x00ccf810, &args![this, 1u32, 2u32]).u32(), list + 3);
        assert_eq!(
            e.call(0x00ccf830, &args![this, 1u32, 2u32, 3u32]).u32(),
            list + 6
        );
        assert_eq!(e.call(0x00ccf850, &args![this, 7u32]).u32(), list + 7);
        // No Havok list shape (or no shape at all): nothing is called.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccf810, &args![this, 1u32, 2u32]);
        e.call(0x00ccf830, &args![0u32, 1u32, 2u32, 3u32]);
        e.call(0x00ccf850, &args![this, 7u32]);
        assert_eq!(order(&e), vec![0x00ccf810, 0x00ccf830, 0x00ccf850]);
    }

    #[test]
    fn allocate_and_construct_makes_a_0x14_byte_shape() {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        quiet(&mut e, &[BASE_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let made = e.call(0x00ccf890, &args![]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(e.mem.u32(made), VTABLE_CONVEX_LIST_SHAPE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ccf890, &args![]).u32(), 0);
    }

    #[test]
    fn copy_members_clones_the_children_that_are_not_shared() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 0);
        let _ = children;
        // The cinfo: three wrappers; slot 0xB4 answers it.
        let cinfo = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(cinfo + 0x10, array);
        e.mem.set_u32(cinfo + 0x14, 3);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x40, cinfo);
        // Child 0 is shared (slot 0xD0 true), child 1 is cloned, child 2 is null.
        let shared = object(&mut e, &[(0xd0, 0x0900_0140)]);
        let cloned = object(&mut e, &[(0xd0, 0x0900_0141), (0x48, 0x0900_0142)]);
        e.register(0x0900_0140, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x0900_0141, |_, _| Ret::default());
        e.register(0x0900_0142, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        let clone_wrapper = e.mem.alloc(0x20);
        e.mem.set_u32(clone_wrapper + 8, 0xc10e);
        e.mem.set_u32(cloned + 0x30, clone_wrapper);
        for (i, object) in [shared, cloned].into_iter().enumerate() {
            let wrapper = e.mem.alloc(0x20);
            e.mem.set_u32(wrapper + 8, object);
            e.mem.set_u32(array + 4 * i as u32, wrapper);
        }
        let original = e.mem.u32(array);
        quiet(&mut e, &[SHAPE_COPY_MEMBERS]);
        e.call_log = Some(vec![]);
        e.call(0x00ccf900, &args![this, 0x9999u32, 0x5555u32]);
        assert_eq!(e.mem.u32(array), original);
        assert_eq!(e.mem.u32(array + 4), 0xc10e);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert_eq!(calls_to(&e, 0x0900_0142), vec![vec![cloned, 0x5555]]);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, 0x9999, 0x5555]]
        );
    }

    #[test]
    fn register_streamables_registers_every_child_then_the_base() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 2);
        quiet(&mut e, &[REGISTER_STREAMABLES]);
        e.call_log = Some(vec![]);
        e.call(0x00ccf990, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, 0x0900_0122),
            vec![vec![children[0], 0x6000], vec![children[1], 0x6000]]
        );
        assert_eq!(calls_to(&e, REGISTER_STREAMABLES), vec![vec![this, 0x6000]]);
    }

    #[test]
    fn register_streamables_answers_the_base_al() {
        let mut e = Engine::new();
        let (this, _, _) = shape(&mut e, 2);
        e.register(REGISTER_STREAMABLES, |_, _| 0x0000_0101u32.into_ret());
        assert!(e.call(0x00ccf990, &args![this, 0x6000u32]).bool());
        e.register(REGISTER_STREAMABLES, |_, _| 0x0000_0100u32.into_ret());
        assert!(!e.call(0x00ccf990, &args![this, 0x6000u32]).bool());
    }

    #[test]
    fn children_are_referenced_last_to_first() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 2);
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ccfa00, &args![this]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![children[1] + 4], vec![children[0] + 4]]
        );
        // No list shape: nothing happens.
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ccfa00, &args![this]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn children_are_released_last_to_first_and_destroyed_at_zero() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let (this, _, children) = shape(&mut e, 2);
        // Only the first child's count reaches zero.
        let first = children[0] + 4;
        e.register_double(INTERLOCKED_DECREMENT, move |_, a| Ret {
            eax: (a[0] != first) as u32,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ccfd40, &args![this]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![children[1] + 4], vec![children[0] + 4]]
        );
        assert_eq!(calls_to(&e, 0x0900_0120), vec![vec![children[0]]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn can_share_is_the_base_and_every_child() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 3);
        e.register(SHAPE_CAN_SHARE, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        // Child answers come from +0x34 (1 by default for all three).
        for child in &children {
            e.mem.set_u32(child + 0x34, 1);
        }
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ccfa50, &args![this, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_0123).len(), 3);
        // The second child says no: the third is not asked.
        e.mem.set_u32(children[1] + 0x34, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ccfa50, &args![this, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0900_0123).len(), 2);
        // The base says no: no child is asked.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00ccfa50, &args![this, 0u32]).bool());
        assert!(calls_to(&e, 0x0900_0123).is_empty());
    }

    #[test]
    fn build_display_geometry_calls_each_child_with_both_words() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ccfad0, &args![this, 0x11u32, 0x22u32]);
        assert_eq!(
            calls_to(&e, 0x0900_0124),
            vec![vec![children[0], 0x11, 0x22], vec![children[1], 0x11, 0x22]]
        );
    }

    #[test]
    fn q_sub_material_prefers_the_child_and_masks_five_bits() {
        let mut e = Engine::new();
        let (this, _, children) = shape(&mut e, 2);
        e.mem.set_u32(this + 0x10, 0xff);
        e.mem.set_u32(children[1] + 0x10, 0x3f);
        let q = |e: &mut Engine, index: u32| e.call(0x00ccfb40, &args![this, index]).u32();
        assert_eq!(q(&mut e, 0xffff_ffff), 0x1f);
        assert_eq!(q(&mut e, 1), 0x1f);
        e.mem.set_u32(children[1] + 0x10, 0x03);
        assert_eq!(q(&mut e, 1), 0x03);
        // Index 2 equals the count: allowed, but the array entry is null, so the own value.
        assert_eq!(q(&mut e, 2), 0x1f);
        assert_eq!(q(&mut e, 3), 0x1f);
    }

    #[test]
    fn create_clone_brackets_the_new_shape_with_the_clone_lock() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        e.map(0x0126_8000, 0x1000);
        let (this, _, _) = shape(&mut e, 0);
        quiet(
            &mut e,
            &[CLONE_LOCK, CLONE_UNLOCK, BASE_CONSTRUCT, 0x0900_0102],
        );
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let clone = e.call(0x00ccfba0, &args![this, 0x4242u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE_CONVEX_LIST_SHAPE);
        assert_eq!(
            order(&e),
            vec![
                0x00ccfba0,
                CLONE_LOCK,
                OPERATOR_NEW,
                BASE_CONSTRUCT,
                0x0900_0102,
                CLONE_UNLOCK
            ]
        );
        assert_eq!(calls_to(&e, 0x0900_0102), vec![vec![this, clone, 0x4242]]);
        assert_eq!(scope(&mut e), 0x77);
        // A failed allocation clones into null.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccfba0, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, 0x0900_0102), vec![vec![this, 0, 1]]);
    }

    #[test]
    fn init_builds_the_havok_list_from_the_cinfo() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let (this, _, _) = shape(&mut e, 0);
        e.register(HK_LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        quiet(&mut e, &[REMOVE_REFERENCE, 0x0900_0103, 0x0900_0104]);
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(cinfo + 0x10, 0xa11a);
        e.mem.set_u32(cinfo + 0x14, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ccfc60, &args![this, cinfo]);
        let built = calls_to(&e, HK_LIST_CONSTRUCT);
        assert_eq!(&built[0][1..], &[0xa11a, 5]);
        let block = built[0][0];
        assert_eq!(e.mem.u16(block + 4), 0x50);
        assert_eq!(calls_to(&e, 0x0900_0103), vec![vec![this, block]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![block]]);
        assert_eq!(calls_to(&e, 0x0900_0104), vec![vec![this, cinfo]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn link_references_or_releases_the_children_then_the_base_link() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let (this, _, children) = shape(&mut e, 1);
        quiet(&mut e, &[SHAPE_LINK, INTERLOCKED_INCREMENT]);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ccfe20, &args![this, 0x101u32]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![children[0] + 4]]
        );
        e.call(0x00ccfe20, &args![this, 0x100u32]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![vec![children[0] + 4]]
        );
        assert_eq!(
            calls_to(&e, SHAPE_LINK),
            vec![vec![this, 0x101], vec![this, 0x100]]
        );
    }

    #[test]
    fn copy_cinfo_copies_the_radius_flag_and_children() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let (this, list, _) = shape(&mut e, 2);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO]);
        e.register(HK_LIST_FLAG, |_, _| Ret {
            eax: 0x1_07,
            ..Ret::default()
        });
        // A signalling NaN radius comes out quiet.
        e.mem.set_u32(list + 0x18, 0x7f80_0001);
        let child_array = e.mem.u32(list + 0x44);
        e.register(ARRAY_RESERVE, |e, a| {
            // Give the array room: data pointer and capacity.
            let data = e.mem.alloc(4 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let cinfo = e.mem.alloc(0x40);
        e.mem.set_u32(cinfo + 0x18, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ccfe50, &args![this, cinfo]);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u8(cinfo + 8), 7);
        assert_eq!(e.mem.u32(cinfo + 0x14), 2);
        let data = e.mem.u32(cinfo + 0x10);
        assert_eq!(e.mem.u32(data), e.mem.u32(child_array));
        assert_eq!(e.mem.u32(data + 4), e.mem.u32(child_array + 4));
        // Capacity 1 < 2: reserve the larger of 2 and the count (here equal) with element size 4.
        let reserve = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(&reserve[0][2..], &[2, 4]);
        assert_eq!(scope(&mut e), 0x77);
        // Without a list shape: zero radius, zero flag, no children.
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u8(cinfo + 8, 9);
        e.call(0x00ccfe50, &args![this, cinfo]);
        assert_eq!((e.mem.u32(cinfo + 0xc), e.mem.u8(cinfo + 8)), (0, 0));
        assert_eq!(e.mem.u32(cinfo + 0x14), 0);
    }

    #[test]
    fn create_cinfo_builds_the_missing_cinfo_once() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let (this, list, _) = shape(&mut e, 0);
        quiet(&mut e, &[CONVEX_SHAPE_COPY_CINFO]);
        e.register(HK_LIST_FLAG, |_, _| Ret::default());
        e.register(POOL_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.map(0x010c_7000, 0x1000);
        e.mem.set_u32(0x010c_72bc, 0x3f80_0000);
        e.mem.set_u32(list + 0x18, 0x4000_0000);
        let created = e.mem.alloc(8);
        e.mem.set_u8(created, 9);
        e.call_log = Some(vec![]);
        let cinfo = e.call(0x00ccffd0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(calls_to(&e, POOL_ALLOCATE), vec![vec![POOL, 0x1c]]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert_eq!(e.mem.u32(cinfo + 4), 0x3f80_0000);
        assert_eq!(e.mem.u32(cinfo + 0x18), 0x8000_0000);
        // CopyCinfo ran: the radius is the list shape's.
        assert_eq!(e.mem.u32(cinfo + 0xc), 0x4000_0000);
        // The second call finds it.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ccffd0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
        assert!(calls_to(&e, POOL_ALLOCATE).is_empty());
        assert_eq!(scope(&mut e), 0x77);
        // A failed pool allocation stores null and answers null.
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(this + 8, 0);
        e.register(POOL_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ccffd0, &args![this, created]).u32(), 0);
    }

    /// A stream whose reader (`+0x24C`, function at `+8`) and writer (`+0x250`, function at
    /// `+0xC`) are doubles: reads fill the destination from the reader's word at `+0x10`;
    /// writes append to the writer's log at `+0x100`. Slot `0x30` of the stream records.
    fn stream(e: &mut Engine) -> (u32, u32, u32) {
        let stream = object(e, &[(0x30, 0x0900_0150)]);
        quiet(e, &[0x0900_0150]);
        let reader = e.mem.alloc(0x20);
        let writer = e.mem.alloc(0x400);
        e.mem.set_u32(reader + 8, 0x0900_0151);
        e.mem.set_u32(writer + 0xc, 0x0900_0152);
        e.mem.set_u32(stream + 0x24c, reader);
        e.mem.set_u32(stream + 0x250, writer);
        e.register(0x0900_0151, |e, a| {
            let value = e.mem.u32(a[0] + 0x10);
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, (value >> (8 * i)) as u8);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        e.register(0x0900_0152, |e, a| {
            let at = e.mem.u32(a[0] + 0x14);
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + 0x100 + at + i, byte);
            }
            e.mem.set_u32(a[0] + 0x14, at + a[2]);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        (stream, reader, writer)
    }

    #[test]
    fn load_binary_reads_flag_and_float_and_swaps_on_the_other_endianness() {
        let mut e = Engine::new();
        let (stream, reader, _) = stream(&mut e);
        let (this, _, _) = shape(&mut e, 0);
        let cinfo = e.mem.alloc(0x40);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x40, cinfo);
        e.register(0x0900_0101, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        quiet(
            &mut e,
            &[
                STREAM_READ_LINK_IDS,
                CONVEX_SHAPE_LOAD_BINARY,
                SWAP_HK_ARRAY,
            ],
        );
        e.mem.set_u32(reader + 0x10, 0x0102_0304);
        e.mem.set_u8(stream + 0x1e5, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cd00a0, &args![this, stream]);
        assert_eq!(e.mem.u8(cinfo + 8), 4);
        assert_eq!(e.mem.u32(cinfo + 0xc), 0x0102_0304);
        assert!(calls_to(&e, SWAP_HK_ARRAY).is_empty());
        assert_eq!(e.mem.u32(reader + 4), 5);
        e.mem.set_u8(stream + 0x1e5, 0);
        e.call_log = Some(vec![]);
        e.call(0x00cd00a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SWAP_HK_ARRAY),
            vec![vec![cinfo, 3, cinfo + 0x10], vec![cinfo, 3, cinfo + 0x10]]
        );
        let first = order(&e).iter().position(|a| *a == SWAP_HK_ARRAY).unwrap();
        let read = order(&e).iter().position(|a| *a == 0x0900_0151).unwrap();
        assert!(first < read);
        assert_eq!(
            order(&e)[..3],
            [0x00cd00a0, STREAM_READ_LINK_IDS, CONVEX_SHAPE_LOAD_BINARY]
        );
    }

    #[test]
    fn link_object_rebuilds_the_cinfo_array_from_the_stream() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let (this, _, _) = shape(&mut e, 0);
        let cinfo = e.mem.alloc(0x40);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x40, cinfo);
        // An old array of capacity 4 (owned): it is freed with 16 bytes.
        let old = e.mem.alloc(16);
        e.mem.set_u32(cinfo + 0x10, old);
        e.mem.set_u32(cinfo + 0x14, 2);
        e.mem.set_u32(cinfo + 0x18, 4);
        quiet(&mut e, &[SHAPE_LINK_OBJECT]);
        e.register(STREAM_GET_LINK_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(4 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        quiet(&mut e, &[ARRAY_GROW_ONE]);
        let linked = e.mem.alloc(0x20);
        e.mem.set_u32(linked + 8, 0xbee);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, {
            // Answers: the object, nothing, the object (popped from the end).
            let mut queue = vec![linked, 0, linked];
            move |_, _| Ret {
                eax: queue.pop().unwrap(),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.call(0x00cd0170, &args![this, 0x5000u32]);
        assert_eq!(calls_to(&e, 0x0900_0201)[0][1..], [old, 16]);
        let data = e.mem.u32(cinfo + 0x10);
        assert_eq!(e.mem.u32(cinfo + 0x14), 2);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0xbee, 0xbee));
        assert_eq!(calls_to(&e, SHAPE_LINK_OBJECT), vec![vec![this, 0x5000]]);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn save_binary_writes_count_links_and_the_cinfo_fields_then_frees_the_array() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let (stream, _, writer) = stream(&mut e);
        let (this, _, _) = shape(&mut e, 2);
        let cinfo = e.mem.alloc(0x40);
        e.register(0x0900_0100, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x40),
            ..Ret::default()
        });
        e.mem.set_u32(this + 0x40, cinfo);
        let data = e.mem.alloc(16);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0xf1);
        e.mem.set_u32(data, first);
        e.mem.set_u32(cinfo + 0x10, data);
        e.mem.set_u32(cinfo + 0x14, 2);
        e.mem.set_u32(cinfo + 0x18, 2);
        e.mem.set_u8(cinfo + 8, 0x5a);
        e.mem.set_u32(cinfo + 0xc, 0x0a0b_0c0d);
        quiet(&mut e, &[CONVEX_SHAPE_SAVE_BINARY, SWAP_HK_ARRAY]);
        e.register(0x0900_0101, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.mem.set_u8(stream + 0x1e4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00cd02d0, &args![this, stream]);
        // Writer log: the count, then the byte, then the dword.
        assert_eq!(e.mem.u32(writer + 0x100), 2);
        assert_eq!(e.mem.u8(writer + 0x104), 0x5a);
        assert_eq!(e.mem.u32(writer + 0x105), 0x0a0b_0c0d);
        // Links: the first child's wrapper holds 0xf1, the second is null (0).
        assert_eq!(
            calls_to(&e, 0x0900_0150),
            vec![vec![stream, 0xf1], vec![stream, 0]]
        );
        // The cinfo's array was detached, and the saved storage freed (2 * 4 bytes).
        assert_eq!(
            (
                e.mem.u32(cinfo + 0x10),
                e.mem.u32(cinfo + 0x14),
                e.mem.u32(cinfo + 0x18)
            ),
            (0, 0, 0x8000_0000)
        );
        assert_eq!(calls_to(&e, 0x0900_0201)[0][1..], [data, 8]);
        assert!(calls_to(&e, SWAP_HK_ARRAY).is_empty());
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        // The other endianness swaps the (empty) array first.
        e.mem.set_u8(stream + 0x1e4, 0);
        e.mem.set_u32(cinfo + 0x10, data);
        e.mem.set_u32(cinfo + 0x18, 0x8000_0002);
        e.call_log = Some(vec![]);
        e.call(0x00cd02d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, SWAP_HK_ARRAY),
            vec![vec![cinfo, 9, cinfo + 0x10]]
        );
        // A not-owned array is not freed.
        assert_eq!(calls_to(&e, 0x0900_0201).len(), 0);
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn get_viewer_strings_adds_two_lines_and_formats_each_index() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        let (this, _, _) = shape(&mut e, 3);
        quiet(
            &mut e,
            &[CONVEX_SHAPE_GET_VIEWER_STRINGS, FORMAT_TEXT, ARRAY_GROW_ONE],
        );
        e.map(CLASS_TEXT, 8);
        e.set_global(CLASS_TEXT, 0x1111u32);
        e.register(VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VIEWER_STRING_INT, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        quiet(&mut e, &[0x0096_ad30]);
        e.register(0x0096_ae90, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x100 + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let list = e.mem.alloc(0x200);
        e.mem.set_u16(list + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x00cd0480, &args![this, list]);
        assert_eq!(e.mem.u32(list + 0x100), 0x1112);
        assert_eq!(e.mem.u32(list + 0x104), 0x010c_41b0 + 3);
        assert_eq!(e.mem.u16(list + 0xa), 2);
        let formats = calls_to(&e, FORMAT_TEXT);
        assert_eq!(
            formats
                .iter()
                .map(|c| (c[1], c[2], c[3]))
                .collect::<Vec<_>>(),
            vec![
                (0x20, 0x010c_6040, 0),
                (0x20, 0x010c_6040, 1),
                (0x20, 0x010c_6040, 2)
            ]
        );
        assert_eq!(scope(&mut e), 0x77);
    }

    #[test]
    fn kill_cinfo_frees_the_cinfo_only_when_asked() {
        let mut e = Engine::new();
        mark_scope(&mut e);
        install_allocator(&mut e);
        let this = e.mem.alloc(0x20);
        let cinfo = e.mem.alloc(0x20);
        let data = e.mem.alloc(16);
        e.mem.set_u32(cinfo + 0x10, data);
        e.mem.set_u32(cinfo + 0x14, 2);
        e.mem.set_u32(cinfo + 0x18, 4);
        e.mem.set_u32(this + 0xc, cinfo);
        quiet(&mut e, &[POOL_RELEASE]);
        e.call_log = Some(vec![]);
        e.call(0x00cd05e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        assert!(calls_to(&e, POOL_RELEASE).is_empty());
        e.call(0x00cd05e0, &args![this, 0x0100u32]);
        assert_eq!(e.mem.u32(this + 0xc), cinfo);
        e.call(0x00cd05e0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls_to(&e, POOL_RELEASE), vec![vec![POOL, cinfo]]);
        assert_eq!(calls_to(&e, 0x0900_0201)[0][1..], [data, 16]);
        assert_eq!(
            (
                e.mem.u32(cinfo + 0x10),
                e.mem.u32(cinfo + 0x14),
                e.mem.u32(cinfo + 0x18)
            ),
            (0, 0, 0x8000_0000)
        );
        // No cinfo: still clears +0xC.
        e.call(0x00cd05e0, &args![this, 1u32]);
        assert_eq!(scope(&mut e), 0x77);
    }
}
